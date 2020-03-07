use super::{
    Operation,
    View,
    image_difference
};
use std::{
    cmp::{
        min,
        max,
        Ordering
    }
};
use rayon::prelude::*;

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
        let matches = view.layers
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
                let mut result = ((0, 0), 0.0);
                for g in (6..=0_u32).map(|x| 2_u32.pow(x)) {
                    result = (px..(px + rx))
                        .into_par_iter()
                        .filter(move |i| i % g as i32 == 0)
                        .map(move |x| 
                            (py..(py + ry))
                                .into_par_iter()
                                .filter(move |i| i % g as i32 == 0)
                                .map(move |y| (x, y))
                        )
                        .flatten()
                        // Iterates through all possible image positions.
                        .map(move |(x, y)| {
                            let i2_rel_to_i1 = (x, y);
                            ((i2_rel_to_i1), image_difference(
                                &l1.image, 
                                &l2.image,
                                i2_rel_to_i1,
                                g
                            ))
                        })
                        .reduce(|| ((0, 0), 1.0), |acc, x| {
                            if x.1 < acc.1 {
                                x
                            } else {
                                acc
                            }
                        });

                    px = (result.0).0 - g as i32;
                    rx = g as i32 * 2;
                    py = (result.0).1 - g as i32;
                    ry = g as i32 * 2;
                }
                (n1, n2, result.0)
            })
            .collect::<Vec<(usize, usize, (i32, i32))>>();

        println!("{:?}", matches);

        for (i, layer) in view.layers.iter_mut().enumerate().skip(1) {

            let position = (0.0, 0.0);

            matches
                .par_iter()
                .filter(|(_, n, _)| *n == i)
                .map(|| (0, (0.0, 0.0)))

            layer.position = (0, 0);
                
        }
        
        view
    }
}