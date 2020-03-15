use super::{
    Operation,
    View,
    Vector
};
use std::{
    iter::FromIterator,
    cmp::{
        min,
        max
    }
};
use image::{
    RgbImage
};
use rayon::prelude::*;
use partitions::PartitionVec;
use crate::error;


// Calculates the difference between two images.
fn image_difference(i1: &RgbImage, i2: &RgbImage, i2_rel_to_i1: &Vector, density: u32) -> f32 {
    let p1 = (max(0, i2_rel_to_i1.x), max(0, i2_rel_to_i1.y));
    let p2 = (max(0, -i2_rel_to_i1.x), max(0, -i2_rel_to_i1.y));
    let size = (
        min(i1.width() as i32 - i2_rel_to_i1.x, i2.width() as i32 + i2_rel_to_i1.x),
        min(i1.height() as i32 - i2_rel_to_i1.y, i2.height() as i32 + i2_rel_to_i1.y)
    );
    let result = (0..size.0)
        .into_par_iter()
        .filter(|i| i % density as i32 == 0)
        .map(|x| 
            (0..size.1)
                .into_par_iter()
                .filter(|i| i % density as i32 == 0)
                .map(move |y| (x, y))
        )
        .flatten()
        .map(|(x, y)| {
            let a = i1.get_pixel((x + p1.0) as u32, (y + p1.1) as u32);
            let b = i2.get_pixel((x + p2.0) as u32, (y + p2.1) as u32);
            let error = (
                ((a[0] as f32 - b[0] as f32) / 256.0).powi(2) + 
                ((a[1] as f32 - b[1] as f32) / 256.0).powi(2) + 
                ((a[2] as f32 - b[2] as f32) / 256.0).powi(2)
            ).sqrt();
            (error, 1)
        })
        .reduce(|| (0.0, 1), |acc, e| ((acc.0 + e.0), (acc.1 + e.1)));

    let result = result.0 / result.1 as f32;

    result
}

pub struct Position {
}

impl Position {
    pub fn new() -> Position {
        Position {
        }
    }
}

impl Operation for Position {
    fn apply(&self, mut view: View) -> error::Result<View> {
        println!("Finding relative positions of images ...");

        // Matches contains a vector with the positions of the images relative to each other.
        let mut matches = view.layers
            .par_iter()
            .enumerate()
            .map(|(n1, l1)|
                view.layers
                    .par_iter()
                    .enumerate()
                    .filter(move |(n2, _)|
                        n1 < *n2
                    )
                    .map(move |(n2, l2)| ((n1, l1), (n2, l2)))
            )
            .flatten()
            // Iterates through all possible layer combinations.
            .map(|((n1, l1), (n2, l2))| {
                let mut px = -(min(l1.image.width(), l2.image.width()) as i32 / 2);
                let mut rx = max(l1.image.width(), l2.image.width())  as i32;
                let mut py = -(min(l1.image.height(), l2.image.height()) as i32 / 2);
                let mut ry = max(l1.image.height(), l2.image.height())  as i32;
                let mut result = (Vector::zeros(), 0.0);

                let r =
                    (min(
                        min(l1.image.width(), l1.image.height()), 
                        min(l2.image.width(), l2.image.height())
                    ) as f32 / 32.0).log2() as u32;

                for g in (0..=r).map(|x| 2_u32.pow(r - x)) {

                    result = (px..(px + rx))
                        .into_par_iter()
                        .filter(move |i| i % g as i32 == 0)
                        .map(move |x| 
                            (py..(py + ry))
                                .into_par_iter()
                                .filter(move |i| i % g as i32 == 0)
                                .map(move |y| Vector::new(x, y))
                        )
                        .flatten()
                        // Iterates through all possible image positions.
                        .map(move |i2_rel_to_i1| {
                            (i2_rel_to_i1, image_difference(
                                &l1.image, 
                                &l2.image,
                                &i2_rel_to_i1,
                                g
                            ))
                        })
                        .reduce(|| (Vector::zeros(), 1.0), |a, b| {
                            if a.1 < b.1 {
                                a
                            } else {
                                b
                            }
                        });

                    println!("Searched area {} {} {} {}, best position was {} {}.", px, py, px + rx, py + ry, result.0.x, result.0.y);

                    px = result.0.x - g as i32;
                    rx = g as i32 * 2;
                    py = result.0.y - g as i32;
                    ry = g as i32 * 2;
                }
                (n1, n2, result.0, result.1)
            })
            .collect::<Vec<(usize, usize, Vector, f32)>>();

        // We now perform Kruskal's algorithm to join the images.
        let mut partitions = PartitionVec::from_iter((0..view.layers.len()).map(|_| Vector::zeros()));
        matches.par_sort_by(|(_, _, _, e1), (_, _, _, e2)|
            e1.partial_cmp(e2).unwrap()
        );

        println!("Matches are: {:?}", matches);

        for (i1, i2, i2_rel_to_i1, _) in matches {
            if !partitions.same_set(i1, i2) {
                let move_to = (partitions[i1] + i2_rel_to_i1) - partitions[i2];
                for (_, position) in partitions.set_mut(i2) {
                    *position += move_to;
                }
                partitions.union(i1, i2);
            }
            if partitions.amount_of_sets() == 1 {
                break;
            }
        }

        println!("Partitions are: {:?}", partitions);
        
        for (i, position) in partitions.into_iter().enumerate() {
            view.layers[i].position = position;
        }

        Ok(view)
    }
}
