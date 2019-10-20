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

pub mod unsigned;

/*
pub trait Uintz {
    fn is_zero(&self) -> bool;
    fn addc(self, other: Self, carry: bool) -> (Self, bool)
    where
        Self: std::marker::Sized;
    fn subb(self, other: Self, borrow: bool) -> (Self, bool)
    where
        Self: std::marker::Sized;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uz<T: Uintz> {
    hi: T,
    lo: T,
}

impl Uintz for Uz<Uz32> {
    fn is_zero(&self) -> bool {
        self.lo.is_zero() && self.hi.is_zero()
    }
    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let (lo, loc) = self.lo.addc(other.lo, carry);
        let (hi, hic) = self.hi.addc(other.hi, loc);
        (Self { lo, hi }, hic)
    }
    fn subb(self, other: Self, borrow: bool) -> (Self, bool) {
        let (lo, lob) = self.lo.subb(other.lo, borrow);
        let (hi, hib) = self.hi.subb(other.hi, lob);
        (Self { lo, hi }, hib)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uz32 {
    v: u32,
}

impl Uintz for Uz32 {
    fn is_zero(&self) -> bool {
        self.v == 0
    }
    fn addc(self, other: Self, carry: bool) -> (Self, bool) {
        let nv: u64 = self.v as u64 + other.v as u64 + if carry { 1 } else { 0 };
        (
            Self {
                v: (nv % 0x100000000u64) as u32,
            },
            nv / 0x100000000u64 != 0,
        )
    }
    fn subb(self, other: Self, borrow: bool) -> (Self, bool) {
        let v = self.v as u64;
        let o = other.v as u64 + if borrow { 1 } else { 0 };
        let nb = o > v;
        let nv = if nb { v + 0x100000000u64 } else { v } - o;
        (Self { v: nv as u32 }, nb)
    }
}

pub trait Intz {
    fn neg(self) -> Self;
    fn add(self, other: Self) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn sub(self, other: Self) -> Option<Self>
    where
        Self: std::marker::Sized;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Iz<T: Uintz> {
    p: bool,
    u: T,
}

impl Intz for Iz<Uz32> {
    fn neg(self) -> Self {
        if self.u.is_zero() {
            Self { p: true, u: self.u }
        } else {
            Self {
                p: !self.p,
                u: self.u,
            }
        }
    }
    fn add(self, other: Self) -> Option<Self> {
        if self.p == other.p {
            let (nu, c) = self.u.addc(other.u, false);
            if c {
                None
            } else {
                Some(Self { p: self.p, u: nu })
            }
        } else {
            if self.u >= other.u {
                let (nu, _) = self.u.subb(other.u, false);
                Some(Self {
                    p: nu.is_zero() || self.p,
                    u: nu,
                })
            } else {
                let (nu, _) = other.u.subb(self.u, false);
                Some(Self { p: other.p, u: nu })
            }
        }
    }
    fn sub(self, other: Self) -> Option<Self> {
        self.add(other.neg())
    }
}

impl Intz for Iz<Uz<Uz32>> {
    fn neg(self) -> Self {
        if self.u.is_zero() {
            Self { p: true, u: self.u }
        } else {
            Self {
                p: !self.p,
                u: self.u,
            }
        }
    }
    fn add(self, other: Self) -> Option<Self> {
        if self.p == other.p {
            let (nu, c) = self.u.addc(other.u, false);
            if c {
                None
            } else {
                Some(Self { p: self.p, u: nu })
            }
        } else {
            if self.u >= other.u {
                let (nu, _) = self.u.subb(other.u, false);
                Some(Self {
                    p: nu.is_zero() || self.p,
                    u: nu,
                })
            } else {
                let (nu, _) = other.u.subb(self.u, false);
                Some(Self { p: other.p, u: nu })
            }
        }
    }
    fn sub(self, other: Self) -> Option<Self> {
        self.add(other.neg())
    }
}
*/
