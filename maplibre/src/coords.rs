//! File which exposes all kinds of coordinates used throughout maplibre-rs

use std::fmt;
use std::fmt::Formatter;

use cgmath::num_traits::Pow;
use cgmath::{AbsDiffEq, Matrix4, Point3, Vector3};

use crate::style::source::TileAddressingScheme;

use crate::util::math::{div_floor, Aabb2};
use crate::util::SignificantlyDifferent;

pub const EXTENT_UINT: u32 = 4096;
pub const EXTENT_SINT: i32 = EXTENT_UINT as i32;
pub const EXTENT: f64 = EXTENT_UINT as f64;
pub const TILE_SIZE: f64 = 512.0;
pub const MAX_ZOOM: usize = 32;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub struct Quadkey([u8; MAX_ZOOM]);

impl Quadkey {
    pub fn new(quad_encoded: &[u8]) -> Self {
        let mut key = [0u8; 32];
        key[0] = quad_encoded.len() as u8;
        for (i, part) in quad_encoded.iter().enumerate() {
            key[i + 1] = *part;
        }
        Self(key)
    }
}

impl fmt::Debug for Quadkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let len = self.0[0] as usize;
        for part in &self.0[0..len] {
            write!(f, "{:?}", part)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Zoom(f64);

impl Zoom {
    pub fn new(zoom: f64) -> Self {
        Zoom(zoom)
    }
}

impl Default for Zoom {
    fn default() -> Self {
        Zoom(0.0)
    }
}

impl fmt::Display for Zoom {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (self.0 * 100.0).round() / 100.0)
    }
}

impl std::ops::Add for Zoom {
    type Output = Zoom;

    fn add(self, rhs: Self) -> Self::Output {
        Zoom(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Zoom {
    type Output = Zoom;

    fn sub(self, rhs: Self) -> Self::Output {
        Zoom(self.0 - rhs.0)
    }
}

impl Zoom {
    pub fn scale_to_tile(&self, coords: &WorldTileCoords) -> f64 {
        2.0_f64.powf(coords.z as f64 - self.0)
    }

    pub fn scale_to_zoom_level(&self, z: u8) -> f64 {
        2.0_f64.powf(z as f64 - self.0)
    }

    pub fn scale_delta(&self, zoom: &Zoom) -> f64 {
        2.0_f64.powf(zoom.0 - self.0)
    }

    pub fn level(&self) -> u8 {
        self.0.floor() as u8
    }
}

impl SignificantlyDifferent for Zoom {
    type Epsilon = f64;

    fn ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}

/// Within each tile there is a separate coordinate system. Usually this coordinate system is
/// within [`crate::coords::EXTENT`]. Therefore, `x` and `y` must be within the bounds of
/// [`crate::coords::EXTENT`].
///
/// # Coordinate System Origin
///
/// The origin is in the upper-left corner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InnerCoords {
    pub x: f64,
    pub y: f64,
}

/// Every tile has tile coordinates. These tile coordinates are also called
/// [Slippy map tilenames](https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames).
///
/// # Coordinate System Origin
///
/// For Web Mercator the origin of the coordinate system is in the upper-left corner.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct TileCoords {
    pub x: u32,
    pub y: u32,
    pub z: u8,
}

impl TileCoords {
    /// Transforms the tile coordinates as defined by the tile grid addressing scheme into a
    /// representation which is used in the 3d-world.
    /// This is not possible if the coordinates of this [`TileCoords`] exceed their bounds.
    ///
    /// # Example
    /// The [`TileCoords`] `T(x=5,y=5,z=0)` exceeds its bounds because there is no tile
    /// `x=5,y=5` at zoom level `z=0`.
    pub fn into_world_tile(self, scheme: TileAddressingScheme) -> Option<WorldTileCoords> {
        let bounds = 2i32.pow(self.z as u32);
        let x = self.x as i32;
        let y = self.y as i32;

        if x >= bounds || y >= bounds {
            return None;
        }

        Some(match scheme {
            TileAddressingScheme::XYZ => WorldTileCoords { x, y, z: self.z },
            TileAddressingScheme::TMS => WorldTileCoords {
                x,
                y: bounds - 1 - y,
                z: self.z,
            },
        })
    }
}

impl From<(u32, u32, u8)> for TileCoords {
    fn from(tuple: (u32, u32, u8)) -> Self {
        TileCoords {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

/// Every tile has tile coordinates. Every tile coordinate can be mapped to a coordinate within
/// the world. This provides the freedom to map from [TMS](https://wiki.openstreetmap.org/wiki/TMS)
/// to [Slippy_map_tilenames](https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames).
///
/// # Coordinate System Origin
///
/// The origin of the coordinate system is in the upper-left corner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldTileCoords {
    pub x: i32,
    pub y: i32,
    pub z: u8,
}

impl WorldTileCoords {
    /// Returns the tile coords according to an addressing scheme. This is not possible if the
    /// coordinates of this [`WorldTileCoords`] exceed their bounds.
    ///
    /// # Example
    ///
    /// The [`WorldTileCoords`] `WT(x=5,y=5,z=0)` exceeds its bounds because there is no tile
    /// `x=5,y=5` at zoom level `z=0`.
    pub fn into_tile(self, scheme: TileAddressingScheme) -> Option<TileCoords> {
        let bounds = 2u32.pow(self.z as u32);
        let x = self.x as u32;
        let y = self.y as u32;

        if x >= bounds || y >= bounds {
            return None;
        }

        Some(match scheme {
            TileAddressingScheme::XYZ => TileCoords { x, y, z: self.z },
            TileAddressingScheme::TMS => TileCoords {
                x,
                y: bounds - 1 - y,
                z: self.z,
            },
        })
    }

    #[tracing::instrument(skip_all)]
    pub fn transform_for_zoom(&self, zoom: Zoom) -> Matrix4<f64> {
        /*
           For tile.z = zoom:
               => scale = 512
           If tile.z < zoom:
               => scale > 512
           If tile.z > zoom:
               => scale < 512
        */
        let tile_scale = TILE_SIZE * Zoom::new(self.z as f64).scale_delta(&zoom);

        let translate = Matrix4::from_translation(Vector3::new(
            self.x as f64 * tile_scale,
            self.y as f64 * tile_scale,
            0.0,
        ));

        // Divide by EXTENT to normalize tile
        // Scale tiles where zoom level = self.z to 512x512
        let normalize_and_scale =
            Matrix4::from_nonuniform_scale(tile_scale / EXTENT, tile_scale / EXTENT, 1.0);
        translate * normalize_and_scale
    }

    pub fn into_aligned(self) -> AlignedWorldTileCoords {
        AlignedWorldTileCoords(WorldTileCoords {
            x: div_floor(self.x, 2) * 2,
            y: div_floor(self.y, 2) * 2,
            z: self.z,
        })
    }

    /// Adopted from [tilebelt](https://github.com/mapbox/tilebelt)
    pub fn build_quad_key(&self) -> Option<Quadkey> {
        let bounds = 2u32.pow(self.z as u32);
        let x = self.x as u32;
        let y = self.y as u32;

        if x >= bounds || y >= bounds {
            return None;
        }

        let mut key = [0u8; 32];

        key[0] = self.z;

        for z in 1..self.z + 1 {
            let mut b = 0;
            let mask: i32 = 1 << (z - 1);
            if (self.x & mask) != 0 {
                b += 1u8;
            }
            if (self.y & mask) != 0 {
                b += 2u8;
            }
            key[z as usize] = b;
        }
        Some(Quadkey(key))
    }

    /// Adopted from [tilebelt](https://github.com/mapbox/tilebelt)
    pub fn get_children(&self) -> [WorldTileCoords; 4] {
        [
            WorldTileCoords {
                x: self.x * 2,
                y: self.y * 2,
                z: self.z + 1,
            },
            WorldTileCoords {
                x: self.x * 2 + 1,
                y: self.y * 2,
                z: self.z + 1,
            },
            WorldTileCoords {
                x: self.x * 2 + 1,
                y: self.y * 2 + 1,
                z: self.z + 1,
            },
            WorldTileCoords {
                x: self.x * 2,
                y: self.y * 2 + 1,
                z: self.z + 1,
            },
        ]
    }

    /// Get the tile which is one zoom level lower and contains this one
    pub fn get_parent(&self) -> Option<WorldTileCoords> {
        if self.z == 0 {
            return None;
        }

        Some(WorldTileCoords {
            x: self.x >> 1,
            y: self.y >> 1,
            z: self.z - 1,
        })
    }
}

impl From<(i32, i32, u8)> for WorldTileCoords {
    fn from(tuple: (i32, i32, u8)) -> Self {
        WorldTileCoords {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

/// An aligned world tile coordinate aligns a world coordinate at a 4x4 tile raster within the
/// world. The aligned coordinates is defined by the coordinates of the upper left tile in the 4x4
/// tile raster divided by 2 and rounding to the ceiling.
///
///
/// # Coordinate System Origin
///
/// The origin of the coordinate system is in the upper-left corner.
pub struct AlignedWorldTileCoords(pub WorldTileCoords);

impl AlignedWorldTileCoords {
    pub fn upper_left(self) -> WorldTileCoords {
        self.0
    }

    pub fn upper_right(&self) -> WorldTileCoords {
        WorldTileCoords {
            x: self.0.x + 1,
            y: self.0.y,
            z: self.0.z,
        }
    }

    pub fn lower_left(&self) -> WorldTileCoords {
        WorldTileCoords {
            x: self.0.x,
            y: self.0.y - 1,
            z: self.0.z,
        }
    }

    pub fn lower_right(&self) -> WorldTileCoords {
        WorldTileCoords {
            x: self.0.x + 1,
            y: self.0.y + 1,
            z: self.0.z,
        }
    }
}

/// Actual coordinates within the 3D world. The `z` value of the [`WorldCoors`] is not related to
/// the `z` value of the [`WorldTileCoors`]. In the 3D world all tiles are rendered at `z` values
/// which are determined only by the render engine and not by the zoom level.
///
/// # Coordinate System Origin
///
/// The origin of the coordinate system is in the upper-left corner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldCoords {
    pub x: f64,
    pub y: f64,
}

fn tiles_with_z(z: u8) -> f64 {
    2.0.pow(z)
}

impl WorldCoords {
    pub fn at_ground(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn into_world_tile(self, z: u8, zoom: Zoom) -> WorldTileCoords {
        let tile_scale = zoom.scale_to_zoom_level(z) / TILE_SIZE; // TODO: Deduplicate
        let x = self.x * tile_scale;
        let y = self.y * tile_scale;

        WorldTileCoords {
            x: x as i32,
            y: y as i32,
            z,
        }
    }
}

impl From<(f32, f32)> for WorldCoords {
    fn from(tuple: (f32, f32)) -> Self {
        WorldCoords {
            x: tuple.0 as f64,
            y: tuple.1 as f64,
        }
    }
}

impl From<(f64, f64)> for WorldCoords {
    fn from(tuple: (f64, f64)) -> Self {
        WorldCoords {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<Point3<f64>> for WorldCoords {
    fn from(point: Point3<f64>) -> Self {
        WorldCoords {
            x: point.x,
            y: point.y,
        }
    }
}

#[derive(Debug)]
pub struct ViewRegion {
    min_tile: WorldTileCoords,
    max_tile: WorldTileCoords,
    z: u8,
    padding: i32,
}

impl ViewRegion {
    pub fn new(view_region: Aabb2<f64>, padding: i32, zoom: Zoom, z: u8) -> Self {
        let min_world: WorldCoords = WorldCoords::at_ground(view_region.min.x, view_region.min.y);
        let min_world_tile: WorldTileCoords = min_world.into_world_tile(z, zoom);
        let max_world: WorldCoords = WorldCoords::at_ground(view_region.max.x, view_region.max.y);
        let max_world_tile: WorldTileCoords = max_world.into_world_tile(z, zoom);

        Self {
            min_tile: min_world_tile,
            max_tile: max_world_tile,
            z,
            padding,
        }
    }

    pub fn zoom_level(&self) -> u8 {
        self.z
    }

    pub fn is_in_view(&self, &world_coords: &WorldTileCoords) -> bool {
        world_coords.x <= self.max_tile.x + self.padding
            && world_coords.y <= self.max_tile.y + self.padding
            && world_coords.x >= self.min_tile.x - self.padding
            && world_coords.y >= self.min_tile.y - self.padding
            && world_coords.z == self.z
    }

    pub fn iter(&self) -> impl Iterator<Item = WorldTileCoords> + '_ {
        (self.min_tile.x - self.padding..self.max_tile.x + 1 + self.padding).flat_map(move |x| {
            (self.min_tile.y - self.padding..self.max_tile.y + 1 + self.padding).map(move |y| {
                let tile_coord: WorldTileCoords = (x, y, self.z as u8).into();
                tile_coord
            })
        })
    }
}

impl fmt::Display for TileCoords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "T(x={x},y={y},z={z})",
            x = self.x,
            y = self.y,
            z = self.z
        )
    }
}

impl fmt::Display for WorldTileCoords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WT(x={x},y={y},z={z})",
            x = self.x,
            y = self.y,
            z = self.z
        )
    }
}
impl fmt::Display for WorldCoords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "W(x={x},y={y})", x = self.x, y = self.y,)
    }
}

#[cfg(test)]
mod tests {
    use cgmath::{Point2, Vector4};

    use crate::style::source::TileAddressingScheme;

    use crate::coords::{
        Quadkey, TileCoords, ViewRegion, WorldCoords, WorldTileCoords, Zoom, EXTENT,
    };
    use crate::util::math::Aabb2;

    const TOP_LEFT: Vector4<f64> = Vector4::new(0.0, 0.0, 0.0, 1.0);
    const BOTTOM_RIGHT: Vector4<f64> = Vector4::new(EXTENT, EXTENT, 0.0, 1.0);

    fn to_from_world(tile: (i32, i32, u8), zoom: Zoom) {
        let tile = WorldTileCoords::from(tile);
        let p1 = tile.transform_for_zoom(zoom) * TOP_LEFT;
        let p2 = tile.transform_for_zoom(zoom) * BOTTOM_RIGHT;
        println!("{:?}\n{:?}", p1, p2);

        assert_eq!(
            WorldCoords::from((p1.x, p1.y)).into_world_tile(zoom.level(), zoom),
            tile
        );
    }

    #[test]
    fn world_coords_tests() {
        to_from_world((1, 0, 1), Zoom::new(1.0));
        to_from_world((67, 42, 7), Zoom::new(7.0));
        to_from_world((17421, 11360, 15), Zoom::new(15.0));
    }

    #[test]
    fn test_quad_key() {
        assert_eq!(
            TileCoords { x: 0, y: 0, z: 1 }
                .into_world_tile(TileAddressingScheme::TMS)
                .unwrap()
                .build_quad_key(),
            Some(Quadkey::new(&[2]))
        );
        assert_eq!(
            TileCoords { x: 0, y: 1, z: 1 }
                .into_world_tile(TileAddressingScheme::TMS)
                .unwrap()
                .build_quad_key(),
            Some(Quadkey::new(&[0]))
        );
        assert_eq!(
            TileCoords { x: 1, y: 1, z: 1 }
                .into_world_tile(TileAddressingScheme::TMS)
                .unwrap()
                .build_quad_key(),
            Some(Quadkey::new(&[1]))
        );
        assert_eq!(
            TileCoords { x: 1, y: 0, z: 1 }
                .into_world_tile(TileAddressingScheme::TMS)
                .unwrap()
                .build_quad_key(),
            Some(Quadkey::new(&[3]))
        );
    }

    #[test]
    fn test_view_region() {
        for tile_coords in ViewRegion::new(
            Aabb2::new(Point2::new(0.0, 0.0), Point2::new(2000.0, 2000.0)),
            1,
            Zoom::default(),
            0,
        )
        .iter()
        {
            println!("{}", tile_coords);
        }
    }
}
