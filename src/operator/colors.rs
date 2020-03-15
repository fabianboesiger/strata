use super::{
    Operation,
    View,
    image_difference,
    Vector,
    ColorVector
};
use std::{
    iter::FromIterator,
    cmp::{
        min,
        max
    }
};
use rayon::prelude::*;
use crate::error;

pub struct Colors {
}

impl Colors {
    pub fn new() -> Colors {
        Colors {
        }
    }
}

impl Operation for Colors {
    fn apply(&self, mut view: View) -> error::Result<View> {
        println!("Adjusting colors ...");

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
                    .map(move |(n2, l2)| {

                    })
            )

        Ok(view)
    }
}
