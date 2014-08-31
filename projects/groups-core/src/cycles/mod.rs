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

impl Default for Cycles {
    fn default() -> Self {
        Self {
            cycles: vec![],
        }
    }
}

impl Cycles {
    pub fn new(cycles: Vec<Vec<usize>>) -> Result<Self, GroupError> {
        Self { cycles }.standardize()
    }
    pub unsafe fn new_unchecked(cycles: Vec<Vec<usize>>) -> Self {
        Self { cycles }
    }
    fn standardize(&mut self) -> Result<Self, GroupError> {
        let mut cycles = Vec::with_capacity(self.cycles.len());
        for cycle in self.cycles.iter() {
            let mut cycle = cycle.clone();
            cycle.sort();
            cycle.dedup();
            cycles.push(cycle);
        }
        cycles.sort();
        cycles.dedup();
        Ok(Self { cycles })
    }

    /// Find the permutation that can turn the last list into the next list
    pub fn find_permutation<T: PartialEq>(last: &[T], next: &[T]) -> Result<Self, GroupError> {
        // Check if the lengths of the input slices are equal
        if last.len() != next.len() {
            return Err(GroupError::invalid_permutation());
        }

        let mut cycles: Vec<Vec<usize>> = Vec::with_capacity(4);
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

impl Cycles {
    /// Calculate the period of the permutation
    pub fn period(&self) -> usize {
        self.cycles.iter().map(|s| s.len()).sum()
    }
    /// Get the inverse cycle permutation
    pub fn inverse(&self) -> Self {
        let mut cycles = Vec::new();
        for cycle in self.cycles.iter() {
            let mut inverse_cycle = Vec::new();
            for i in cycle.iter().rev() {
                inverse_cycle.push(*i);
            }
            cycles.push(inverse_cycle);
        }
        Self { cycles }
    }
    pub fn swap_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::with_capacity(self.cycles.iter().map(|s| s.len()).sum());
        for cycle in self.cycles.iter() {
            for i in 0..cycle.len() {
                let current = *cycle.get(i).unwrap();
                let next = *cycle.get((i + 1) % cycle.len()).unwrap();
                pairs.push((current, next));
            }
        }
        pairs
    }

    /// Permute the data by cycles
    pub fn permute<T: Clone>(&self, data: &[T]) -> Vec<T> {
        let mut result = data.to_vec();
        unsafe { self.permute_in_place(&mut result) }
        result
    }
    /// Permute the data by cycles in-place
    pub unsafe fn permute_in_place<T>(&self, data: &mut [T]) {
        for cycle in self.cycles.iter() {
            debug_assert!(cycle.len() > 1, "Invalid Empty Cycle");
            let mut saved = None;
            for i in 0..cycle.len() {
                let current = *cycle.get_unchecked(i);
                let next = *cycle.get_unchecked((i + 1) % cycle.len());
                match saved {
                    Some(saved_value) => data.swap(saved_value, current),
                    None => saved = Some(current),
                }
                if cycle.len() == i + 1 {
                    data.swap(saved.unwrap_unchecked(), next);
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct PermutationRecord {
    /// 每两行之间的变换
    permutes: Vec<Vec<(usize, usize)>>,
    ///
    font_size: f32,
}

impl Default for PermutationRecord {
    fn default() -> Self {
        Self {
            permutes: vec![],
            font_size: 20.0,
        }
    }
}

impl PermutationRecord {
    pub fn new(permutes: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            permutes,
            ..Default::default()
        }
    }
    /// 绘制每一行的数字, 然后每两行之间有箭头, 表示数字的变换
    pub fn generate_svg(&self, data: &Array2<usize>) -> Document {
        let width = data.shape()[1] * 50;
        let height = data.shape()[0] * 50;

        let mut document = Document::new()
            .set("viewBox", (0, 0, width, height))
            .set("width", "100%")
            .set("height", "100%")
            .add(
                Definitions::new().add(
                    Marker::new()
                        .set("id", "arrowhead")
                        .set("viewBox", (0, 0, 10, 10))
                        .set("refX", 5)
                        .set("refY", 5)
                        .set("markerWidth", 6)
                        .set("markerHeight", 6)
                        .set("orient", "auto-start-reverse")
                        .add(Path::new().set("d", "M 0 0 L 10 5 L 0 10 z")),
                ),
            );

        for (row_index, row) in data.outer_iter().enumerate() {
            for (col_index, &value) in row.iter().enumerate() {
                let x = col_index * 50;
                let y = row_index * 50;

                let text = svg::node::element::Text::new()
                    .set("x", x + 20)
                    .set("y", y + 25)
                    .set("text-anchor", "middle")
                    .set("font-size", self.font_size)
                    .set("fill", "black")
                    .add(svg::node::Text::new(value.to_string()));
                document = document.add(text);
            }
        }

        for (row, permute) in self.permutes.iter().enumerate() {
            let y1 = row * 50 + 30;
            let y2 = row * 50 + 55;
            for (e1, e2) in permute {
                let x1 = e1 * 50 + 20;
                let x2 = e2 * 50 + 20;
                let arrow_path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("marker-end", "url(#arrowhead)") // 添加箭头标记
                    .set("d", Data::new().move_to((x1, y1)).line_to((x2, y2)));
                document = document.add(arrow_path);
            }
        }

        document
    }
}

fn main() {
    let ndarray_data = arr2(&[
        [1, 4, 5, 8, 9, 10, 11, 14, 19],
        [9, 4, 5, 14, 10, 19, 1, 8, 11],
        [10, 4, 5, 8, 19, 11, 9, 14, 1],
        [19, 4, 5, 14, 11, 1, 10, 8, 9],
        [11, 4, 5, 8, 1, 9, 19, 14, 10],
        [1, 4, 5, 14, 9, 10, 11, 8, 19],
        [9, 4, 5, 8, 10, 19, 1, 14, 11],
        [10, 4, 5, 14, 19, 11, 9, 8, 1],
        [19, 4, 5, 8, 11, 1, 10, 14, 9],
        [11, 4, 5, 14, 1, 9, 19, 8, 10],
    ]);
    // {{0, 4}, {1, 1}, {2, 2}, {3, 7}, {4, 5}, {5, 8}, {6, 0}, {7, 3}, {8,
    //   6}}
    let cycle = vec![
        (0, 6),
        (1, 1),
        (2, 2),
        (3, 7),
        (4, 0),
        (5, 4),
        (6, 8),
        (7, 3),
        (8, 5),
    ];
    let test = PermutationRecord::new(vec![
        cycle.clone(),
        cycle.clone(),
        cycle.clone(),
        cycle.clone(),
        cycle.clone(),
        cycle.clone(),
        cycle.clone(),
        cycle.clone(),
        cycle.clone(),
    ]);
    let svg = test.generate_svg(&ndarray_data);
    svg::save("test.svg", &svg).unwrap();
}
