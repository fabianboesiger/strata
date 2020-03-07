mod load;
mod position;

pub use load::Load;
pub use position::Position;
use image::{
    RgbImage,
    Rgb
};
use std::cmp::{
    min, 
    max
};
use rayon::prelude::*;

type Vector = (i32, i32);

fn image_difference(i1: &RgbImage, i2: &RgbImage, i2_rel_to_i1: Vector, density: u32) -> f32 {
    let p1 = (max(0, i2_rel_to_i1.0), max(0, i2_rel_to_i1.1));
    let p2 = (min(0, -i2_rel_to_i1.0), min(0, -i2_rel_to_i1.1));
    let l = max(p1.0, p2.0);
    let r = min(
        p1.0 + i1.width() as i32, 
        p1.0 + i1.width() as i32
    );
    let t = max(p1.1, p2.1);
    let b = min(
        p1.1 + i2.height() as i32, 
        p1.1 + i2.height() as i32
    );
    let size = (r - l, b - t);
    (0..size.0)
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
            let error = ((
                (a[0] as i32 - b[0] as i32).pow(2) + 
                (a[1] as i32 - b[1] as i32).pow(2) + 
                (a[2] as i32 - b[2] as i32).pow(2)
            ) as f32).sqrt();
            error
        })
        .reduce(|| 0.0, |acc, e| acc + e)
}

#[derive(Clone)]
pub struct Layer {
    pub position: Vector,
    pub image: RgbImage
}

impl Layer {
    fn new(image: RgbImage) -> Layer {
        Layer {
            image,
            position: (0, 0)
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> Option<&Rgb<u8>> {
        let rx = x - self.position.0;
        let ry = y - self.position.1;
        if rx >= 0 && rx < self.image.width() as i32 && ry >= 0 && ry < self.image.height() as i32 {
            Some(self.image.get_pixel(rx as u32, ry as u32))
        } else {
            None
        }
    }

    fn difference(&self, other: &Layer) -> f32 {
        let l = max(self.position.0, other.position.0);
        let r = min(
            self.position.0 + self.image.width() as i32, 
            other.position.0 + other.image.width() as i32
        );
        let t = max(self.position.1, other.position.1);
        let b = min(
            self.position.1 + self.image.height() as i32, 
            other.position.1 + other.image.height() as i32
        );

        (l..(r - l))
            .into_par_iter()
            .map(|x| 
                (t..(b - t))
                    .into_par_iter()
                    .map(move |y| (x, y))
            )
            .flatten()
            .map(|(x, y)| {
                let a = self.get_pixel(x, y).unwrap();
                let b = other.get_pixel(x, y).unwrap();
                let error = ((
                    (a[0] as i32 - b[0] as i32).pow(2) + 
                    (a[1] as i32 - b[1] as i32).pow(2) + 
                    (a[2] as i32 - b[2] as i32).pow(2)
                ) as f32).sqrt();
                error
            })
            .reduce(|| 0.0, |acc, e| acc + e)
    }
}

pub trait Operation {
    fn apply(&self, view: View) -> View;
}

#[derive(Clone, Default)]
pub struct View {
    pub layers: Vec<Layer>
}

#[derive(Default)]
pub struct Operator {
    operations: Vec<Box<dyn Operation>>
}

impl Operator {
    // Adds an operation to the operator.
    pub fn add<O: Operation + 'static>(&mut self, operation: O) {
        self.operations.push(Box::new(operation));
    }

    pub fn run(&self) {
        let mut view = View::default();
        for operation in &self.operations {
            view = operation.apply(view);
        }
    }
}