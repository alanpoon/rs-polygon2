#![feature(alloc)]
#![no_std]


extern crate alloc;

extern crate number_traits;


mod area;
mod closest_edge;
mod contains_point;
mod in_rect;
mod intersection;
mod is_convex;
mod is_simple;
mod line_intersection;
mod point_in_triangle;
mod point_to_line_distance;
mod triangulate;


pub use self::area::area;
pub use self::closest_edge::closest_edge;
pub use self::contains_point::contains_point;
pub use self::in_rect::in_rect;
pub use self::intersection::Intersection;
pub use self::is_convex::{is_convex, is_triangle_convex};
pub use self::is_simple::is_simple;
pub use self::line_intersection::line_intersection;
pub use self::point_in_triangle::point_in_triangle;
pub use self::point_to_line_distance::{point_to_line_distance, point_to_line_intersection};
pub use self::triangulate::triangulate;
