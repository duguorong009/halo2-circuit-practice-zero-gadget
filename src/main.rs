pub mod is_zero_gadget;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Layouter, SimpleFloorPlanner, Value},
    dev::MockProver,
    pasta::Fp as F,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};
use is_zero_gadget::*;

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

    pub fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        a: F,
        b: F,
        c: F,
    ) -> Result<AssignedCell<F, F>, Error> {
        let is_zero_chip = IsZeroChip::construct(self.config.a_equals_b.clone());

        layouter.assign_region(
            || "f(a, b, c) = if a == b {c} else {a - b}",
            |mut region| {
                self.config.s.enable(&mut region, 0)?;
                region.assign_advice(|| "a", self.config.a, 0, || Value::known(a))?;
                region.assign_advice(|| "b", self.config.b, 0, || Value::known(b))?;
                region.assign_advice(|| "c", self.config.c, 0, || Value::known(c))?;
                is_zero_chip.assign(&mut region, 0, Value::known(a - b))?;

                let output = if a == b { c } else { a - b };
                region.assign_advice(|| "output", self.config.output, 0, || Value::known(output))
            },
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct CustomCircuit<F> {
    pub a: F,
    pub b: F,
    pub c: F,
}

impl<F: FieldExt> Circuit<F> for CustomCircuit<F> {
    type Config = CustomConfig<F>;

    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        CustomChip::configure(meta)
    }

    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error> {
        let chip = CustomChip::construct(config);

        chip.assign(layouter, self.a, self.b, self.c)?;

        Ok(())
    }
}

fn main() {
    let k = 4;

    let circuit = CustomCircuit {
        a: F::from(10),
        b: F::from(15),
        c: F::from(18),
    };

    let prover = MockProver::run(k, &circuit, vec![]).unwrap();
    prover.assert_satisfied();
}
