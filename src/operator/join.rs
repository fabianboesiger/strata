use super::{
    Operation,
    View,
    Layer,
    Vector
};
use std::{
    cmp::{
        min,
        max
    }
};
use image::{
    RgbImage,
    Rgb
};
use rayon::prelude::*;
use crate::error;

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
                let result = view.layers
                    .par_iter()
                    .map(|layer| {
                        layer.get_pixel(&position)
                            .map(|pixel| {
                                let center = layer.position + Vector::new(layer.image.width() as i32, layer.image.height() as i32) / 2;
                                let distance = position - center;
                                /*(distance.x.pow(2) + distance.y.pow(2)) as u32*/
                                [pixel[0] as u32, pixel[1] as u32, pixel[2] as u32]
                            })
                    })
                    .filter_map(|x| x)
                    .reduce(|| ([0, 0, 0]), |p1, p2| {
                        [max(p1[0], p2[0]), max(p1[1], p2[1]), max(p1[2], p2[2])]
                    });

                //println!("{:?}", result);

                (
                    (position.x - dimensions.0) as u32,
                    (position.y - dimensions.1) as u32, 
                    (result[0] as u8, result[1] as u8, result[2] as u8)
                )
            })
            .collect::<Vec<(u32, u32, (u8, u8, u8))>>();

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
