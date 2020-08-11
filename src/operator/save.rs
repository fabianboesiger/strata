use super::{
    Operation,
    View,
    error,
};
use std::path::PathBuf;

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
    fn apply(&self, view: View) -> error::Result<View> {
        println!("Saving result to \"{}\" ...", self.path.display());

        debug_assert_eq!(view.layers.len(), 1);

        view.layers[0].image.save(&self.path)?;

        println!("Result saved, goodbye!");

        Ok(view)
    }
}