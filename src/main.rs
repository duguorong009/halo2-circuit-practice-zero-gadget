pub mod is_zero_gadget;

use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Selector},
};
use is_zero_gadget::*;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
pub struct CustomConfig<F> {
    s: Selector,
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    a_equals_b: IsZeroConfig<F>,
    output: Column<Advice>,
}

#[derive(Debug)]
pub struct CustomChip<F> {
    config: CustomConfig<F>,
}

impl<F: FieldExt> CustomChip<F> {
    pub fn construct(config: CustomConfig<F>) -> Self {
        CustomChip { config }
    }
}
