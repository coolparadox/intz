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

use crate::unsigned::uz32;
use crate::unsigned::uz32::Uz32;
use crate::unsigned::Uintz;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn augment0() {
        let _ = uz32::new(0).augment();
    }

}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uz<T: Uintz> {
    hi: T,
    lo: T,
}

pub fn new<T: Uintz>(hi: T, lo: T) -> Uz<T> {
    Uz { hi, lo }
}

impl Uintz for Uz<Uz32> {
    fn zero(&self) -> Self {
        new(self.hi.zero(), self.lo.zero())
    }

    fn augment(self) -> Uz<Self> {
        new(self.zero(), self)
    }

    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let (lo, loc) = self.lo.addc(other.lo, carry);
        let (hi, hic) = self.hi.addc(other.hi, loc);
        (new(hi, lo), hic)
    }
}

/*

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

impl std::ops::Add for Uz32 {
    type Output = Self;

    fn add(self, o: Self) -> Self {
        let (v, c) = self.addc(o, false);
        if c {
            panic!("integer overflow");
        }
        v
    }
}

*/
