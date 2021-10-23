use crate::vector::*;

#[cfg(feature = "morton-encoding")]
use crate::morton::{EncodeMorton, Morton2i32, Morton2u32, Morton3i32, Morton3u32};

use core::cmp::Ordering;
use glam::{
    const_ivec2, const_ivec3, const_uvec2, const_uvec3, const_vec2, const_vec3, const_vec3a, IVec2,
    IVec3, UVec2, UVec3, Vec2, Vec3, Vec3A,
};

macro_rules! impl_lattice_order {
    ($vec:ident, $scalar:ident) => {
        impl LatticeOrder for $vec {
            type LatticeVector = WithLatticeOrd<Self>;
            #[inline]
            fn with_lattice_ord(self) -> Self::LatticeVector {
                WithLatticeOrd(self)
            }
            #[inline]
            fn least_upper_bound(self, other: Self) -> Self {
                self.max(other)
            }
            #[inline]
            fn greatest_lower_bound(self, other: Self) -> Self {
                self.min(other)
            }
        }
        impl Min<$scalar> for $vec {
            #[inline]
            fn min_element(self) -> $scalar {
                self.min_element()
            }
        }
        impl Max<$scalar> for $vec {
            #[inline]
            fn max_element(self) -> $scalar {
                self.max_element()
            }
        }
    };
}

macro_rules! impl_integer_vec2_with_lattice_partial_ord {
    ($vec:ident) => {
        impl PartialOrd for WithLatticeOrd<$vec> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                if self < other {
                    Some(Ordering::Less)
                } else if self > other {
                    Some(Ordering::Greater)
                } else if self.0.x == other.0.x && self.0.y == other.0.y {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            #[inline]
            fn lt(&self, other: &Self) -> bool {
                self.0.x < other.0.x && self.0.y < other.0.y
            }

            #[inline]
            fn gt(&self, other: &Self) -> bool {
                self.0.x > other.0.x && self.0.y > other.0.y
            }

            #[inline]
            fn le(&self, other: &Self) -> bool {
                self.0.x <= other.0.x && self.0.y <= other.0.y
            }

            #[inline]
            fn ge(&self, other: &Self) -> bool {
                self.0.x >= other.0.x && self.0.y >= other.0.y
            }
        }
    };
}

macro_rules! impl_integer_vec3_with_lattice_partial_ord {
    ($vec:ident) => {
        impl PartialOrd for WithLatticeOrd<$vec> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                if self < other {
                    Some(Ordering::Less)
                } else if self > other {
                    Some(Ordering::Greater)
                } else if self.0.x == other.0.x && self.0.y == other.0.y && self.0.z == other.0.z {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            #[inline]
            fn lt(&self, other: &Self) -> bool {
                self.0.x < other.0.x && self.0.y < other.0.y && self.0.z < other.0.z
            }

            #[inline]
            fn gt(&self, other: &Self) -> bool {
                self.0.x > other.0.x && self.0.y > other.0.y && self.0.z > other.0.z
            }

            #[inline]
            fn le(&self, other: &Self) -> bool {
                self.0.x <= other.0.x && self.0.y <= other.0.y && self.0.z <= other.0.z
            }

            #[inline]
            fn ge(&self, other: &Self) -> bool {
                self.0.x >= other.0.x && self.0.y >= other.0.y && self.0.z >= other.0.z
            }
        }
    };
}

macro_rules! impl_float_vec2_with_lattice_partial_ord {
    ($vec:ident) => {
        impl PartialOrd for WithLatticeOrd<$vec> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                if self < other {
                    Some(Ordering::Less)
                } else if self > other {
                    Some(Ordering::Greater)
                } else {
                    None
                }
            }

            #[inline]
            fn lt(&self, other: &Self) -> bool {
                self.0.x < other.0.x && self.0.y < other.0.y
            }

            #[inline]
            fn gt(&self, other: &Self) -> bool {
                self.0.x > other.0.x && self.0.y > other.0.y
            }

            #[inline]
            fn le(&self, other: &Self) -> bool {
                self.0.x <= other.0.x && self.0.y <= other.0.y
            }

            #[inline]
            fn ge(&self, other: &Self) -> bool {
                self.0.x >= other.0.x && self.0.y >= other.0.y
            }
        }
    };
}

macro_rules! impl_float_vec3_with_lattice_partial_ord {
    ($vec:ident) => {
        impl PartialOrd for WithLatticeOrd<$vec> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                if self < other {
                    Some(Ordering::Less)
                } else if self > other {
                    Some(Ordering::Greater)
                } else {
                    None
                }
            }

            #[inline]
            fn lt(&self, other: &Self) -> bool {
                self.0.x < other.0.x && self.0.y < other.0.y && self.0.z < other.0.z
            }

            #[inline]
            fn gt(&self, other: &Self) -> bool {
                self.0.x > other.0.x && self.0.y > other.0.y && self.0.z > other.0.z
            }

            #[inline]
            fn le(&self, other: &Self) -> bool {
                self.0.x <= other.0.x && self.0.y <= other.0.y && self.0.z <= other.0.z
            }

            #[inline]
            fn ge(&self, other: &Self) -> bool {
                self.0.x >= other.0.x && self.0.y >= other.0.y && self.0.z >= other.0.z
            }
        }
    };
}

macro_rules! impl_signed_shift_ops {
    ($vec:ident, $scalar:ident, $uvec:ident) => {
        impl AllShiftOps<$scalar> for $vec {
            type UintVec = $uvec;
        }
        impl ShiftOps<u8> for $vec {}
        impl ShiftOps<u16> for $vec {}
        impl ShiftOps<u32> for $vec {}
        impl ShiftOps<$scalar> for $vec {}
        impl ShiftOps<$vec> for $vec {}
        impl ShiftOps<$uvec> for $vec {}
    };
}

macro_rules! impl_unsigned_shift_ops {
    ($vec:ident, $scalar:ident) => {
        impl AllShiftOps<$scalar> for $vec {
            type UintVec = $vec;
        }
        impl ShiftOps<u8> for $vec {}
        impl ShiftOps<u16> for $vec {}
        impl ShiftOps<u32> for $vec {}
        impl ShiftOps<$vec> for $vec {}
    };
}

macro_rules! impl_signed_vector {
    ($vec:ident) => {
        impl Abs for $vec {
            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }
        }
    };
}

macro_rules! impl_integer_vector {
    ($vec:ident, $dim:literal, $scalar:ident, $uvec:ident, $ones:expr) => {
        impl IntegerVector for $vec {
            type IntScalar = $scalar;
        }
        impl Vector for $vec {
            type Scalar = $scalar;
        }
        impl VectorArithmetic<$scalar> for $vec {}
        impl ScalarBitwiseLogic<$scalar> for $vec {}
        impl VectorBitwiseLogic for $vec {}

        impl Splat<$scalar> for $vec {
            #[inline]
            fn splat(value: $scalar) -> Self {
                Self::splat(value)
            }
        }
        impl Zero for $vec {
            const ZERO: Self = Self::ZERO;
        }
        impl Ones for $vec {
            const ONES: Self = $ones;
        }
    };
}

macro_rules! impl_float_vector {
    ($vec:ident, $scalar:ident, $ivec:ident, $ones:expr) => {
        impl Vector for $vec {
            type Scalar = $scalar;
        }
        impl VectorArithmetic<$scalar> for $vec {}
        impl Splat<$scalar> for $vec {
            #[inline]
            fn splat(value: $scalar) -> Self {
                Self::splat(value)
            }
        }
        impl Zero for $vec {
            const ZERO: Self = $vec::ZERO;
        }
        impl Ones for $vec {
            const ONES: Self = $ones;
        }
        impl RoundingOps for $vec {
            #[inline]
            fn floor(self) -> Self {
                self.floor()
            }
            #[inline]
            fn ceil(self) -> Self {
                self.ceil()
            }
        }
    };
}

macro_rules! impl_float_vec2 {
    ($vec:ident, $ivec:ident, $iscalar:ident) => {
        impl CastInteger for $vec {
            type Int = $ivec;

            #[inline]
            fn cast_int(self) -> Self::Int {
                Self::Int::new(self.x as $iscalar, self.y as $iscalar)
            }
        }
    };
}

macro_rules! impl_float_vec3 {
    ($vec:ident, $ivec:ident, $iscalar:ident) => {
        impl CastInteger for $vec {
            type Int = $ivec;

            #[inline]
            fn cast_int(self) -> Self::Int {
                Self::Int::new(self.x as $iscalar, self.y as $iscalar, self.z as $iscalar)
            }
        }
    };
}

macro_rules! impl_vec2 {
    ($vec:ident, $scalar:ident) => {
        impl Vector2 for $vec {
            #[inline]
            fn x(self) -> Self::Scalar {
                self.x
            }
            #[inline]
            fn y(self) -> Self::Scalar {
                self.y
            }
            #[inline]
            fn x_mut(&mut self) -> &mut Self::Scalar {
                &mut self.x
            }
            #[inline]
            fn y_mut(&mut self) -> &mut Self::Scalar {
                &mut self.y
            }
        }
        impl Fold<$scalar> for $vec {
            #[inline]
            fn fold<T>(self, init: T, f: impl Fn(<Self as Vector>::Scalar, T) -> T) -> T {
                let mut out = init;
                out = f(self.x, out);
                out = f(self.y, out);
                out
            }
        }
        impl Map<$scalar> for $vec {
            /// Applies `f` to all components, returning the results as `Self`.
            #[inline]
            fn map(self, f: impl Fn($scalar) -> $scalar) -> Self {
                Self::new(f(self.x), f(self.y))
            }
        }
        impl ZipMap<$scalar> for $vec {
            /// Zips the components of `self` and `other`, applying `f`, and returning the results as `Self`.
            #[inline]
            fn zip_map(self, other: Self, f: impl Fn($scalar, $scalar) -> $scalar) -> Self {
                Self::new(f(self.x, other.x), f(self.y, other.y))
            }
        }
    };
}

macro_rules! impl_vec3 {
    ($vec:ident, $scalar:ident) => {
        impl Vector3 for $vec {
            #[inline]
            fn x(self) -> Self::Scalar {
                self.x
            }
            #[inline]
            fn y(self) -> Self::Scalar {
                self.y
            }
            #[inline]
            fn z(self) -> Self::Scalar {
                self.z
            }
            #[inline]
            fn x_mut(&mut self) -> &mut Self::Scalar {
                &mut self.x
            }
            #[inline]
            fn y_mut(&mut self) -> &mut Self::Scalar {
                &mut self.y
            }
            #[inline]
            fn z_mut(&mut self) -> &mut Self::Scalar {
                &mut self.z
            }
        }
        impl Fold<$scalar> for $vec {
            #[inline]
            fn fold<T>(self, init: T, f: impl Fn(<Self as Vector>::Scalar, T) -> T) -> T {
                let mut out = init;
                out = f(self.x, out);
                out = f(self.y, out);
                out = f(self.z, out);
                out
            }
        }
        impl Map<$scalar> for $vec {
            /// Applies `f` to all components, returning the results as `Self`.
            #[inline]
            fn map(self, f: impl Fn($scalar) -> $scalar) -> Self {
                Self::new(f(self.x), f(self.y), f(self.z))
            }
        }
        impl ZipMap<$scalar> for $vec {
            /// Zips the components of `self` and `other`, applying `f`, and returning the results as `Self`.
            #[inline]
            fn zip_map(self, other: Self, f: impl Fn($scalar, $scalar) -> $scalar) -> Self {
                Self::new(f(self.x, other.x), f(self.y, other.y), f(self.z, other.z))
            }
        }
    };
}

// IVec2
impl_vec2!(IVec2, i32);
impl_integer_vector!(IVec2, 2, i32, UVec2, const_ivec2!([1; 2]));
impl_signed_vector!(IVec2);
impl_signed_shift_ops!(IVec2, i32, UVec2);
impl_integer_vec2_with_lattice_partial_ord!(IVec2);
impl_lattice_order!(IVec2, i32);
impl Bounded for IVec2 {
    const MIN: Self = const_ivec2!([i32::MIN; 2]);
    const MAX: Self = const_ivec2!([i32::MAX; 2]);
}

// IVec3
impl_vec3!(IVec3, i32);
impl_integer_vector!(IVec3, 3, i32, UVec3, const_ivec3!([1; 3]));
impl_signed_vector!(IVec3);
impl_signed_shift_ops!(IVec3, i32, UVec3);
impl_integer_vec3_with_lattice_partial_ord!(IVec3);
impl_lattice_order!(IVec3, i32);
impl Bounded for IVec3 {
    const MIN: Self = const_ivec3!([i32::MIN; 3]);
    const MAX: Self = const_ivec3!([i32::MAX; 3]);
}

// UVec2
impl_vec2!(UVec2, u32);
impl_integer_vector!(UVec2, 2, u32, UVec2, const_uvec2!([1; 2]));
impl_unsigned_shift_ops!(UVec2, u32);
impl_integer_vec2_with_lattice_partial_ord!(UVec2);
impl_lattice_order!(UVec2, u32);
impl Bounded for UVec2 {
    const MIN: Self = const_uvec2!([u32::MIN; 2]);
    const MAX: Self = const_uvec2!([u32::MAX; 2]);
}

// UVec3
impl_vec3!(UVec3, u32);
impl_integer_vector!(UVec3, 3, u32, UVec3, const_uvec3!([1; 3]));
impl_unsigned_shift_ops!(UVec3, u32);
impl_integer_vec3_with_lattice_partial_ord!(UVec3);
impl_lattice_order!(UVec3, u32);
impl Bounded for UVec3 {
    const MIN: Self = const_uvec3!([u32::MIN; 3]);
    const MAX: Self = const_uvec3!([u32::MAX; 3]);
}

// Vec2
impl_vec2!(Vec2, f32);
impl_float_vector!(Vec2, f32, IVec2, const_vec2!([1.0; 2]));
impl_float_vec2!(Vec2, IVec2, i32);
impl_signed_vector!(Vec2);
impl_float_vec2_with_lattice_partial_ord!(Vec2);
impl_lattice_order!(Vec2, f32);
impl Bounded for Vec2 {
    const MIN: Self = const_vec2!([f32::MIN; 2]);
    const MAX: Self = const_vec2!([f32::MAX; 2]);
}

// Vec3
impl_vec3!(Vec3, f32);
impl_float_vector!(Vec3, f32, IVec3, const_vec3!([1.0; 3]));
impl_float_vec3!(Vec3, IVec3, i32);
impl_signed_vector!(Vec3);
impl_float_vec3_with_lattice_partial_ord!(Vec3);
impl_lattice_order!(Vec3, f32);
impl Bounded for Vec3 {
    const MIN: Self = const_vec3!([f32::MIN; 3]);
    const MAX: Self = const_vec3!([f32::MAX; 3]);
}

// Vec3A
impl_vec3!(Vec3A, f32);
impl_float_vector!(Vec3A, f32, IVec3, const_vec3a!([1.0; 3]));
impl_float_vec3!(Vec3A, IVec3, i32);
impl_signed_vector!(Vec3A);
impl_float_vec3_with_lattice_partial_ord!(Vec3A);
impl_lattice_order!(Vec3A, f32);
impl Bounded for Vec3A {
    const MIN: Self = const_vec3a!([f32::MIN; 3]);
    const MAX: Self = const_vec3a!([f32::MAX; 3]);
}

#[cfg(feature = "morton-encoding")]
mod impl_morton {
    use super::*;

    macro_rules! impl_encode_morton {
        ($vec:ident, $dim:literal, $scalar:ty, $morton:ident) => {
            impl EncodeMorton for $vec {
                type Morton = $morton;
            }
            impl From<$morton> for $vec {
                #[inline]
                fn from(m: $morton) -> Self {
                    Self::from(<[$scalar; $dim]>::from(m))
                }
            }
            impl From<$vec> for $morton {
                #[inline]
                fn from(v: $vec) -> $morton {
                    $morton::from(v.to_array())
                }
            }
        };
    }

    impl_encode_morton!(IVec2, 2, i32, Morton2i32);
    impl_encode_morton!(IVec3, 3, i32, Morton3i32);
    impl_encode_morton!(UVec2, 2, u32, Morton2u32);
    impl_encode_morton!(UVec3, 3, u32, Morton3u32);
}
