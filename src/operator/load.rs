use super::{
    Operation,
    View,
    Layer
};
use std::{
    fs,
    path::PathBuf
};
use image::{
    DynamicImage,
    //imageops::FilterType
};
use rayon::prelude::*;
use crate::error;

pub struct Load {
    path: PathBuf
}

impl Load {
    pub fn new(path: PathBuf) -> Load {
        Load {
            path
        }
    }
}

impl Operation for Load {
    fn apply(&self, mut view: View) -> error::Result<View> {
        println!("Loading images from \"{}\"", self.path.display());

        view.layers = fs::read_dir(&self.path)
            .unwrap()
            .into_iter()
            .map(|path| path.unwrap().path())
            .collect::<Vec<PathBuf>>()
            .par_iter()
            .map(|path| {
                image::open(path).unwrap()/*.resize(1024, 1024, FilterType::Gaussian)*/
            })
            .map(|image|
                if let DynamicImage::ImageRgb8(image) = image {
                    println!("Loaded image");
                    Layer::new(image)
                } else {
                    panic!()
                }
            )
            .collect::<Vec<Layer>>();

        Ok(view)
    }
}