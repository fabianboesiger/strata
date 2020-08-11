use super::{
    Operation,
    View,
    Layer,
    Vector,
    error,
};
use std::{
    cmp::{
        min,
        max,
    }
};
use image::{
    RgbImage,
    Rgb
};
use rayon::prelude::*;

pub struct Sharpness;

impl Sharpness {
    pub fn new() -> Sharpness {
        Sharpness {
        }
    }
}

impl Operation for Sharpness {
    fn apply(&self, mut view: View) -> error::Result<View> {
        println!("Calculating sharpness ...");

        view.layers
            .par_iter()
            .map(|layer| {
                layer.sharpness = (dimensions.0..dimensions.2)
                    .into_par_iter()
                    .map(move |x| 
                        (dimensions.1..dimensions.3)
                            .into_par_iter()
                            .map(move |y| Vector::new(x, y))
                    )
                    .flatten()
                    .map(|position| {
                        
                    })
                    .collect::<Vec<f32>>();
            });

        Ok(view)
    }
}
