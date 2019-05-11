use std::ops::{Add, Div, Mul, Sub};
use std::{fmt, ptr};

use num_traits::{Signed, Zero};

use super::{intersect, Vertex};

pub struct Polygon<T> {
    first: *mut Vertex<T>,
}

impl<'slice, T> From<&'slice [[T; 2]]> for Polygon<T>
where
    T: Clone + Zero + Signed + PartialEq + PartialOrd,
    for<'a, 'b> &'a T: Div<&'b T, Output = T>
        + Sub<&'b T, Output = T>
        + Add<&'b T, Output = T>
        + Mul<&'b T, Output = T>,
{
    #[inline]
    fn from(slice: &'slice [[T; 2]]) -> Self {
        let mut polygon = Self::new();

        for v in slice {
            let vertex = Vertex::new(v.clone(), T::zero(), false, true, false);
            polygon.add(vertex);
        }

        polygon
    }
}

impl<T> fmt::Debug for Polygon<T>
where
    T: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Polygon {:?}",
            self.iter()
                .map(|v| unsafe { &v.as_ref().unwrap().point })
                .collect::<Vec<&[T; 2]>>()
        )
    }
}

impl<T> Polygon<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            first: ptr::null_mut(),
        }
    }
}

impl<T> Polygon<T> {
    #[inline]
    pub fn iter(&self) -> PolygonIter<T> {
        PolygonIter::new(self)
    }
}

impl<T> Polygon<T>
where
    T: Clone + Signed + PartialEq + PartialOrd,
    for<'a, 'b> &'a T: Div<&'b T, Output = T>
        + Sub<&'b T, Output = T>
        + Add<&'b T, Output = T>
        + Mul<&'b T, Output = T>,
{
    #[inline]
    pub fn add(&mut self, vertex: *mut Vertex<T>) {
        if self.first.is_null() {
            self.first = vertex;
        } else {
            unsafe {
                let next = self.first;
                let prev = next.as_ref().unwrap().prev;

                next.as_mut().unwrap().prev = vertex;
                vertex.as_mut().unwrap().next = next;
                vertex.as_mut().unwrap().prev = prev;
                prev.as_mut().unwrap().next = vertex;
            }
        }
    }

    /// Insert and sort a vertex between a specified pair of vertices
    #[inline]
    pub fn insert(&mut self, vertex: *mut Vertex<T>, start: *mut Vertex<T>, end: *mut Vertex<T>) {
        unsafe {
            let mut current = start;

            while current != end && current.as_ref().unwrap().alpha < vertex.as_ref().unwrap().alpha
            {
                current = current.as_ref().unwrap().next;
            }

            vertex.as_mut().unwrap().next = current;
            let prev = current.as_ref().unwrap().prev;
            vertex.as_mut().unwrap().prev = prev;
            prev.as_mut().unwrap().next = vertex;
            current.as_mut().unwrap().prev = vertex;
        }
    }

    /// Return the next non intersecting vertex after the one specified
    #[inline]
    pub fn next(&self, vertex: *mut Vertex<T>) -> *mut Vertex<T> {
        let mut current = vertex;

        unsafe {
            while current.as_ref().unwrap().inter {
                current = current.as_ref().unwrap().next;
            }
        }

        current
    }

    /// Return the first unchecked intersection point in the polygon
    #[inline]
    pub fn first_intersect(&self) -> *mut Vertex<T> {
        let mut ve = ptr::null_mut();

        for v in self.iter() {
            ve = v;
            unsafe {
                if v.as_ref().unwrap().inter && !v.as_ref().unwrap().checked {
                    break;
                }
            }
        }

        ve
    }

    /// Return the polygon points as a list of points, in counter clock wise order clear consecutive equals points
    #[inline]
    pub fn points(&self) -> Vec<[T; 2]> {
        let mut polygon = Vec::new();

        for vertex in self.iter() {
            let point = unsafe { &vertex.as_ref().unwrap().point };
            let prev_point = unsafe { &vertex.as_ref().unwrap().prev.as_ref().unwrap().point };

            if prev_point != point {
                polygon.push(point.clone());
            }
        }

        polygon
    }
    #[inline]
    pub fn points_rev(&self) -> Vec<[T; 2]> {
        let mut polygon = Vec::new();

        for vertex in self.iter().rev() {
            let point = unsafe { &vertex.as_ref().unwrap().point };
            let prev_point = unsafe { &vertex.as_ref().unwrap().prev.as_ref().unwrap().point };

            if prev_point != point {
                polygon.push(point.clone());
            }
        }

        polygon
    }

    /// Check if an unchecked intersection remain in the polygon
    #[inline]
    pub fn unprocessed(&self) -> bool {
        for vertex in self.iter() {
            if unsafe { vertex.as_ref().unwrap().inter && !vertex.as_ref().unwrap().checked } {
                return true;
            }
        }

        false
    }

    #[inline]
    pub fn phase_one(&mut self, polygon: &mut Polygon<T>) {
        for s in self.iter() {
            unsafe {
                if !s.as_ref().unwrap().inter {
                    for c in polygon.iter() {
                        if !c.as_ref().unwrap().inter {
                            let s_point = &s.as_ref().unwrap().point;
                            let s_next_point =
                                &self.next(s.as_ref().unwrap().next).as_ref().unwrap().point;
                            let c_point = &c.as_ref().unwrap().point;
                            let c_next_point = &polygon
                                .next(c.as_ref().unwrap().next)
                                .as_ref()
                                .unwrap()
                                .point;

                            match intersect(s_point, s_next_point, c_point, c_next_point) {
                                Some((i, alpha_s, alpha_c)) => {
                                    let mut is =
                                        Vertex::new(i.clone(), alpha_s, true, false, false);
                                    let mut ic = Vertex::new(i, alpha_c, true, false, false);
                                    is.as_mut().unwrap().neighbour = ic;
                                    ic.as_mut().unwrap().neighbour = is;

                                    let s_next = self.next(s.as_ref().unwrap().next);
                                    self.insert(is, s, s_next);

                                    let c_next = polygon.next(c.as_ref().unwrap().next);
                                    polygon.insert(ic, c, c_next);
                                }
                                None => {}
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline]
    pub fn phase_two(&mut self, polygon: &mut Polygon<T>, mut s_entry: bool, mut c_entry: bool) {
        s_entry ^= unsafe { self.first.as_ref().unwrap().is_inside(polygon) };

        for s in self.iter() {
            if unsafe { s.as_ref().unwrap().inter } {
                unsafe {
                    s.as_mut().unwrap().entry = s_entry;
                }
                s_entry = !s_entry;
            }
        }

        c_entry ^= unsafe { polygon.first.as_ref().unwrap().is_inside(self) };

        for c in polygon.iter() {
            if unsafe { c.as_ref().unwrap().inter } {
                unsafe {
                    c.as_mut().unwrap().entry = c_entry;
                }
                c_entry = !c_entry;
            }
        }
    }

    #[inline]
    pub fn phase_three(&mut self, rev: bool) -> Vec<Vec<[T; 2]>> {
        let mut list = Vec::new();

        while self.unprocessed() {
            let mut current = self.first_intersect();
            let mut clipped = Polygon::new();

            clipped.add(Vertex::clone(current));

            loop {
                unsafe {
                    current.as_mut().unwrap().set_checked();
                }
                if unsafe { current.as_ref().unwrap().entry } {
                    loop {
                        current = unsafe { current.as_ref().unwrap().next };

                        clipped.add(Vertex::clone(current));

                        if unsafe { current.as_ref().unwrap().inter } {
                            break;
                        }
                    }
                } else {
                    loop {
                        current = unsafe { current.as_ref().unwrap().prev };

                        clipped.add(Vertex::clone(current));

                        if unsafe { current.as_ref().unwrap().inter } {
                            break;
                        }
                    }
                }

                current = unsafe { current.as_ref().unwrap().neighbour };
                if unsafe { current.as_ref().unwrap().checked } {
                    break;
                }
            }

            if rev {
                list.push(clipped.points_rev());
            } else {
                list.push(clipped.points());
            }
        }

        if list.is_empty() {
            list.push(self.points());
        }

        list
    }

    #[inline]
    pub fn clip(
        &mut self,
        polygon: &mut Polygon<T>,
        s_entry: bool,
        c_entry: bool,
        rev: bool,
    ) -> Vec<Vec<[T; 2]>> {
        self.phase_one(polygon);
        self.phase_two(polygon, s_entry, c_entry);
        self.phase_three(rev)
    }

    #[inline(always)]
    pub fn union(&mut self, polygon: &mut Polygon<T>) -> Vec<Vec<[T; 2]>> {
        self.clip(polygon, false, false, true)
    }

    #[inline(always)]
    pub fn intersection(&mut self, polygon: &mut Polygon<T>) -> Vec<Vec<[T; 2]>> {
        self.clip(polygon, true, true, false)
    }

    #[inline(always)]
    pub fn difference(&mut self, polygon: &mut Polygon<T>) -> Vec<Vec<[T; 2]>> {
        self.clip(polygon, false, true, true)
    }
}

impl<T> Drop for Polygon<T> {
    #[inline]
    fn drop(&mut self) {
        let mut current = self.first;

        if !current.is_null() {
            loop {
                unsafe {
                    let next = current.as_ref().unwrap().next;

                    Box::from_raw(current);
                    current = next;

                    if current == self.first {
                        break;
                    }
                }
            }
        }
    }
}

pub struct PolygonIter<T> {
    first: *mut Vertex<T>,
    current: *mut Vertex<T>,
    is_first: bool,
}

impl<T> PolygonIter<T> {
    #[inline(always)]
    fn new(polygon: &Polygon<T>) -> Self {
        Self {
            first: polygon.first,
            current: polygon.first,
            is_first: true,
        }
    }
}

impl<T> Iterator for PolygonIter<T> {
    type Item = *mut Vertex<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            if self.current.is_null() {
                None
            } else {
                let vertex = self.current;
                unsafe {
                    self.current = self.current.as_ref().unwrap().next;
                }
                self.is_first = false;
                Some(vertex)
            }
        } else {
            if self.current == self.first {
                None
            } else {
                let vertex = self.current;
                unsafe {
                    self.current = self.current.as_ref().unwrap().next;
                }
                Some(vertex)
            }
        }
    }
}

impl<T> DoubleEndedIterator for PolygonIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_first {
            if self.current.is_null() {
                None
            } else {
                let vertex = self.current;
                unsafe {
                    self.current = self.current.as_ref().unwrap().prev;
                }
                self.is_first = false;
                Some(vertex)
            }
        } else {
            if self.current == self.first {
                None
            } else {
                let vertex = self.current;
                unsafe {
                    self.current = self.current.as_ref().unwrap().prev;
                }
                Some(vertex)
            }
        }
    }
}
