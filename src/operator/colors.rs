use super::{
    Operation,
    View,
    Vector,
};
use std::{
    iter::FromIterator,
    cmp::{
        min,
        max
    }
};
use rayon::prelude::*;
use crate::error;

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
