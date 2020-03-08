mod load;
mod position;
mod join;
mod save;

pub use load::Load;
pub use position::Position;
pub use join::Join;
pub use save::Save;
use image::{
    RgbImage,
    Rgb
};
use std::{
    path::PathBuf,
    cmp::{
        min, 
        max
    }
};
use rayon::prelude::*;
use nalgebra::Vector2;
use crate::error;

pub type Vector = Vector2<i32>;

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

#[derive(Clone)]
pub struct Layer {
    pub position: Vector,
    pub image: RgbImage
}

impl Layer {
    fn new(image: RgbImage) -> Layer {
        Layer {
            image,
            position: Vector::zeros()
        }
    }

    fn get_pixel(&self, position: &Vector) -> Option<&Rgb<u8>> {
        let absolute_position = position - self.position;

        if absolute_position.x >= 0
            && absolute_position.y >= 0
            && absolute_position.x < self.image.width() as i32
            && absolute_position.y < self.image.height() as i32
        {
            Some(self.image.get_pixel(absolute_position.x as u32, absolute_position.y as u32))
            //Some(Rgb::from([self.position.x as u8 * 32, self.position.y as u8 * 32, self.position.x as u8 * self.position.y as u8]))
        } else {
            None
        }
    }
}

pub trait Operation {
    fn apply(&self, view: View) -> error::Result<View>;
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

    pub fn run(&self) -> error::Result<()> {
        let mut view = View::default();

        for operation in &self.operations {
            view = operation.apply(view)?;
        }

        Ok(())
    }
}

pub async fn run(input: PathBuf, output: PathBuf) -> error::Result<()> {
    let mut operator = Operator::default();
    operator.add(Load::new(input));
    operator.add(Position::new());
    operator.add(Join::new());
    operator.add(Save::new(output));
    operator.run()?;

    Ok(())
}
