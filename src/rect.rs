use crate::line::{Line, Point};
use core::cmp;

extern crate num_traits;

/// A rectangle constructed by two points. p1 describes the upper left corner and p2 the lower
/// right corner of the rectangle
#[derive(Clone, PartialEq, Debug)]
pub struct Rect<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd,
{
    p1: Point<T>,
    p2: Point<T>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Corner {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RectLine {
    Upper,
    Right,
    Lower,
    Left,
}

impl<T> Rect<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign + Ord,
{
    /// constructs a rectangle based on its height and width placing it to (0, 0)
    pub fn new(width: T, height: T) -> Rect<T> {
        Rect {
            p1: Point {
                x: T::zero(),
                y: T::zero(),
            },
            p2: Point {
                x: width,
                y: height,
            },
        }
    }

    pub fn new_by_points(p1: &Point<T>, p2: &Point<T>, p3: &Point<T>, p4: &Point<T>) -> Rect<T> {
        let mut list: Vec<&Point<T>> = Vec::with_capacity(3);
        list.extend_from_slice(&[p2, p3, p4]);
        let mut upper_left = p1.clone();
        for e in list {
            if e.x < upper_left.x && e.y > upper_left.y {
                upper_left = e.clone();
            }
        }
        let mut lower_right = p1.clone();
        for e in list {
            if e.x > lower_right.x && e.y < lower_right.y {
                lower_right = e.clone();
            }
        }
        Rect {
            p1: upper_left,
            p2: lower_right,
        }
    }

    /// Moves the rectangle by the x and y offset
    pub fn move_by(&mut self, x_offset: T, y_offset: T) {
        self.p1.x += x_offset;
        self.p1.y += y_offset;
        self.p2.x += x_offset;
        self.p2.y += y_offset;
    }

    /// Moves the rectangle to the point to using the upper left corner as a reference point
    pub fn move_to(&mut self, to: Point<T>) {
        let d_x = self.p2.x - self.p1.x;
        let d_y = self.p2.y - self.p1.y;

        self.p1.x = to.x;
        self.p1.y = to.y;
        self.p2.x = to.x + d_x;
        self.p2.y = to.y + d_y;
    }

    /// Return the points
    pub fn get_corner(&self, corner: Corner) -> Point<T> {
        match corner {
            Corner::UpperLeft => self.p1.clone(),
            Corner::UpperRight => Point {
                x: self.p2.x,
                y: self.p1.y,
            },
            Corner::LowerLeft => Point {
                x: self.p1.x,
                y: self.p2.y,
            },
            Corner::LowerRight => self.p2.clone(),
        }
    }

    pub fn get_line_iterator(&self) -> RectLineIterator<T> {
        RectLineIterator {
            rect: self,
            line_id: RectLine::Upper,
        }
    }

    pub fn get_overlapped_area(&self, other: &Rect<T>) -> T {
        cmp::max(
            0,
            cmp::min(other.p2.x, self.p2.x) - cmp::max(other.p1.x, self.p1.x),
        ) * cmp::max(
            0,
            cmp::min(other.p2.y, self.p2.y) - cmp::max(other.p1.y, self.p1.y),
        )
    }

    pub fn overlapped(&self, other: &Rect<T>) -> bool {
        self.get_overlapped_area(other) != T::zero()
    }
}

pub struct RectLineIterator<'a, T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign + Ord,
{
    rect: &'a Rect<T>,
    line_id: RectLine,
}

impl<'a, T> Iterator for RectLineIterator<'a, T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign + Ord,
{
    type Item = Line<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line_id == RectLine::Left {
            return None;
        }
        Some(match self.line_id {
            RectLine::Upper => {
                self.line_id = RectLine::Right;
                Line {
                    p1: self.rect.get_corner(Corner::UpperLeft),
                    p2: self.rect.get_corner(Corner::UpperRight),
                }
            }
            RectLine::Right => {
                self.line_id = RectLine::Lower;
                Line {
                    p1: self.rect.get_corner(Corner::UpperLeft),
                    p2: self.rect.get_corner(Corner::UpperRight),
                }
            }
            RectLine::Lower => {
                self.line_id = RectLine::Left;
                Line {
                    p1: self.rect.get_corner(Corner::UpperLeft),
                    p2: self.rect.get_corner(Corner::UpperRight),
                }
            }
            RectLine::Left => Line {
                p1: self.rect.get_corner(Corner::UpperLeft),
                p2: self.rect.get_corner(Corner::UpperRight),
            },
        })
    }
}

/// This trait describes the functions of a 2d object which creates a plane
pub trait Area<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd,
{
    /// calculates an unsigned area
    fn area(&self) -> T;
}

impl<T> Area<T> for Rect<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd,
{
    fn area(&self) -> T {
        let width = if self.p1.x < self.p2.x {
            self.p2.x - self.p1.x
        } else {
            self.p1.x - self.p2.x
        };
        let height = if self.p1.y < self.p2.y {
            self.p2.y - self.p1.y
        } else {
            self.p1.y - self.p2.y
        };
        width * height
    }
}
