#![feature(alloc)]
#![no_std]

extern crate alloc;

extern crate number_traits;
extern crate vec2;

mod clipping;
mod area;
mod closest_edge;
mod contains_point;
mod distance;
mod intersection;
mod is_convex;
mod is_simple;
mod line_intersection;
mod point_in_rect;
mod point_in_triangle;
mod point_to_line_distance;
mod triangulate;

pub use self::clipping::{difference, intersection, union};
pub use self::area::area;
pub use self::closest_edge::{closest_edge, closest_edge_offset_angle};
pub use self::contains_point::contains_point;
pub use self::distance::distance;
pub use self::intersection::Intersection;
pub use self::is_convex::{is_convex, is_triangle_convex};
pub use self::is_simple::is_simple;
pub use self::line_intersection::line_intersection;
pub use self::point_in_rect::point_in_rect;
pub use self::point_in_triangle::point_in_triangle;
pub use self::point_to_line_distance::{point_to_line_distance, point_to_line_intersection};
pub use self::triangulate::triangulate;
