use super::{
    Operation,
    View,
    Layer
};
use std::path::PathBuf;

pub struct Options {
    paths: Vec<PathBuf>
}

impl Operation for Options {
    fn apply(&self, mut view: View) -> View {
        for path in &self.paths {
            let image = image::open(path).unwrap();
            view.layers.push(Layer::new(image));
        }
        view
    }
}