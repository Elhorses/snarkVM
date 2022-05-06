// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<E: Environment> Zero for Field<E> {
    type Boolean = Boolean<E>;

    fn zero() -> Self {
        E::zero().into()
    }

    fn is_zero(&self) -> Self::Boolean {
        self.is_equal(&Field::zero())
    }
}

impl<E: Environment> Metadata<dyn Zero<Boolean = Boolean<E>>> for Field<E> {
    type Case = ();
    type OutputType = CircuitType<Field<E>>;

    fn count(_case: &Self::Case) -> Count {
        Count::is(0, 0, 0, 0)
    }

    fn output_type(_case: Self::Case) -> Self::OutputType {
        CircuitType::from(Field::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_circuits_environment::Circuit;

    #[test]
    fn test_zero() {
        let zero = <Circuit as Environment>::BaseField::zero();

        Circuit::scope("Zero", || {
            assert_scope!(0, 0, 0, 0);
            let candidate = Field::<Circuit>::zero();
            assert_eq!(zero, candidate.eject_value());
            assert_count!(Zero<Boolean>() => Field, &());
            assert_output_type!(Zero<Boolean>() => Field, (), candidate);
        });
    }

    #[test]
    fn test_is_zero() {
        let candidate = Field::<Circuit>::zero();
        // Should equal 0.
        assert!(candidate.is_zero().eject_value());
        // Should not equal 1.
        assert!(!candidate.is_one().eject_value());
    }
}
