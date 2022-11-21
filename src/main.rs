pub mod is_zero_gadget;

use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, ConstraintSystem, Expression, Selector},
    poly::Rotation,
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

    pub fn configure(meta: &mut ConstraintSystem<F>) -> CustomConfig<F> {
        let s = meta.selector();

        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();

        let is_zero_advice_column = meta.advice_column();
        let a_equals_b = IsZeroChip::configure(
            meta,
            |meta| meta.query_selector(s),
            |meta| meta.query_advice(a, Rotation::cur()) - meta.query_advice(b, Rotation::cur()),
            is_zero_advice_column,
        );

        let output = meta.advice_column();

        meta.create_gate("f(a, b, c) = if a == b {c} else {a - b}", |meta| {
            let s = meta.query_selector(s);

            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());

            let output = meta.query_advice(output, Rotation::cur());

            vec![
                s.clone() * (a_equals_b.is_zero_expr.clone() * (output.clone() - c)),
                s * (Expression::Constant(F::one()) - a_equals_b.is_zero_expr.clone())
                    * (output.clone() - (a - b)),
            ]
        });

        CustomConfig {
            s,
            a,
            b,
            c,
            a_equals_b,
            output,
        }
    }
}
