use super::{
    Operation,
    View,
    image_difference,
    Vector
};
use std::{
    iter::FromIterator,
    cmp::{
        min,
        max
    }
};
use rayon::prelude::*;
use partitions::PartitionVec;

pub struct Position {
}

impl Position {
    pub fn new() -> Position {
        Position {
        }
    }
}

impl Operation for Position {
    fn apply(&self, mut view: View) -> View {
        println!("Finding relative positions of images");

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
                    ) as f32 / 16.0).log2() as u32;

                println!("r = {}", r);

                for g in (0..=r).map(|x| 2_u32.pow(r - x)) {
                    println!("Now searching area {} {} {} {}", px, py, px + rx, py + ry);

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
                                i2_rel_to_i1,
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

                    println!("Best position with granularity {} was {:?}", g, result);

                    px = (result.0).x - g as i32;
                    rx = g as i32 * 2;
                    py = (result.0).y - g as i32;
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

        println!("{:?}", matches);

        for (i1, i2, i2_rel_to_i1, _) in matches {
            if !partitions.same_set(i1, i2) {
                let move_to = (partitions[i1] + i2_rel_to_i1) - partitions[i2];
                for (_, position) in partitions.set_mut(i1) {
                    *position += move_to;
                }
                partitions.union(i1, i2);
            }
            if partitions.amount_of_sets() == 1 {
                break;
            }
        }

        println!("{:?}", partitions);
        
        for (i, position) in partitions.into_iter().enumerate() {
            view.layers[i].position = position;
        }

        view
    }
}
