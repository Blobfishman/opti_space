use std::error::Error;
use std::io::ErrorKind;
use std::ops::{Add, Div, Mul, Sub};

/// A 2d Point
struct Point<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + std::cmp::PartialOrd,
{
    x: T,
    y: T,
}

/// A rectangle constructed by two points
struct Rect<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + std::cmp::PartialOrd,
{
    p1: Point<T>,
    p2: Point<T>,
}

impl<T> Rect<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + std::cmp::PartialOrd,
{
    fn new(width: T, height: T) -> Rect<T> {
        Rect {
            p1: Point {
                x: width - width,
                y: height - height,
            },
            p2: Point {
                x: width,
                y: height,
            },
        }
    }
}

/// This trait describes the functions of a 2d object which creates a plane
trait Area<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + std::cmp::PartialOrd,
{
    /// calculates an unsigned area
    fn area(&self) -> T;
}

impl<T> Area<T> for Rect<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + std::cmp::PartialOrd,
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
    let c_min = ((total_area / (max_height * max_width)) as i64) + 1;

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
