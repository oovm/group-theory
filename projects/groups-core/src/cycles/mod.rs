use ndarray::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::{Definitions, Marker, Path};
use svg::Document;
use crate::{GroupError};
use std::iter::from_coroutine;


mod elements;
mod notations;

mod iterations;

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CycleElement {
    /// Use indexes to represent the permutation
    pub(crate) chain: Vec<usize>,
}

/// 群置换
#[derive(Debug)]
pub struct CycleNotation {
    /// Use indexes to represent the permutation
    pub(crate) cycles: Vec<CycleElement>,
}

