/// https://gitlab.com/nyradr/clipping/blob/master/src/gh.rs

mod intersect;
mod polygon;
mod vertex;

pub use self::intersect::intersect;
pub use self::polygon::Polygon;
pub use self::vertex::Vertex;
