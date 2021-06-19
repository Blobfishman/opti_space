use crate::line::Line;
use crate::rect::{Area, Rect};

fn calc_mea<T>(rects: Vec<Rect<T>>) -> Vec<Rect<T>>
where
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign + Ord,
{
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
        if mea[upper_index].overlapped(&mea[lower_index]) {
            mea.remove(lower_index);
        }
        lower_index -= 1;
    }
    mea
}

/// Represents a list of rectangles inside a bigger rectangle(size: height*width) which have
/// Additionally the spacing describes the required space between each place
/// rectangle
#[derive(Clone, PartialEq, Debug)]
pub struct PlacedRects<T>
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
    T: Copy + num_traits::Num + std::cmp::PartialOrd + std::ops::AddAssign + Ord,
{
    /// Calculates all maximal (area) empty areas, which does not contain any other mea or a
    /// placed rectangle, for a to-be-placed rectangle.
    pub fn calc_mea_for(&self, to_be_placed: Rect<T>) -> Vec<Rect<T>> {
        let mut rects = self.rects.to_vec();
        rects.push(to_be_placed);
        rects.push(Rect::new(self.width, self.height));

        calc_mea(rects)
    }

    /// Calculates all maximal (area) empty areas, which does not contain any other mea or a
    /// placed rectangle.
    pub fn calc_mea(&self) -> Vec<Rect<T>> {
        let mut rects = self.rects.to_vec();
        rects.push(Rect::new(self.width, self.height));

        calc_mea(rects)
    }
}
