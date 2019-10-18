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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uz<T: Uintz> {
    hi: T,
    lo: T,
}

pub trait Uintz {
    fn addc(self, other: Self, carry: bool) -> (Self, bool) where Self: std::marker::Sized;
}

impl Uintz for Uz<Uz32> {
    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let (lo, loc) = self.lo.addc(other.lo, carry);
        let (hi, hic) = self.hi.addc(other.hi, loc);
        (Self { lo, hi, }, hic)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uz32 {
    v: u32,
}

impl Uintz for Uz32 {
    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let v: u64 = self.v as u64 + other.v as u64 + if carry { 1 } else { 0 };
        (Self { v:(v % 2^32) as u32 }, v / 2^32 != 0)
    }
}
