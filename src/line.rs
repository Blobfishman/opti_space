extern crate num_traits;

/// A 2d Point
#[derive(Clone, PartialEq, Debug)]
pub struct Point<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd,
{
    pub(crate) x: T,
    pub(crate) y: T,
}

/// a endless line described by two points
#[derive(Clone, PartialEq, Debug)]
pub struct Line<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd,
{
    pub(crate) p1: Point<T>,
    pub(crate) p2: Point<T>,
}

impl<T> Line<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd,
{
    /// Calculate the intersection of two lines
    /// It returns a point in case of an intersection, if the lines are parallel or the same line
    /// None is returned.
    ///
    /// Based on: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    pub fn intersection(&self, other: &Line<T>) -> Option<Point<T>> {
        let denominator = ((self.p1.x - self.p2.x) * (other.p1.y - other.p2.y))
            - ((self.p1.y - self.p2.y) * (other.p1.x - other.p2.x));
        if denominator == T::zero() {
            return None;
        }

        return Some(Point {
            x: ((((self.p1.x * self.p2.y) - (self.p1.y * self.p2.x)) * (other.p1.x - other.p2.x))
                - ((self.p1.x - self.p2.x)
                    * ((other.p1.x * other.p2.y) - (other.p1.y * other.p2.x))))
                / denominator,
            y: ((((self.p1.x * self.p2.y) - (self.p1.y * self.p2.x)) * (other.p1.y - other.p2.y))
                - ((self.p1.y - self.p2.y)
                    * ((other.p1.x * other.p2.y) - (other.p1.y * other.p2.x))))
                / denominator,
        });
    }
}
