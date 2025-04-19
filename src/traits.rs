use crate::{Owo, RoundingMode};



pub trait BatchOperations {
    fn multiply_all(&self, scalar: f64) -> Vec<Owo>;
    fn divide_all(&self, scalar: f64) -> Vec<Owo>;
    fn percentage_all(&self, percent: f64) -> Vec<Owo>;
    fn multiply_all_with_mode(&self, scalar: f64, mode: RoundingMode) -> Vec<Owo>;
    fn divide_all_with_mode(&self, scalar: f64, mode: RoundingMode) -> Vec<Owo>;
    fn percentage_all_with_mode(&self, percent: f64, mode: RoundingMode) -> Vec<Owo>;
}
