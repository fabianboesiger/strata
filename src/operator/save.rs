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

pub struct Save {
    path: PathBuf
}

impl Save {
    pub fn new(path: PathBuf) -> Save {
        Save {
            path
        }
    }
}

impl Operation for Save {
    fn apply(&self, mut view: View) -> View {
        println!("Saving result to \"{}\"", self.path.display());

        debug_assert_eq!(view.layers.len(), 0);

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

        view
    }
}