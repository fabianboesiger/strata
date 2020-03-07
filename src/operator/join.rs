use super::{
    Operation,
    View,
    image_difference,
    Vector
};
use std::{
    iter::FromIterator,
    cmp::{
        min,
        max
    }
};
use rayon::prelude::*;
use partitions::PartitionVec;

pub struct Join {
}

impl Join {
    pub fn new() -> Join {
        Join {
        }
    }
}

impl Operation for Join {
    fn apply(&self, mut view: View) -> View {
        println!("Joining images");

        view
    }
}
