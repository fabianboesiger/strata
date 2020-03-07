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
    DynamicImage
};
use rayon::prelude::*;

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
    fn apply(&self, mut view: View) -> View {
        view.layers = fs::read_dir(&self.path)
            .unwrap()
            .into_iter()
            .map(|path| path.unwrap().path())
            .collect::<Vec<PathBuf>>()
            .par_iter()
            .map(|path| {
                image::open(path).unwrap()
            })
            .map(|image|
                if let DynamicImage::ImageRgb8(image) = image {
                    Layer::new(image)
                } else {
                    panic!()
                }
            )
            .collect::<Vec<Layer>>();

        

        view
    }
}