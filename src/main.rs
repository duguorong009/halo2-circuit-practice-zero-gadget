use halo2_proofs::{
    arithmetic::FieldExt,
    plonk::{Advice, Column, Expression},
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct IsZeroConfig<F> {
    pub value_inv: Column<Advice>,
    pub is_zero_expr: Expression<F>,
}

pub struct IsZeroChip<F: FieldExt> {
    config: IsZeroConfig<F>,
}
