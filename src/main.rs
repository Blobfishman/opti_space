mod line;
mod placed_rects;
mod rect;

use crate::placed_rects::PlacedRects;
use crate::rect::{Area, Rect};
use std::error::Error;
use std::io::ErrorKind;

extern crate num_traits;

/// loads all to be placed polygons into an iterable collection
fn load(path: &str) -> Result<Vec<Rect<i64>>, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rects: Vec<Rect<i64>> = Vec::new();
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

    let max_width: i64 = args.get(2).unwrap().parse().unwrap();
    let max_height: i64 = args.get(3).unwrap().parse().unwrap();
    let border_width: i64 = args.get(4).unwrap().parse().unwrap();

    let output_path = args.get(1).unwrap();
    let mut recs = load(args.get(0).unwrap())?;
    if recs.is_empty() {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::InvalidData,
            "No elements in the list!",
        )));
    }

    // calculate minimal number of blocks based on max area of each block and the total area of
    // all polygons
    let mut total_area: i64 = 0;
    recs.iter().for_each(|element| {
        total_area += element.area();
    });
    // This calculation assumes that casting to an integer floors the value
    let minimal_block_count = ((total_area / (max_height * max_width)) as usize) + 1;
    // blocks holds all rectangles placed on areas caped by max_width and max_height
    let mut blocks: Vec<PlacedRects<i64>>;
    blocks.reserve(minimal_block_count);

    while !recs.is_empty() {}

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
