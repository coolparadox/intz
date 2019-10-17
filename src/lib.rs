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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// FIXME: replace by i32
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Iz32 {
    v: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Iz<T: Intz> {
    hi: T,
    lo: T,
}

pub trait Intz {
    fn depth(&self) -> u32;
    fn zero(&self) -> Self;
    fn augment(self) -> Iz<Self>
    where
        Self: std::marker::Sized;
}

impl Intz for Iz32 {
    fn depth(&self) -> u32 {
        0
    }
    fn zero(&self) -> Self {
        Self { v:0, }
    }
    fn augment(self) -> Iz<Self> {
        Iz {
            hi: self.zero(),
            lo: self,
        }
    }
}

impl Intz for Iz<Iz32> {
    fn depth(&self) -> u32 {
        0
    }
    fn zero(&self) -> Self {
        Self {
            hi: self.hi.zero(),
            lo: self.lo.zero(),
        }
    }
    fn augment(self) -> Iz<Self> {
        Iz {
            hi: self.zero(),
            lo: self,
        }
    }
}

impl std::ops::Add for Iz32 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { v:0, }
    }
}

impl std::ops::Add<i32> for Iz32 {
    type Output = Self;
    fn add(self, other: i32) -> Self {
        Self { v:0, }
    }
}

impl std::ops::AddAssign for Iz32 {
    fn add_assign(&mut self, other: Self) {}
}

impl std::ops::AddAssign<i32> for Iz32 {
    fn add_assign(&mut self, other: i32) {}
}

impl std::ops::Sub for Iz32 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { v:0, }
    }
}

impl std::ops::Sub<i32> for Iz32 {
    type Output = Self;
    fn sub(self, other: i32) -> Self {
        Self { v:0, }
    }
}

impl std::ops::SubAssign for Iz32 {
    fn sub_assign(&mut self, other: Self) {}
}

impl std::ops::SubAssign<i32> for Iz32 {
    fn sub_assign(&mut self, other: i32) {}
}


impl std::ops::Mul for Iz32 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self { v:0, }
    }
}

impl std::ops::Mul<i32> for Iz32 {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self { v:0, }
    }
}

impl std::ops::MulAssign for Iz32 {
    fn mul_assign(&mut self, other: Self) {}
}

impl std::ops::MulAssign<i32> for Iz32 {
    fn mul_assign(&mut self, other: i32) {}
}

impl std::ops::Neg for Iz32 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { v:0, }
    }
}

impl std::ops::Div<i32> for Iz32 {
    type Output = Self;
    fn div(self, other: i32) -> Self {
        Self { v:0, }
    }
}

impl std::ops::Rem<i32> for Iz32 {
    type Output = Self;
    fn rem(self, other: i32) -> Self {
        Self { v:0, }
    }
}
