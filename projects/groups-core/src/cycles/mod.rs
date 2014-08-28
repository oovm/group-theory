use ndarray::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::{Definitions, Marker, Path};
use svg::Document;
use crate::GroupError;


/// 群置换
#[derive(Debug)]
pub struct Cycles {
    /// Use indexes to represent the permutation
    cycles: Vec<Vec<usize>>,
}

impl Cycles {
    /// 找到能将 last 列表变为 next 列表的置换
    fn find_permutation<T: PartialEq>(last: &[T], next: &[T]) -> Result<Self, GroupError> {
        // Check if the lengths of the input slices are equal
        if last.len() != next.len() {
            return Err(GroupError::invalid_permutation());
        }

        let mut cycles: Vec<Vec<usize>> = Vec::new();
        let mut visited: Vec<bool> = vec![false; last.len()];

        for i in 0..last.len() {
            if !visited[i] {
                let mut cycle: Vec<usize> = Vec::new();
                let mut j = i;

                loop {
                    visited[j] = true;
                    cycle.push(j);

                    let next_index = next.iter().position(|x| *x == last[j]);

                    match next_index {
                        Some(index) => j = index,
                        None => {
                            return Err(GroupError::invalid_permutation());
                        }
                    }

                    if j == i {
                        break;
                    }
                }
                if cycle.len() != 1 {
                    cycles.push(cycle);
                }
            }
        }

        Ok(Cycles { cycles })
    }
}

