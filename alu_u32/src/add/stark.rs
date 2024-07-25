use super::columns::Add32Cols;
use super::Add32Chip;
use core::borrow::Borrow;

use crate::add::columns::NUM_ADD_COLS;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::PrimeField;
use p3_matrix::MatrixRowSlices;

impl<F> BaseAir<F> for Add32Chip {
    fn width(&self) -> usize {
        NUM_ADD_COLS
    }
}

impl<F, AB> Air<AB> for Add32Chip
where
    F: PrimeField,
    AB: AirBuilder<F = F>,
{
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: &Add32Cols<AB::Var> = main.row_slice(0).borrow();

        let one = AB::F::one();
        let base = AB::F::from_canonical_u32(1 << 8);

        let carry_1 = local.carry[0];
        let carry_2 = local.carry[1];
        let carry_3 = local.carry[2];
        let carry_4 = local.carry[3];

        let overflow_0 = local.input_1[3] + local.input_2[3] - local.sum[3];            // 255 + 255 - 254 = 256
        let overflow_1 = local.input_1[2] + local.input_2[2] - local.sum[2] + carry_1;  // 255 + 255 - 255 + 1 = 256
        let overflow_2 = local.input_1[1] + local.input_2[1] - local.sum[1] + carry_2;  // 255 + 255 - 255 + 1 = 256
        let overflow_3 = local.input_1[0] + local.input_2[0] - local.sum[0] + carry_3;  // 255 + 255 - 255 + 1 = 256

        // Limb constraints
        builder.assert_zero(overflow_0.clone() * (overflow_0.clone() - base.clone()));
        builder.assert_zero(overflow_1.clone() * (overflow_1.clone() - base.clone()));
        builder.assert_zero(overflow_2.clone() * (overflow_2.clone() - base.clone()));
        builder.assert_zero(overflow_3.clone() * (overflow_3.clone() - base.clone()));

        // Carry constraints
        builder.assert_zero(
            overflow_0.clone() * (carry_1 - one) + (overflow_0 - base.clone()) * carry_1,
        );
        builder.assert_zero(
            overflow_1.clone() * (carry_2 - one) + (overflow_1 - base.clone()) * carry_2,
        );
        builder.assert_zero(overflow_2.clone() * (carry_3 - one) + (overflow_2.clone() - base) * carry_3);
        builder.assert_zero(overflow_3.clone() * (carry_4 - one) + (overflow_3 - base) * carry_4);
        builder.assert_bool(carry_1);
        builder.assert_bool(carry_2);
        builder.assert_bool(carry_3);
        builder.assert_bool(carry_4);

        builder.when(local.is_add).assert_eq(local.sum[0], local.output[0]);
        builder.when(local.is_add).assert_eq(local.sum[1], local.output[1]);
        builder.when(local.is_add).assert_eq(local.sum[2], local.output[2]);
        builder.when(local.is_add).assert_eq(local.sum[3], local.output[3]);

        builder.when(local.is_carry).assert_eq(carry_4, local.output[3]);
        builder.when(local.is_carry).assert_zero(local.output[0]+local.output[1]+local.output[2]);
    }
}
