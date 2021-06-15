use core::cmp;
use std::error::Error;
use std::io::ErrorKind;

extern crate num_traits;

/// A 2d Point
#[derive(Clone, PartialEq, Debug)]
struct Point<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd,
{
    x: T,
    y: T,
}

/// a endless line described by two points
#[derive(Clone, PartialEq, Debug)]
struct Line<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd,
{
    p1: Point<T>,
    p2: Point<T>,
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
    fn intersection(&self, other: &Line<T>) -> Option<Point<T>> {
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

/// A rectangle constructed by two points. p1 describes the upper left corner and p2 the lower
/// right corner of the rectangle
#[derive(Clone, PartialEq, Debug)]
struct Rect<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd,
{
    p1: Point<T>,
    p2: Point<T>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Corner {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum RectLine {
    Upper,
    Right,
    Lower,
    Left,
}

impl<T> Rect<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign,
{
    /// constructs a rectangle based on its height and width placing it to (0, 0)
    fn new(width: T, height: T) -> Rect<T> {
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

    fn new_by_points(p1: &Point<T>, p2: &Point<T>, p3: &Point<T>, p4: &Point<T>) -> Rect<T> {
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
    fn move_by(&mut self, x_offset: T, y_offset: T) {
        self.p1.x += x_offset;
        self.p1.y += y_offset;
        self.p2.x += x_offset;
        self.p2.y += y_offset;
    }

    /// Moves the rectangle to the point to using the upper left corner as a reference point
    fn move_to(&mut self, to: Point<T>) {
        let d_x = self.p2.x - self.p1.x;
        let d_y = self.p2.y - self.p1.y;

        self.p1.x = to.x;
        self.p1.y = to.y;
        self.p2.x = to.x + d_x;
        self.p2.y = to.y + d_y;
    }

    /// Return the points
    fn get_corner(&self, corner: Corner) -> Point<T> {
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

    fn get_line_iterator(&self) -> RectLineIterator<T> {
        RectLineIterator {
            rect: self,
            line_id: RectLine::Upper,
        }
    }

    fn get_overlapped_area(&self, other: &Rect<T>) -> T {
        intersecting_area = cmp::max(
            0,
            cmp::min(other.p2.x, self.p2.x) - cmp::max(other.p1.x, self.p1.x),
        ) * cmp::max(
            0,
            cmp::min(other.p2.y, self.p2.y) - cmp::max(other.p1.y, self.p1.y),
        )
    }

    fn overlapped(&self, other: &Rect<T>) -> bool {
        self.get_overlapped_area(other) != 0
    }
}

struct RectLineIterator<'a, T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign,
{
    rect: &'a Rect<T>,
    line_id: RectLine,
}

impl<'a, T> Iterator for RectLineIterator<'a, T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign,
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
trait Area<T>
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

/// Represents a list of rectangles inside a bigger rectangle(size: height*width) which have
/// Additionally the spacing describes the required space between each place
/// rectangle
#[derive(Clone, PartialEq, Debug)]
struct PlacedRects<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign,
{
    height: T,
    width: T,
    rects: Vec<Rect<T>>,
    spacing: T,
}

impl<T> PlacedRects<T>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign,
{
    /// Calculates all maximal (area) empty areas, which does not contain any other mea or a
    /// placed rectangle, for a to-be-placed rectangle.
    fn calc_mea_for(&self, to_be_placed: Rect<T>) -> Vec<Rect<T>> {
        let mut rects = self.rects.to_vec();
        rects.push(to_be_placed);
        rects.push(Rect::new(self.width, self.height));

        // get all lines
        let mut lines: Vec<Line<T>> = Vec::with_capacity(rects.len() * 4);
        for rect in rects {
            for line in rect.get_line_iterator() {
                lines.push(line);
            }
        }
        // TODO: Remove duplicates
        lines.shrink_to_fit();

        // create rects
        let mut mea: Vec<Rect<T>> = Vec::with_capacity(lines.len().pow(4));
        for a in lines {
            for b in lines {
                if let Some(p1) = a.intersection(&b) {
                    for c in lines {
                        if let Some(p2) = b.intersection(&c) {
                            for d in lines {
                                if let Some(p3) = c.intersection(&d) {
                                    if let Some(p4) = d.intersection(&a) {
                                        mea.push(Rect::new_by_points(&p1, &p2, &p3, &p4));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        mea.shrink_to_fit();

        // Remove placed rectangles
        mea.retain(|e| rects.contains(e));
        mea.shrink_to_fit();

        // Order by area size
        mea.sort_by(|a, b| a.area().partial_cmp(&b.area()).unwrap());

        let mut upper_index = 0;
        let mut lower_index = mea.len() - 1;
        while upper_index == mea.len() - 1 {
            if lower_index == upper_index {
                // remove all rects which are part of the rectangle of the next upper_index
                lower_index = mea.len() - 1;
                upper_index += 1;
                continue;
            }
            // remove rect if it is part of this mea
            if &mea[upper_index].overlapped(&mea[lower_index]) {
                mea.remove(lower_index);
            }
            lower_index -= 1;
        }
        mea
    }

    /// Calculates all maximal (area) empty areas, which does not contain any other mea or a
    /// placed rectangle.
    fn calc_mea(&self) -> Vec<Vec<T>> {
        let mut mea: Vec<Vec<T>> = vec![];

        mea
    }
}

/// loads all to be placed polygons into an iterable collection
fn load(path: &str) -> Result<Vec<Rect<f64>>, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rects: Vec<Rect<f64>> = Vec::new();
    for record in rdr.records() {
        let rec = record?;
        if rec.len() == 2 {
            rects.push(Rect::new(
                rec.get(0).unwrap().parse().unwrap(),
                rec.get(1).unwrap().parse().unwrap(),
            ));
        }
    }
    Ok(rects)
}

/// executes the program. It is used as a convenience function to catch errors and simplify error
/// handling in the early dev stage
fn exec() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Missing arguments:\n path output-path max-width max-height border-width",
        )));
    }

    let max_width: f64 = args.get(2).unwrap().parse().unwrap();
    let max_height: f64 = args.get(3).unwrap().parse().unwrap();
    let border_width: f64 = args.get(4).unwrap().parse().unwrap();

    let output_path = args.get(1).unwrap();
    let recs = load(args.get(0).unwrap())?;
    if recs.is_empty() {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::InvalidData,
            "No elements in the list!",
        )));
    }

    // calculate minimal number of blocks based on max area of each block and the total area of
    // all polygons
    let mut total_area: f64 = 0.0;
    recs.iter().for_each(|element| {
        total_area += element.area();
    });
    // This calculation assumes that casting to an integer floors the value
    let minimal_block_count = ((total_area / (max_height * max_width)) as usize) + 1;
    /// blocks holds all rectangles placed on areas caped by max_width and max_height
    let mut blocks: Vec<PlacedRects<f64>>;
    blocks.reserve(minimal_block_count);

    Ok(())
}

fn main() {
    match exec() {
        Ok(_) => {
            println!("converting failed")
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
