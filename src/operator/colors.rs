use super::{
    Operation,
    View,
    Vector,
    error,
};
use std::{
    iter::FromIterator,
    cmp::{
        min,
        max,
    }
};
use rayon::prelude::*;

pub struct Colors {
}

impl Colors {
    pub fn new() -> Colors {
        Colors {
        }
    }
}

impl Operation for Colors {
    fn apply(&self, mut view: View) -> error::Result<View> {
        println!("Adjusting colors ...");



        Ok(view)
    }
}   
