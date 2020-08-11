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

pub struct Join {
}

impl Join {
    pub fn new() -> Join {
        Join {
        }
    }
}

impl Operation for Join {
    fn apply(&self, view: View) -> error::Result<View> {
        println!("Joining images ...");

        let dimensions = view.layers
            .par_iter()
            .map(|layer| {
                let l = layer.position.x;
                let t = layer.position.y;
                let r = layer.position.x + layer.image.width() as i32;
                let b = layer.position.y + layer.image.height() as i32;

                (l, t, r, b)
            })
            .reduce(|| (i32::max_value(), i32::max_value(), i32::min_value(), i32::min_value()), |(l1, t1, r1, b1), (l2, t2, r2, b2)| {
                (min(l1, l2), min(t1, t2), max(r1, r2), max(b1, b2))
            });

        let size = (dimensions.2 - dimensions.0, dimensions.3 - dimensions.1);

        debug_assert!(size.0 > 0 && size.1 > 0);

        let pixels = (dimensions.0..dimensions.2)
            .into_par_iter()
            .map(move |x| 
                (dimensions.1..dimensions.3)
                    .into_par_iter()
                    .map(move |y| Vector::new(x, y))
            )
            .flatten()
            .map(|position| {
                let colors = view.layers
                    .par_iter()
                    .enumerate()
                    .map(|(i, layer)| {
                        layer.get_pixel(&position)
                            .map(|pixel| {
                                let center = layer.position + Vector::new(layer.image.width() as i32, layer.image.height() as i32) / 2;
                                let distance = position - center;
                                (
                                    /*
                                    match i {
                                        0 => (255.0, 0.0, 0.0),
                                        1 => (255.0, 255.0, 0.0),
                                        2 => (0.0, 255.0, 0.0),
                                        3 => (0.0, 255.0, 255.0),
                                        _ => unreachable!()
                                    },
                                    */
                                    //((i % 2) as f64 * 255.0, (i % 4) as f64 * 255.0 / 3.0, (i % 8) as f64 * 255.0 / 7.0)

                                    (pixel[0] as f64, pixel[1] as f64, pixel[2] as f64),
                                    (1.0 / ((distance.x.pow(2) + distance.y.pow(2)) as f64).sqrt()).powf(4.0)
                                )
                            })
                    })
                    .filter_map(|x| x)
                    .collect::<Vec<((f64, f64, f64), f64)>>();
                
                let sum = colors
                    .par_iter()
                    .map(|(_, d)| d)
                    .sum::<f64>();
                
                let result = colors
                    .into_iter()
                    .fold((0.0, 0.0, 0.0), |acc, (c, d)| {
                        (acc.0 + c.0 * d, acc.1 + c.1 * d, acc.2 + c.2 * d)
                    });

                (
                    (position.x - dimensions.0) as u32,
                    (position.y - dimensions.1) as u32, 
                    ((result.0 / sum) as u8, (result.1 / sum) as u8, (result.2 / sum) as u8)
                )
            })
            .collect::<Vec<(u32, u32, (u8, u8, u8))>>();
        
        // Put pixels into a new image.
        let mut image = RgbImage::new(size.0 as u32, size.1 as u32);
        for pixel in pixels {
            let color = pixel.2;
            image.put_pixel(pixel.0, pixel.1, Rgb::from([color.0, color.1, color.2]));
        }

        Ok(View {
            layers: vec![Layer::new(image)]
        })
    }
}
