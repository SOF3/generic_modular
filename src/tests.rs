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

crate::def_modular!(Mod101 : u16 | i16, 101 ; lbl);

#[test]
fn test_identical() {
    assert_eq!(Mod101::new(2), Mod101::new(2));
}

#[test]
fn test_eq_down() {
    assert_eq!(Mod101::new(2 + 101 + 101), Mod101::new(2));
}

#[test]
fn test_eq_up() {
    assert_eq!(Mod101::new(2 - 101 - 101), Mod101::new(2));
}

#[test]
fn test_normal_add() {
    assert_eq!(Mod101::new(2) + Mod101::new(3), Mod101::new(5));
}

#[test]
fn test_wrapping_add() {
    assert_eq!(Mod101::new(92) + Mod101::new(13), Mod101::new(4));
}

#[test]
fn test_normal_sub() {
    assert_eq!(Mod101::new(9) - Mod101::new(4), Mod101::new(5));
}

#[test]
fn test_wrapping_sub() {
    assert_eq!(Mod101::new(5) - Mod101::new(9), Mod101::new(97));
}

#[test]
fn test_normal_mul() {
    assert_eq!(Mod101::new(10) * Mod101::new(10), Mod101::new(100));
}

#[test]
fn test_wrapping_mul() {
    assert_eq!(Mod101::new(100) * Mod101::new(100), Mod101::new(10000));
}
