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
    imageops::FilterType
};
use rayon::prelude::*;
use crate::error;

pub struct Load {
    path: PathBuf,
    preview: bool
}

impl Load {
    pub fn new(path: PathBuf) -> Load {
        Load {
            path,
            preview: false
        }
    }
}

impl Operation for Load {
    fn apply(&self, mut view: View) -> error::Result<View> {
        println!("Loading images from \"{}\" ...", self.path.display());

        view.layers = fs::read_dir(&self.path)
            .unwrap()
            .into_iter()
            .map(|path| path.unwrap().path())
            .collect::<Vec<PathBuf>>()
            .par_iter()
            .map(|path| {
                let mut result = image::open(path).unwrap();
                if self.preview {
                    result = result.resize(1024, 1024, FilterType::Gaussian);
                }
                println!("Finished loading image \"{}\"", path.display());
                result
            })
            .map(|image|
                if let DynamicImage::ImageRgb8(image) = image {
                    Layer::new(image)
                } else {
                    panic!()
                }
            )
            .collect::<Vec<Layer>>();

        Ok(view)
    }
}