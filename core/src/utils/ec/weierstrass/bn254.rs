use hybrid_array::Array;
use num::{BigUint, Num, Zero};
use serde::{Deserialize, Serialize};

use super::{SwCurve, WeierstrassParameters};
use crate::operations::field::params::DEFAULT_NUM_LIMBS_T;
use crate::runtime::Syscall;
use crate::stark::WeierstrassAddAssignChip;
use crate::stark::WeierstrassDoubleAssignChip;
use crate::syscall::precompiles::create_ec_add_event;
use crate::syscall::precompiles::create_ec_double_event;
use crate::utils::ec::field::FieldParameters;
use crate::utils::ec::CurveType;
use crate::utils::ec::EllipticCurveParameters;
use crate::utils::ec::WithAddition;
use crate::utils::ec::WithDoubling;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Bn254 curve parameter
pub struct Bn254Parameters;

pub type Bn254 = SwCurve<Bn254Parameters>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Bn254 base field parameter
pub struct Bn254BaseField;

impl FieldParameters for Bn254BaseField {
    type NB_LIMBS = DEFAULT_NUM_LIMBS_T;

    const NB_BITS_PER_LIMB: usize = 8;

    const MODULUS: Array<u8, Self::NB_LIMBS> = Array([
        71, 253, 124, 216, 22, 140, 32, 60, 141, 202, 113, 104, 145, 106, 129, 151, 93, 88, 129,
        129, 182, 69, 80, 184, 41, 160, 49, 225, 114, 78, 100, 48,
    ]);

    // A rough witness-offset estimate given the size of the limbs and the size of the field.
    const WITNESS_OFFSET: usize = 1usize << 13;

    // The modulus has been taken from py_ecc python library by Ethereum Foundation.
    // https://github.com/ethereum/py_pairing/blob/5f609da/py_ecc/bn128/bn128_field_elements.py#L10-L11
    fn modulus() -> BigUint {
        BigUint::from_str_radix(
            "21888242871839275222246405745257275088696311157297823662689037894645226208583",
            10,
        )
        .unwrap()
    }
}

impl EllipticCurveParameters for Bn254Parameters {
    type BaseField = Bn254BaseField;

    const CURVE_TYPE: CurveType = CurveType::Bn254;
}

impl WithAddition for Bn254Parameters {
    fn add_events(
        record: &crate::runtime::ExecutionRecord,
    ) -> &[crate::syscall::precompiles::ECAddEvent<<Self::BaseField as FieldParameters>::NB_LIMBS>]
    {
        &record.bn254_add_events
    }
}

impl WithDoubling for Bn254Parameters {
    fn double_events(
        record: &crate::runtime::ExecutionRecord,
    ) -> &[crate::syscall::precompiles::ECDoubleEvent<
        <Self::BaseField as FieldParameters>::NB_LIMBS,
    >] {
        &record.bn254_double_events
    }
}

impl Syscall for WeierstrassAddAssignChip<Bn254> {
    fn execute(
        &self,
        rt: &mut crate::runtime::SyscallContext<'_>,
        arg1: u32,
        arg2: u32,
    ) -> Option<u32> {
        let event = create_ec_add_event::<Bn254>(rt, arg1, arg2);
        rt.record_mut().bn254_add_events.push(event);
        None
    }

    fn num_extra_cycles(&self) -> u32 {
        1
    }
}

impl Syscall for WeierstrassDoubleAssignChip<Bn254> {
    fn execute(
        &self,
        rt: &mut crate::runtime::SyscallContext<'_>,
        arg1: u32,
        arg2: u32,
    ) -> Option<u32> {
        let event = create_ec_double_event::<Bn254>(rt, arg1, arg2);
        rt.record_mut().bn254_double_events.push(event);
        None
    }
}

impl WeierstrassParameters for Bn254Parameters {
    // The values have been taken from py_ecc python library by Ethereum Foundation.
    // https://github.com/ethereum/py_pairing/blob/5f609da/py_ecc/bn128/bn128_field_elements.py
    const A: Array<u16, <Self::BaseField as FieldParameters>::NB_LIMBS> = Array([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);

    const B: Array<u16, <Self::BaseField as FieldParameters>::NB_LIMBS> = Array([
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]);
    fn generator() -> (BigUint, BigUint) {
        let x = BigUint::from(1u32);
        let y = BigUint::from(2u32);
        (x, y)
    }

    fn prime_group_order() -> BigUint {
        BigUint::from_str_radix(
            "21888242871839275222246405745257275088548364400416034343698204186575808495617",
            10,
        )
        .unwrap()
    }

    fn a_int() -> BigUint {
        BigUint::zero()
    }

    fn b_int() -> BigUint {
        BigUint::from(3u32)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::ec::utils::biguint_from_limbs;

    #[test]
    fn test_weierstrass_biguint_scalar_mul() {
        assert_eq!(
            biguint_from_limbs(&Bn254BaseField::MODULUS),
            Bn254BaseField::modulus()
        );
    }
}
