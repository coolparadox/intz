/*
 * Copyright 2019 Rafael Lorandi <coolparadox@gmail.com>
 *
 * This file is part of intz, a big integer library for the rust
 * language.
 *
 * intz is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * intz is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with intz.  If not, see <http://www.gnu.org/licenses/>
 */

use crate::uz::Uintz;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eq0() {
        assert_eq!(new(0), new(0));
    }

    #[test]
    fn eq1() {
        assert_eq!(new(u32::max_value()), new(u32::max_value()));
    }

    #[test]
    fn eq2() {
        assert_ne!(new(0), new(u32::max_value()));
    }

//     #[test]
//     fn add0() {
//         assert_eq!(new(0) + new(0), new(0));
//     }

//     #[test]
//     #[should_panic(expected = "division by zero")]
//     fn forbids_undefined_ratio() {
//         new(true, 0, 0);
//     }

}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uz32 {
    v: u32,
}

pub fn new(v: u32) -> Uz32 {
    Uz32 { v }
}

impl Uintz for Uz32 {
    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let nv: u64 = self.v as u64 + other.v as u64 + if carry { 1 } else { 0 };
        (
            Self {
                v: (nv % 0x100000000u64) as u32,
            },
            nv / 0x100000000u64 != 0,
        )
    }
}