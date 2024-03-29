/*
 * indent-stack
 *
 * Copyright (C) 2019 chankyin
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/// This macro generates a struct that implements `Modular`.
///
/// See [`ExampleModular101`](struct.ExampleModular101.html) for an example of what is generated.
///
/// # Parameters
/// - `$name` is the name of the struct to be generated.
/// - `$int` is the type of integer to work with. It must be one of `u8`, `u16`, `u32`, `u64` or
///   `u128`.
/// - `$sint` is the type of integer to work with. It must be the signed version of `$int`, i.e.
///   `i8`, `i16`, `i32`, `i64` or `i128`. 
/// - `$mod` is the modulus of this type of modular value. It must be a const value. It must
///   satisfy * $mod < $int::max_value()` and `$mod * 2 < $sint::max_value()`.
/// - `$label` is a dummy label name for static assertions. This is unused on nightly builds with
///   the `underscore_const_names` feature.
///
/// # Example
/// ```ignore
/// def_modular!(ExampleModular101 : u16 | i16, 101 ; some_random_label
///              #[doc = "your own documentation here"]);
/// ```
#[macro_export]
macro_rules! def_modular {
    ($name:ident : $int:ty | $sint:ty, $mod:expr ; $label:ident $(#[$docs:meta])*) => {
        #[allow(unused)]
        mod $label {
            use alloc::vec::Vec;
            use core::fmt::Debug;
            use core::ops::{Add, Div, Mul, Rem, Sub};

            static_assertions::assert_impl!(impl_modular; $int, Copy, Debug, Default, Add, Sub, Mul, Rem);
            static_assertions::const_assert!(overflow_check; {
                ($mod as u128) <= (<$int>::max_value() as u128) &&
                    ($mod as u128) + ($mod as u128) <= (<$sint>::max_value() as u128) &&
                    ($mod as u128) <= (u64::max_value() as u128) && // self < u64::max is required for u128 (and automatically true for all other types)
                    ($mod as u128) * ($mod as u128) < (<$int>::max_value() as u128) // squared must not overflow since it is less than u64::max
            });

            lazy_static::lazy_static! {
                pub static ref IS_MOD_PRIME: bool = {
                    $crate::is_prime($mod)
                };

                pub static ref MOD_PRIME_FACTOR_LIST: Vec<$int> = {
                    $crate::list_prime_factors($mod)
                };
            }
        }

        $(#[$docs])*
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct $name($int);

        impl $name {
            /// Instantiated from the signed type, useful for literal instantiation.
            ///
            /// Use the `From` conversions instead if the input is not a literal.
            const fn new(mut int: $sint) -> Self {
                int %= $mod; // now int is in (-$mod, $mod)
                int += $mod; // now int is in (0, $mod * 2),
                int %= $mod; // now int is in [0, $mod)
                Self(int as $int)
            }
        }

        impl $crate::Modular<$int> for $name {
            const MOD: $int = $mod;

            fn remainder(&self) -> $int { self.0 }

            fn pow(self, n: usize) -> Self {
                if n == 0 {
                    return Self(1);
                }
                let root = self.pow(n / 2);
                let result = root * root;
                if n % 1 == 1 {
                    result * self
                } else {
                    result
                }
            }

            fn prime_inv(self) -> Self {
                assert!(*$label::IS_MOD_PRIME);
                self.pow($mod - 2)
            }

            fn coprime_inv(self) -> Option<Self> {
                assert!($label::MOD_PRIME_FACTOR_LIST.iter().all(|f| self.0 % f != 0));
                let (x, _, g) = $crate::extended_gcd(self.0, $mod);
                if g == 1 {
                    Some(Self(x))
                } else {
                    None
                }
            }

            fn brute_force_inv(self) -> Option<Self> {
                for i in 1..$mod {
                    if self * Self(i) == Self(1) {
                        return Some(Self(i))
                    }
                }
                None
            }

            fn inv(self) -> Option<Self> {
                if *$label::IS_MOD_PRIME {
                    Some(self.prime_inv())
                } else if $label::MOD_PRIME_FACTOR_LIST.iter().all(|f| self.0 % f != 0) {
                    self.coprime_inv()
                } else {
                    self.brute_force_inv()
                }
            }
        }

        /// Converts a number of the base type into this modular type.
        ///
        /// Use the `new` method instead for literal inputs, because the compiler canot determine
        /// if the signed `From` or the unsigned `From` is intended.
        impl From<$int> for $name {
            fn from(int: $int) -> Self {
                Self(int % $mod)
            }
        }

        /// Converts a number of the signed type into this modular type.
        ///
        /// Use the `new` method instead for literal inputs, because the compiler canot determine
        /// if the signed `From` or the unsigned `From` is intended.
        impl From<$sint> for $name {
            fn from(int: $sint) -> Self { Self::new(int) }
        }

        impl ::core::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self { Self((self.0 + rhs.0) % $mod) }
        }

        impl ::core::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                // add $mod to prevent negative integer overflow
                Self(((self.0 + $mod) - rhs.0) % $mod)
            }
        }

        impl ::core::ops::Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self { Self((self.0 * rhs.0) % $mod) }
        }

        /// Multiplies the modular inverse of the right operand.
        ///
        /// # Panics
        /// If modular inverse does not exist for the right operand, the function panicks.
        impl ::core::ops::Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                use $crate::Modular;
                self * rhs.inv().expect("Modular inverse does not exist for rhs")
            }
        }

        impl $crate::Zero for $name {
            fn zero() -> Self { Self(0) }

            fn is_zero(&self) -> bool { self.0 == 0 }
        }

        impl $crate::One for $name {
            fn one() -> Self { Self(1) }
        }
    };
}
