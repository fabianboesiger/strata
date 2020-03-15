mod load;
mod position;
mod join;
mod save;
mod colors;

pub use load::Load;
pub use position::Position;
pub use join::Join;
pub use save::Save;
pub use colors::Colors;
use image::{
    RgbImage,
    Rgb
};
use std::{
    path::PathBuf
};
use nalgebra::{
    Vector2,
    Vector3
};
use crate::error;

pub type Vector = Vector2<i32>;

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
    operator.add(Colors::new());
    operator.add(Join::new());
    operator.add(Save::new(output));
    operator.run()?;

    Ok(())
}
