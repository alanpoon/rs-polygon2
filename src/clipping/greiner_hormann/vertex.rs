use std::ops::{Add, Div, Mul, Sub};
use std::ptr;

use num_traits::Signed;

use super::Polygon;

pub struct Vertex<T> {
    pub point: [T; 2],
    pub next: *mut Vertex<T>,
    pub prev: *mut Vertex<T>,
    pub neighbour: *mut Vertex<T>,
    pub entry: bool,
    pub alpha: T,
    pub inter: bool,
    pub checked: bool,
}

impl<T> Vertex<T> {
    #[inline]
    pub(super) fn new(
        point: [T; 2],
        alpha: T,
        inter: bool,
        entry: bool,
        checked: bool,
    ) -> *mut Self {
        let ptr = Box::into_raw(Box::new(Self {
            point: point,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            neighbour: ptr::null_mut(),
            entry: entry,
            alpha: alpha,
            inter: inter,
            checked: checked,
        }));

        unsafe {
            ptr.as_mut().unwrap().next = ptr;
            ptr.as_mut().unwrap().prev = ptr;
        }

        ptr
    }
}

impl<T> Vertex<T>
where
    T: Clone + Signed + PartialEq + PartialOrd,
    for<'a, 'b> &'a T: Div<&'b T, Output = T>
        + Add<&'b T, Output = T>
        + Sub<&'b T, Output = T>
        + Mul<&'b T, Output = T>,
{
    #[inline]
    pub fn clone(vertex: *mut Vertex<T>) -> *mut Self {
        unsafe {
            Self::new(
                vertex.as_ref().unwrap().point.clone(),
                vertex.as_ref().unwrap().alpha.clone(),
                vertex.as_ref().unwrap().inter,
                vertex.as_ref().unwrap().entry,
                vertex.as_ref().unwrap().checked,
            )
        }
    }

    /// https://github.com/w8r/GreinerHormann/blob/master/src/vertex.js#L110
    #[inline]
    pub fn is_inside(&self, polygon: &Polygon<T>) -> bool {
        let mut odd_nodes = false;
        let x = &self.point[0];
        let y = &self.point[1];

        for vertex in polygon.iter() {
            let vertex_point = unsafe { &vertex.as_ref().unwrap().point };
            let next_point = unsafe {
                &polygon
                    .next(vertex.as_ref().unwrap().next)
                    .as_ref()
                    .unwrap()
                    .point
            };

            if (&vertex_point[1] < y && &next_point[1] >= y
                || &next_point[1] < y && &vertex_point[1] >= y)
                && (&vertex_point[0] <= x || &next_point[0] <= x)
            {
                let a = &vertex_point[0]
                    + &(&(&(y - &vertex_point[1]) / &(&next_point[1] - &vertex_point[1]))
                        * &(&next_point[0] - &vertex_point[0]));
                odd_nodes ^= &a < x;
            }
        }

        odd_nodes
    }

    #[inline]
    pub fn set_checked(&mut self) {
        self.checked = true;
        if !self.neighbour.is_null() {
            unsafe {
                if !self.neighbour.as_ref().unwrap().checked {
                    self.neighbour.as_mut().unwrap().set_checked();
                }
            }
        }
    }
}
