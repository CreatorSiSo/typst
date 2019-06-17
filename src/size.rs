//! General spacing types.

use std::cmp::Ordering;
use std::fmt::{self, Display, Debug, Formatter};
use std::iter::Sum;
use std::ops::*;


/// A general spacing type.
#[derive(Copy, Clone, PartialEq, Default)]
pub struct Size {
    /// The size in typographic points (1/72 inches).
    points: f32,
}

/// A position or extent in 2-dimensional space.
#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Size2D {
    /// The horizontal coordinate.
    pub x: Size,
    /// The vertical coordinate.
    pub y: Size,
}

/// A size in four directions.
#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct SizeBox {
    /// The left extent.
    pub left: Size,
    /// The top extent.
    pub top: Size,
    /// The right extent.
    pub right: Size,
    /// The bottom extent.
    pub bottom: Size,
}

impl Size {
    /// Create a zeroed size.
    #[inline]
    pub fn zero() -> Size { Size { points: 0.0 } }

    /// Create a size from an amount of points.
    #[inline]
    pub fn from_points(points: f32) -> Size { Size { points } }

    /// Create a size from an amount of inches.
    #[inline]
    pub fn from_inches(inches: f32) -> Size { Size { points: 72.0 * inches } }

    /// Create a size from an amount of millimeters.
    #[inline]
    pub fn from_mm(mm: f32) -> Size { Size { points: 2.83465 * mm  } }

    /// Create a size from an amount of centimeters.
    #[inline]
    pub fn from_cm(cm: f32) -> Size { Size { points: 28.3465 * cm } }

    /// Convert this size into points.
    #[inline]
    pub fn to_points(&self) -> f32 { self.points }

    /// Convert this size into inches.
    #[inline]
    pub fn to_inches(&self) -> f32 { self.points * 0.0138889 }

    /// Convert this size into millimeters.
    #[inline]
    pub fn to_mm(&self) -> f32 { self.points * 0.352778 }

    /// Convert this size into centimeters.
    #[inline]
    pub fn to_cm(&self) -> f32 { self.points * 0.0352778 }
}

impl Size2D {
    /// Create a new 2D vector from two sizes.
    #[inline]
    pub fn new(x: Size, y: Size) -> Size2D { Size2D { x, y } }

    /// Create a zeroed vector.
    #[inline]
    pub fn zero() -> Size2D { Size2D { x: Size::zero(), y: Size::zero() } }
}

impl SizeBox {
    /// Create a new box from four sizes.
    #[inline]
    pub fn new(left: Size, top: Size, right: Size, bottom: Size) -> SizeBox {
        SizeBox { left, top, right, bottom }
    }

    /// Create a zeroed vector.
    #[inline]
    pub fn zero() -> SizeBox {
        SizeBox {
            left: Size::zero(),
            top: Size::zero(),
            right: Size::zero(),
            bottom: Size::zero(),
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}pt", self.points)
    }
}

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl PartialOrd for Size {
    #[inline]
    fn partial_cmp(&self, other: &Size) -> Option<Ordering> {
        self.points.partial_cmp(&other.points)
    }
}

impl Neg for Size {
    type Output = Size;

    #[inline]
    fn neg(self) -> Size {
        Size { points: -self.points }
    }
}

impl Sum for Size {
    #[inline]
    fn sum<I>(iter: I) -> Size where I: Iterator<Item=Size> {
        iter.fold(Size::zero(), Add::add)
    }
}

macro_rules! impl_reflexive {
    ($trait:ident, $func:ident, $assign_trait:ident, $assign_func:ident) => {
        impl $trait for Size {
            type Output = Size;

            #[inline]
            fn $func(self, other: Size) -> Size {
                Size { points: $trait::$func(self.points, other.points) }
            }
        }

        impl $assign_trait for Size {
            #[inline]
            fn $assign_func(&mut self, other: Size) {
                $assign_trait::$assign_func(&mut self.points, other.points);
            }
        }
    };
}

macro_rules! impl_num_back {
    ($trait:ident, $func:ident, $assign_trait:ident, $assign_func:ident, $ty:ty) => {
        impl $trait<$ty> for Size {
            type Output = Size;

            #[inline]
            fn $func(self, other: $ty) -> Size {
                Size { points: $trait::$func(self.points, other as f32) }
            }
        }

        impl $assign_trait<$ty> for Size {
            #[inline]
            fn $assign_func(&mut self, other: $ty) {
                $assign_trait::$assign_func(&mut self.points, other as f32);
            }
        }
    };
}

macro_rules! impl_num_both {
    ($trait:ident, $func:ident, $assign_trait:ident, $assign_func:ident, $ty:ty) => {
        impl_num_back!($trait, $func, $assign_trait, $assign_func, $ty);

        impl $trait<Size> for $ty {
            type Output = Size;

            #[inline]
            fn $func(self, other: Size) -> Size {
                Size { points: $trait::$func(self as f32, other.points) }
            }
        }
    };
}

impl_reflexive!(Add, add, AddAssign, add_assign);
impl_reflexive!(Sub, sub, SubAssign, sub_assign);
impl_num_both!(Mul, mul, MulAssign, mul_assign, f32);
impl_num_both!(Mul, mul, MulAssign, mul_assign, i32);
impl_num_back!(Div, div, DivAssign, div_assign, f32);
impl_num_back!(Div, div, DivAssign, div_assign, i32);

impl Display for Size2D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Debug for Size2D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Neg for Size2D {
    type Output = Size2D;

    #[inline]
    fn neg(self) -> Size2D {
        Size2D { x: -self.x, y: -self.y }
    }
}

macro_rules! impl_reflexive2d {
    ($trait:ident, $func:ident, $assign_trait:ident, $assign_func:ident) => {
        impl $trait for Size2D {
            type Output = Size2D;

            #[inline]
            fn $func(self, other: Size2D) -> Size2D {
                Size2D {
                    x: $trait::$func(self.x, other.x),
                    y: $trait::$func(self.y, other.y),
                }
            }
        }

        impl $assign_trait for Size2D {
            #[inline]
            fn $assign_func(&mut self, other: Size2D) {
                $assign_trait::$assign_func(&mut self.x, other.x);
                $assign_trait::$assign_func(&mut self.y, other.y);
            }
        }
    };
}

macro_rules! impl_num_back2d {
    ($trait:ident, $func:ident, $assign_trait:ident, $assign_func:ident, $ty:ty) => {
        impl $trait<$ty> for Size2D {
            type Output = Size2D;

            #[inline]
            fn $func(self, other: $ty) -> Size2D {
                Size2D {
                    x: $trait::$func(self.x, other as f32),
                    y: $trait::$func(self.y, other as f32),
                }
            }
        }

        impl $assign_trait<$ty> for Size2D {
            #[inline]
            fn $assign_func(&mut self, other: $ty) {
                $assign_trait::$assign_func(&mut self.x, other as f32);
                $assign_trait::$assign_func(&mut self.y, other as f32);
            }
        }
    };
}

macro_rules! impl_num_both2d {
    ($trait:ident, $func:ident, $assign_trait:ident, $assign_func:ident, $ty:ty) => {
        impl_num_back2d!($trait, $func, $assign_trait, $assign_func, $ty);

        impl $trait<Size2D> for $ty {
            type Output = Size2D;

            #[inline]
            fn $func(self, other: Size2D) -> Size2D {
                Size2D {
                    x: $trait::$func(self as f32, other.x),
                    y: $trait::$func(self as f32, other.y),
                }
            }
        }
    };
}

impl_reflexive2d!(Add, add, AddAssign, add_assign);
impl_reflexive2d!(Sub, sub, SubAssign, sub_assign);
impl_num_both2d!(Mul, mul, MulAssign, mul_assign, f32);
impl_num_both2d!(Mul, mul, MulAssign, mul_assign, i32);
impl_num_back2d!(Div, div, DivAssign, div_assign, f32);
impl_num_back2d!(Div, div, DivAssign, div_assign, i32);

impl Display for SizeBox {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[left: {}, top: {}, right: {}, bottom: {}]",
            self.left, self.top, self.right, self.bottom)
    }
}

impl Debug for SizeBox {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}