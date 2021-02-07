// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Defines the logical data types of Arrow arrays.
//!
//! The most important things you might be looking for are:
//!  * [`Schema`](crate::datatypes::Schema) to describe a schema.
//!  * [`Field`](crate::datatypes::Field) to describe one field within a schema.
//!  * [`DataType`](crate::datatypes::DataType) to describe the type of a field.

use std::default::Default;
use std::fmt;
use std::mem::size_of;
use std::ops::Neg;
#[cfg(feature = "simd")]
use std::ops::{Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, Div, Mul, Not, Sub};
use std::slice::from_raw_parts;
use std::str::FromStr;
use std::sync::Arc;

#[cfg(feature = "simd")]
use packed_simd::*;
use serde_json::{Number, Value, Value::Number as VNumber};

mod field;
pub use field::Field;

mod schema;
pub use schema::Schema;

pub type SchemaRef = Arc<Schema>;

mod datatype;
pub use datatype::{DataType, IntervalUnit, TimeUnit};

/// Trait declaring any type that is serializable to JSON. This includes all primitive types (bool, i32, etc.).
pub trait JsonSerializable: 'static {
    fn into_json_value(self) -> Option<Value>;
}

/// Trait expressing a Rust type that has the same in-memory representation
/// as Arrow. This includes `i16`, `f32`, but excludes `bool` (which in arrow is represented in bits).
/// In little endian machines, types that implement [`ArrowNativeType`] can be memcopied to arrow buffers
/// as is.
pub trait ArrowNativeType:
    fmt::Debug + Send + Sync + Copy + PartialOrd + FromStr + Default + JsonSerializable
{
    /// Convert native type from usize.
    fn from_usize(_: usize) -> Option<Self> {
        None
    }

    /// Convert native type to usize.
    fn to_usize(&self) -> Option<usize> {
        None
    }

    /// Convert native type from i32.
    fn from_i32(_: i32) -> Option<Self> {
        None
    }

    /// Convert native type from i64.
    fn from_i64(_: i64) -> Option<Self> {
        None
    }
}

/// Trait bridging the dynamic-typed nature of Arrow (via [`DataType`]) with the
/// static-typed nature of rust types ([`ArrowNativeType`]) for all types that implement [`ArrowNativeType`].
pub trait ArrowPrimitiveType: 'static {
    /// Corresponding Rust native type for the primitive type.
    type Native: ArrowNativeType;

    /// the corresponding Arrow data type of this primitive type.
    const DATA_TYPE: DataType;

    /// Returns the byte width of this primitive type.
    fn get_byte_width() -> usize {
        size_of::<Self::Native>()
    }

    /// Returns a default value of this primitive type.
    ///
    /// This is useful for aggregate array ops like `sum()`, `mean()`.
    fn default_value() -> Self::Native {
        Default::default()
    }
}

impl JsonSerializable for bool {
    fn into_json_value(self) -> Option<Value> {
        Some(self.into())
    }
}

impl JsonSerializable for i8 {
    fn into_json_value(self) -> Option<Value> {
        Some(self.into())
    }
}

impl ArrowNativeType for i8 {
    fn from_usize(v: usize) -> Option<Self> {
        num::FromPrimitive::from_usize(v)
    }

    fn to_usize(&self) -> Option<usize> {
        num::ToPrimitive::to_usize(self)
    }
}

impl JsonSerializable for i16 {
    fn into_json_value(self) -> Option<Value> {
        Some(self.into())
    }
}

impl ArrowNativeType for i16 {
    fn from_usize(v: usize) -> Option<Self> {
        num::FromPrimitive::from_usize(v)
    }

    fn to_usize(&self) -> Option<usize> {
        num::ToPrimitive::to_usize(self)
    }
}

impl JsonSerializable for i32 {
    fn into_json_value(self) -> Option<Value> {
        Some(self.into())
    }
}

impl ArrowNativeType for i32 {
    fn from_usize(v: usize) -> Option<Self> {
        num::FromPrimitive::from_usize(v)
    }

    fn to_usize(&self) -> Option<usize> {
        num::ToPrimitive::to_usize(self)
    }

    /// Convert native type from i32.
    fn from_i32(val: i32) -> Option<Self> {
        Some(val)
    }
}

impl JsonSerializable for i64 {
    fn into_json_value(self) -> Option<Value> {
        Some(VNumber(Number::from(self)))
    }
}

impl ArrowNativeType for i64 {
    fn from_usize(v: usize) -> Option<Self> {
        num::FromPrimitive::from_usize(v)
    }

    fn to_usize(&self) -> Option<usize> {
        num::ToPrimitive::to_usize(self)
    }

    /// Convert native type from i64.
    fn from_i64(val: i64) -> Option<Self> {
        Some(val)
    }
}

impl JsonSerializable for u8 {
    fn into_json_value(self) -> Option<Value> {
        Some(self.into())
    }
}

impl ArrowNativeType for u8 {
    fn from_usize(v: usize) -> Option<Self> {
        num::FromPrimitive::from_usize(v)
    }

    fn to_usize(&self) -> Option<usize> {
        num::ToPrimitive::to_usize(self)
    }
}

impl JsonSerializable for u16 {
    fn into_json_value(self) -> Option<Value> {
        Some(self.into())
    }
}

impl ArrowNativeType for u16 {
    fn from_usize(v: usize) -> Option<Self> {
        num::FromPrimitive::from_usize(v)
    }

    fn to_usize(&self) -> Option<usize> {
        num::ToPrimitive::to_usize(self)
    }
}

impl JsonSerializable for u32 {
    fn into_json_value(self) -> Option<Value> {
        Some(self.into())
    }
}

impl ArrowNativeType for u32 {
    fn from_usize(v: usize) -> Option<Self> {
        num::FromPrimitive::from_usize(v)
    }

    fn to_usize(&self) -> Option<usize> {
        num::ToPrimitive::to_usize(self)
    }
}

impl JsonSerializable for u64 {
    fn into_json_value(self) -> Option<Value> {
        Some(self.into())
    }
}

impl ArrowNativeType for u64 {
    fn from_usize(v: usize) -> Option<Self> {
        num::FromPrimitive::from_usize(v)
    }

    fn to_usize(&self) -> Option<usize> {
        num::ToPrimitive::to_usize(self)
    }
}

impl JsonSerializable for f32 {
    fn into_json_value(self) -> Option<Value> {
        Number::from_f64(f64::round(self as f64 * 1000.0) / 1000.0).map(VNumber)
    }
}

impl JsonSerializable for f64 {
    fn into_json_value(self) -> Option<Value> {
        Number::from_f64(self).map(VNumber)
    }
}

impl ArrowNativeType for f32 {}
impl ArrowNativeType for f64 {}

// BooleanType is special: its bit-width is not the size of the primitive type, and its `index`
// operation assumes bit-packing.
#[derive(Debug)]
pub struct BooleanType {}

impl BooleanType {
    pub const DATA_TYPE: DataType = DataType::Boolean;
}

macro_rules! make_type {
    ($name:ident, $native_ty:ty, $data_ty:expr) => {
        #[derive(Debug)]
        pub struct $name {}

        impl ArrowPrimitiveType for $name {
            type Native = $native_ty;
            const DATA_TYPE: DataType = $data_ty;
        }
    };
}

make_type!(Int8Type, i8, DataType::Int8);
make_type!(Int16Type, i16, DataType::Int16);
make_type!(Int32Type, i32, DataType::Int32);
make_type!(Int64Type, i64, DataType::Int64);
make_type!(UInt8Type, u8, DataType::UInt8);
make_type!(UInt16Type, u16, DataType::UInt16);
make_type!(UInt32Type, u32, DataType::UInt32);
make_type!(UInt64Type, u64, DataType::UInt64);
make_type!(Float32Type, f32, DataType::Float32);
make_type!(Float64Type, f64, DataType::Float64);
make_type!(
    TimestampSecondType,
    i64,
    DataType::Timestamp(TimeUnit::Second, None)
);
make_type!(
    TimestampMillisecondType,
    i64,
    DataType::Timestamp(TimeUnit::Millisecond, None)
);
make_type!(
    TimestampMicrosecondType,
    i64,
    DataType::Timestamp(TimeUnit::Microsecond, None)
);
make_type!(
    TimestampNanosecondType,
    i64,
    DataType::Timestamp(TimeUnit::Nanosecond, None)
);
make_type!(Date32Type, i32, DataType::Date32);
make_type!(Date64Type, i64, DataType::Date64);
make_type!(Time32SecondType, i32, DataType::Time32(TimeUnit::Second));
make_type!(
    Time32MillisecondType,
    i32,
    DataType::Time32(TimeUnit::Millisecond)
);
make_type!(
    Time64MicrosecondType,
    i64,
    DataType::Time64(TimeUnit::Microsecond)
);
make_type!(
    Time64NanosecondType,
    i64,
    DataType::Time64(TimeUnit::Nanosecond)
);
make_type!(
    IntervalYearMonthType,
    i32,
    DataType::Interval(IntervalUnit::YearMonth)
);
make_type!(
    IntervalDayTimeType,
    i64,
    DataType::Interval(IntervalUnit::DayTime)
);
make_type!(
    DurationSecondType,
    i64,
    DataType::Duration(TimeUnit::Second)
);
make_type!(
    DurationMillisecondType,
    i64,
    DataType::Duration(TimeUnit::Millisecond)
);
make_type!(
    DurationMicrosecondType,
    i64,
    DataType::Duration(TimeUnit::Microsecond)
);
make_type!(
    DurationNanosecondType,
    i64,
    DataType::Duration(TimeUnit::Nanosecond)
);

/// A subtype of primitive type that represents legal dictionary keys.
/// See <https://arrow.apache.org/docs/format/Columnar.html>
pub trait ArrowDictionaryKeyType: ArrowPrimitiveType {}

impl ArrowDictionaryKeyType for Int8Type {}

impl ArrowDictionaryKeyType for Int16Type {}

impl ArrowDictionaryKeyType for Int32Type {}

impl ArrowDictionaryKeyType for Int64Type {}

impl ArrowDictionaryKeyType for UInt8Type {}

impl ArrowDictionaryKeyType for UInt16Type {}

impl ArrowDictionaryKeyType for UInt32Type {}

impl ArrowDictionaryKeyType for UInt64Type {}

/// A subtype of primitive type that represents numeric values.
///
/// SIMD operations are defined in this trait if available on the target system.
#[cfg(simd)]
pub trait ArrowNumericType: ArrowPrimitiveType
where
    Self::Simd: Add<Output = Self::Simd>
        + Sub<Output = Self::Simd>
        + Mul<Output = Self::Simd>
        + Div<Output = Self::Simd>
        + Copy,
    Self::SimdMask: BitAnd<Output = Self::SimdMask>
        + BitOr<Output = Self::SimdMask>
        + BitAndAssign
        + BitOrAssign
        + Not<Output = Self::SimdMask>
        + Copy,
{
    /// Defines the SIMD type that should be used for this numeric type
    type Simd;

    /// Defines the SIMD Mask type that should be used for this numeric type
    type SimdMask;

    /// The number of SIMD lanes available
    fn lanes() -> usize;

    /// Initializes a SIMD register to a constant value
    fn init(value: Self::Native) -> Self::Simd;

    /// Loads a slice into a SIMD register
    fn load(slice: &[Self::Native]) -> Self::Simd;

    /// Creates a new SIMD mask for this SIMD type filling it with `value`
    fn mask_init(value: bool) -> Self::SimdMask;

    /// Creates a new SIMD mask for this SIMD type from the lower-most bits of the given `mask`.
    /// The number of bits used corresponds to the number of lanes of this type
    fn mask_from_u64(mask: u64) -> Self::SimdMask;

    /// Creates a bitmask from the given SIMD mask.
    /// Each bit corresponds to one vector lane, starting with the least-significant bit.
    fn mask_to_u64(mask: &Self::SimdMask) -> u64;

    /// Gets the value of a single lane in a SIMD mask
    fn mask_get(mask: &Self::SimdMask, idx: usize) -> bool;

    /// Sets the value of a single lane of a SIMD mask
    fn mask_set(mask: Self::SimdMask, idx: usize, value: bool) -> Self::SimdMask;

    /// Selects elements of `a` and `b` using `mask`
    fn mask_select(mask: Self::SimdMask, a: Self::Simd, b: Self::Simd) -> Self::Simd;

    /// Returns `true` if any of the lanes in the mask are `true`
    fn mask_any(mask: Self::SimdMask) -> bool;

    /// Performs a SIMD binary operation
    fn bin_op<F: Fn(Self::Simd, Self::Simd) -> Self::Simd>(
        left: Self::Simd,
        right: Self::Simd,
        op: F,
    ) -> Self::Simd;

    /// SIMD version of equal
    fn eq(left: Self::Simd, right: Self::Simd) -> Self::SimdMask;

    /// SIMD version of not equal
    fn ne(left: Self::Simd, right: Self::Simd) -> Self::SimdMask;

    /// SIMD version of less than
    fn lt(left: Self::Simd, right: Self::Simd) -> Self::SimdMask;

    /// SIMD version of less than or equal to
    fn le(left: Self::Simd, right: Self::Simd) -> Self::SimdMask;

    /// SIMD version of greater than
    fn gt(left: Self::Simd, right: Self::Simd) -> Self::SimdMask;

    /// SIMD version of greater than or equal to
    fn ge(left: Self::Simd, right: Self::Simd) -> Self::SimdMask;

    /// Writes a SIMD result back to a slice
    fn write(simd_result: Self::Simd, slice: &mut [Self::Native]);
}

#[cfg(not(simd))]
pub trait ArrowNumericType: ArrowPrimitiveType {}

macro_rules! make_numeric_type {
    ($impl_ty:ty, $native_ty:ty, $simd_ty:ident, $simd_mask_ty:ident) => {
        #[cfg(simd)]
        impl ArrowNumericType for $impl_ty {
            type Simd = $simd_ty;

            type SimdMask = $simd_mask_ty;

            #[inline]
            fn lanes() -> usize {
                Self::Simd::lanes()
            }

            #[inline]
            fn init(value: Self::Native) -> Self::Simd {
                Self::Simd::splat(value)
            }

            #[inline]
            fn load(slice: &[Self::Native]) -> Self::Simd {
                unsafe { Self::Simd::from_slice_unaligned_unchecked(slice) }
            }

            #[inline]
            fn mask_init(value: bool) -> Self::SimdMask {
                Self::SimdMask::splat(value)
            }

            #[inline]
            fn mask_from_u64(mask: u64) -> Self::SimdMask {
                // this match will get removed by the compiler since the number of lanes is known at
                // compile-time for each concrete numeric type
                match Self::lanes() {
                    8 => {
                        // the bit position in each lane indicates the index of that lane
                        let vecidx = i64x8::new(1, 2, 4, 8, 16, 32, 64, 128);

                        // broadcast the lowermost 8 bits of mask to each lane
                        let vecmask = i64x8::splat((mask & 0xFF) as i64);
                        // compute whether the bit corresponding to each lanes index is set
                        let vecmask = (vecidx & vecmask).eq(vecidx);

                        // transmute is necessary because the different match arms return different
                        // mask types, at runtime only one of those expressions will exist per type,
                        // with the type being equal to `SimdMask`.
                        unsafe { std::mem::transmute(vecmask) }
                    }
                    16 => {
                        // same general logic as for 8 lanes, extended to 16 bits
                        let vecidx = i32x16::new(
                            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096,
                            8192, 16384, 32768,
                        );

                        let vecmask = i32x16::splat((mask & 0xFFFF) as i32);
                        let vecmask = (vecidx & vecmask).eq(vecidx);

                        unsafe { std::mem::transmute(vecmask) }
                    }
                    32 => {
                        // compute two separate m32x16 vector masks from  from the lower-most 32 bits of `mask`
                        // and then combine them into one m16x32 vector mask by writing and reading a temporary
                        let tmp = &mut [0_i16; 32];

                        let vecidx = i32x16::new(
                            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096,
                            8192, 16384, 32768,
                        );

                        let vecmask = i32x16::splat((mask & 0xFFFF) as i32);
                        let vecmask = (vecidx & vecmask).eq(vecidx);

                        i16x16::from_cast(vecmask)
                            .write_to_slice_unaligned(&mut tmp[0..16]);

                        let vecmask = i32x16::splat(((mask >> 16) & 0xFFFF) as i32);
                        let vecmask = (vecidx & vecmask).eq(vecidx);

                        i16x16::from_cast(vecmask)
                            .write_to_slice_unaligned(&mut tmp[16..32]);

                        unsafe { std::mem::transmute(i16x32::from_slice_unaligned(tmp)) }
                    }
                    64 => {
                        // compute four m32x16 vector masks from  from all 64 bits of `mask`
                        // and convert them into one m8x64 vector mask by writing and reading a temporary
                        let tmp = &mut [0_i8; 64];

                        let vecidx = i32x16::new(
                            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096,
                            8192, 16384, 32768,
                        );

                        let vecmask = i32x16::splat((mask & 0xFFFF) as i32);
                        let vecmask = (vecidx & vecmask).eq(vecidx);

                        i8x16::from_cast(vecmask)
                            .write_to_slice_unaligned(&mut tmp[0..16]);

                        let vecmask = i32x16::splat(((mask >> 16) & 0xFFFF) as i32);
                        let vecmask = (vecidx & vecmask).eq(vecidx);

                        i8x16::from_cast(vecmask)
                            .write_to_slice_unaligned(&mut tmp[16..32]);

                        let vecmask = i32x16::splat(((mask >> 32) & 0xFFFF) as i32);
                        let vecmask = (vecidx & vecmask).eq(vecidx);

                        i8x16::from_cast(vecmask)
                            .write_to_slice_unaligned(&mut tmp[32..48]);

                        let vecmask = i32x16::splat(((mask >> 48) & 0xFFFF) as i32);
                        let vecmask = (vecidx & vecmask).eq(vecidx);

                        i8x16::from_cast(vecmask)
                            .write_to_slice_unaligned(&mut tmp[48..64]);

                        unsafe { std::mem::transmute(i8x64::from_slice_unaligned(tmp)) }
                    }
                    _ => panic!("Invalid number of vector lanes"),
                }
            }

            #[inline]
            fn mask_to_u64(mask: &Self::SimdMask) -> u64 {
                mask.bitmask() as u64
            }

            #[inline]
            fn mask_get(mask: &Self::SimdMask, idx: usize) -> bool {
                unsafe { mask.extract_unchecked(idx) }
            }

            #[inline]
            fn mask_set(mask: Self::SimdMask, idx: usize, value: bool) -> Self::SimdMask {
                unsafe { mask.replace_unchecked(idx, value) }
            }

            /// Selects elements of `a` and `b` using `mask`
            #[inline]
            fn mask_select(
                mask: Self::SimdMask,
                a: Self::Simd,
                b: Self::Simd,
            ) -> Self::Simd {
                mask.select(a, b)
            }

            #[inline]
            fn mask_any(mask: Self::SimdMask) -> bool {
                mask.any()
            }

            #[inline]
            fn bin_op<F: Fn(Self::Simd, Self::Simd) -> Self::Simd>(
                left: Self::Simd,
                right: Self::Simd,
                op: F,
            ) -> Self::Simd {
                op(left, right)
            }

            #[inline]
            fn eq(left: Self::Simd, right: Self::Simd) -> Self::SimdMask {
                left.eq(right)
            }

            #[inline]
            fn ne(left: Self::Simd, right: Self::Simd) -> Self::SimdMask {
                left.ne(right)
            }

            #[inline]
            fn lt(left: Self::Simd, right: Self::Simd) -> Self::SimdMask {
                left.lt(right)
            }

            #[inline]
            fn le(left: Self::Simd, right: Self::Simd) -> Self::SimdMask {
                left.le(right)
            }

            #[inline]
            fn gt(left: Self::Simd, right: Self::Simd) -> Self::SimdMask {
                left.gt(right)
            }

            #[inline]
            fn ge(left: Self::Simd, right: Self::Simd) -> Self::SimdMask {
                left.ge(right)
            }

            #[inline]
            fn write(simd_result: Self::Simd, slice: &mut [Self::Native]) {
                unsafe { simd_result.write_to_slice_unaligned_unchecked(slice) };
            }
        }

        #[cfg(not(simd))]
        impl ArrowNumericType for $impl_ty {}
    };
}

make_numeric_type!(Int8Type, i8, i8x64, m8x64);
make_numeric_type!(Int16Type, i16, i16x32, m16x32);
make_numeric_type!(Int32Type, i32, i32x16, m32x16);
make_numeric_type!(Int64Type, i64, i64x8, m64x8);
make_numeric_type!(UInt8Type, u8, u8x64, m8x64);
make_numeric_type!(UInt16Type, u16, u16x32, m16x32);
make_numeric_type!(UInt32Type, u32, u32x16, m32x16);
make_numeric_type!(UInt64Type, u64, u64x8, m64x8);
make_numeric_type!(Float32Type, f32, f32x16, m32x16);
make_numeric_type!(Float64Type, f64, f64x8, m64x8);

make_numeric_type!(TimestampSecondType, i64, i64x8, m64x8);
make_numeric_type!(TimestampMillisecondType, i64, i64x8, m64x8);
make_numeric_type!(TimestampMicrosecondType, i64, i64x8, m64x8);
make_numeric_type!(TimestampNanosecondType, i64, i64x8, m64x8);
make_numeric_type!(Date32Type, i32, i32x16, m32x16);
make_numeric_type!(Date64Type, i64, i64x8, m64x8);
make_numeric_type!(Time32SecondType, i32, i32x16, m32x16);
make_numeric_type!(Time32MillisecondType, i32, i32x16, m32x16);
make_numeric_type!(Time64MicrosecondType, i64, i64x8, m64x8);
make_numeric_type!(Time64NanosecondType, i64, i64x8, m64x8);
make_numeric_type!(IntervalYearMonthType, i32, i32x16, m32x16);
make_numeric_type!(IntervalDayTimeType, i64, i64x8, m64x8);
make_numeric_type!(DurationSecondType, i64, i64x8, m64x8);
make_numeric_type!(DurationMillisecondType, i64, i64x8, m64x8);
make_numeric_type!(DurationMicrosecondType, i64, i64x8, m64x8);
make_numeric_type!(DurationNanosecondType, i64, i64x8, m64x8);

/// A subtype of primitive type that represents signed numeric values.
///
/// SIMD operations are defined in this trait if available on the target system.
#[cfg(simd)]
pub trait ArrowSignedNumericType: ArrowNumericType
where
    Self::SignedSimd: Neg<Output = Self::SignedSimd>,
{
    /// Defines the SIMD type that should be used for this numeric type
    type SignedSimd;

    /// Loads a slice of signed numeric type into a SIMD register
    fn load_signed(slice: &[Self::Native]) -> Self::SignedSimd;

    /// Performs a SIMD unary operation on signed numeric type
    fn signed_unary_op<F: Fn(Self::SignedSimd) -> Self::SignedSimd>(
        a: Self::SignedSimd,
        op: F,
    ) -> Self::SignedSimd;

    /// Writes a signed SIMD result back to a slice
    fn write_signed(simd_result: Self::SignedSimd, slice: &mut [Self::Native]);
}

#[cfg(not(simd))]
pub trait ArrowSignedNumericType: ArrowNumericType
where
    Self::Native: Neg<Output = Self::Native>,
{
}

macro_rules! make_signed_numeric_type {
    ($impl_ty:ty, $simd_ty:ident) => {
        #[cfg(simd)]
        impl ArrowSignedNumericType for $impl_ty {
            type SignedSimd = $simd_ty;

            #[inline]
            fn load_signed(slice: &[Self::Native]) -> Self::SignedSimd {
                unsafe { Self::SignedSimd::from_slice_unaligned_unchecked(slice) }
            }

            #[inline]
            fn signed_unary_op<F: Fn(Self::SignedSimd) -> Self::SignedSimd>(
                a: Self::SignedSimd,
                op: F,
            ) -> Self::SignedSimd {
                op(a)
            }

            #[inline]
            fn write_signed(simd_result: Self::SignedSimd, slice: &mut [Self::Native]) {
                unsafe { simd_result.write_to_slice_unaligned_unchecked(slice) };
            }
        }

        #[cfg(not(simd))]
        impl ArrowSignedNumericType for $impl_ty {}
    };
}

make_signed_numeric_type!(Int8Type, i8x64);
make_signed_numeric_type!(Int16Type, i16x32);
make_signed_numeric_type!(Int32Type, i32x16);
make_signed_numeric_type!(Int64Type, i64x8);
make_signed_numeric_type!(Float32Type, f32x16);
make_signed_numeric_type!(Float64Type, f64x8);

/// A subtype of primitive type that represents temporal values.
pub trait ArrowTemporalType: ArrowPrimitiveType {}

impl ArrowTemporalType for TimestampSecondType {}
impl ArrowTemporalType for TimestampMillisecondType {}
impl ArrowTemporalType for TimestampMicrosecondType {}
impl ArrowTemporalType for TimestampNanosecondType {}
impl ArrowTemporalType for Date32Type {}
impl ArrowTemporalType for Date64Type {}
impl ArrowTemporalType for Time32SecondType {}
impl ArrowTemporalType for Time32MillisecondType {}
impl ArrowTemporalType for Time64MicrosecondType {}
impl ArrowTemporalType for Time64NanosecondType {}
// impl ArrowTemporalType for IntervalYearMonthType {}
// impl ArrowTemporalType for IntervalDayTimeType {}

/// A timestamp type allows us to create array builders that take a timestamp.
pub trait ArrowTimestampType: ArrowTemporalType {
    /// Returns the `TimeUnit` of this timestamp.
    fn get_time_unit() -> TimeUnit;
}

impl ArrowTimestampType for TimestampSecondType {
    fn get_time_unit() -> TimeUnit {
        TimeUnit::Second
    }
}
impl ArrowTimestampType for TimestampMillisecondType {
    fn get_time_unit() -> TimeUnit {
        TimeUnit::Millisecond
    }
}
impl ArrowTimestampType for TimestampMicrosecondType {
    fn get_time_unit() -> TimeUnit {
        TimeUnit::Microsecond
    }
}
impl ArrowTimestampType for TimestampNanosecondType {
    fn get_time_unit() -> TimeUnit {
        TimeUnit::Nanosecond
    }
}

/// Allows conversion from supported Arrow types to a byte slice.
pub trait ToByteSlice {
    /// Converts this instance into a byte slice
    fn to_byte_slice(&self) -> &[u8];
}

impl<T: ArrowNativeType> ToByteSlice for [T] {
    #[inline]
    fn to_byte_slice(&self) -> &[u8] {
        let raw_ptr = self.as_ptr() as *const T as *const u8;
        unsafe { from_raw_parts(raw_ptr, self.len() * size_of::<T>()) }
    }
}

impl<T: ArrowNativeType> ToByteSlice for T {
    #[inline]
    fn to_byte_slice(&self) -> &[u8] {
        let raw_ptr = self as *const T as *const u8;
        unsafe { from_raw_parts(raw_ptr, size_of::<T>()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use serde_json::Number;
    use serde_json::Value::{Bool, Number as VNumber};
    use std::{
        collections::{BTreeMap, HashMap},
        f32::NAN,
    };

    #[test]
    fn test_list_datatype_equality() {
        // tests that list type equality is checked while ignoring list names
        let list_a = DataType::List(Box::new(Field::new("item", DataType::Int32, true)));
        let list_b = DataType::List(Box::new(Field::new("array", DataType::Int32, true)));
        let list_c = DataType::List(Box::new(Field::new("item", DataType::Int32, false)));
        let list_d = DataType::List(Box::new(Field::new("item", DataType::UInt32, true)));
        assert!(list_a.equals_datatype(&list_b));
        assert!(!list_a.equals_datatype(&list_c));
        assert!(!list_b.equals_datatype(&list_c));
        assert!(!list_a.equals_datatype(&list_d));

        let list_e =
            DataType::FixedSizeList(Box::new(Field::new("item", list_a, false)), 3);
        let list_f =
            DataType::FixedSizeList(Box::new(Field::new("array", list_b, false)), 3);
        let list_g = DataType::FixedSizeList(
            Box::new(Field::new("item", DataType::FixedSizeBinary(3), true)),
            3,
        );
        assert!(list_e.equals_datatype(&list_f));
        assert!(!list_e.equals_datatype(&list_g));
        assert!(!list_f.equals_datatype(&list_g));

        let list_h = DataType::Struct(vec![Field::new("f1", list_e, true)]);
        let list_i = DataType::Struct(vec![Field::new("f1", list_f.clone(), true)]);
        let list_j = DataType::Struct(vec![Field::new("f1", list_f.clone(), false)]);
        let list_k = DataType::Struct(vec![
            Field::new("f1", list_f.clone(), false),
            Field::new("f2", list_g.clone(), false),
            Field::new("f3", DataType::Utf8, true),
        ]);
        let list_l = DataType::Struct(vec![
            Field::new("ff1", list_f.clone(), false),
            Field::new("ff2", list_g.clone(), false),
            Field::new("ff3", DataType::LargeUtf8, true),
        ]);
        let list_m = DataType::Struct(vec![
            Field::new("ff1", list_f, false),
            Field::new("ff2", list_g, false),
            Field::new("ff3", DataType::Utf8, true),
        ]);
        assert!(list_h.equals_datatype(&list_i));
        assert!(!list_h.equals_datatype(&list_j));
        assert!(!list_k.equals_datatype(&list_l));
        assert!(list_k.equals_datatype(&list_m));
    }

    #[test]
    fn create_struct_type() {
        let _person = DataType::Struct(vec![
            Field::new("first_name", DataType::Utf8, false),
            Field::new("last_name", DataType::Utf8, false),
            Field::new(
                "address",
                DataType::Struct(vec![
                    Field::new("street", DataType::Utf8, false),
                    Field::new("zip", DataType::UInt16, false),
                ]),
                false,
            ),
        ]);
    }

    #[test]
    fn serde_struct_type() {
        let kv_array = [("k".to_string(), "v".to_string())];
        let field_metadata: BTreeMap<String, String> = kv_array.iter().cloned().collect();

        // Non-empty map: should be converted as JSON obj { ... }
        let mut first_name = Field::new("first_name", DataType::Utf8, false);
        first_name.set_metadata(Some(field_metadata));

        // Empty map: should be omitted.
        let mut last_name = Field::new("last_name", DataType::Utf8, false);
        last_name.set_metadata(Some(BTreeMap::default()));

        let person = DataType::Struct(vec![
            first_name,
            last_name,
            Field::new(
                "address",
                DataType::Struct(vec![
                    Field::new("street", DataType::Utf8, false),
                    Field::new("zip", DataType::UInt16, false),
                ]),
                false,
            ),
        ]);

        let serialized = serde_json::to_string(&person).unwrap();

        // NOTE that this is testing the default (derived) serialization format, not the
        // JSON format specified in metadata.md

        assert_eq!(
            "{\"Struct\":[\
             {\"name\":\"first_name\",\"data_type\":\"Utf8\",\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false,\"metadata\":{\"k\":\"v\"}},\
             {\"name\":\"last_name\",\"data_type\":\"Utf8\",\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false},\
             {\"name\":\"address\",\"data_type\":{\"Struct\":\
             [{\"name\":\"street\",\"data_type\":\"Utf8\",\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false},\
             {\"name\":\"zip\",\"data_type\":\"UInt16\",\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false}\
             ]},\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false}]}",
            serialized
        );

        let deserialized = serde_json::from_str(&serialized).unwrap();

        assert_eq!(person, deserialized);
    }

    #[test]
    fn struct_field_to_json() {
        let f = Field::new(
            "address",
            DataType::Struct(vec![
                Field::new("street", DataType::Utf8, false),
                Field::new("zip", DataType::UInt16, false),
            ]),
            false,
        );
        let value: Value = serde_json::from_str(
            r#"{
                "name": "address",
                "nullable": false,
                "type": {
                    "name": "struct"
                },
                "children": [
                    {
                        "name": "street",
                        "nullable": false,
                        "type": {
                            "name": "utf8"
                        },
                        "children": []
                    },
                    {
                        "name": "zip",
                        "nullable": false,
                        "type": {
                            "name": "int",
                            "bitWidth": 16,
                            "isSigned": false
                        },
                        "children": []
                    }
                ]
            }"#,
        )
        .unwrap();
        assert_eq!(value, f.to_json());
    }

    #[test]
    fn primitive_field_to_json() {
        let f = Field::new("first_name", DataType::Utf8, false);
        let value: Value = serde_json::from_str(
            r#"{
                "name": "first_name",
                "nullable": false,
                "type": {
                    "name": "utf8"
                },
                "children": []
            }"#,
        )
        .unwrap();
        assert_eq!(value, f.to_json());
    }
    #[test]
    fn parse_struct_from_json() {
        let json = r#"
        {
            "name": "address",
            "type": {
                "name": "struct"
            },
            "nullable": false,
            "children": [
                {
                    "name": "street",
                    "type": {
                    "name": "utf8"
                    },
                    "nullable": false,
                    "children": []
                },
                {
                    "name": "zip",
                    "type": {
                    "name": "int",
                    "isSigned": false,
                    "bitWidth": 16
                    },
                    "nullable": false,
                    "children": []
                }
            ]
        }
        "#;
        let value: Value = serde_json::from_str(json).unwrap();
        let dt = Field::from(&value).unwrap();

        let expected = Field::new(
            "address",
            DataType::Struct(vec![
                Field::new("street", DataType::Utf8, false),
                Field::new("zip", DataType::UInt16, false),
            ]),
            false,
        );

        assert_eq!(expected, dt);
    }

    #[test]
    fn parse_utf8_from_json() {
        let json = "{\"name\":\"utf8\"}";
        let value: Value = serde_json::from_str(json).unwrap();
        let dt = DataType::from(&value).unwrap();
        assert_eq!(DataType::Utf8, dt);
    }

    #[test]
    fn parse_int32_from_json() {
        let json = "{\"name\": \"int\", \"isSigned\": true, \"bitWidth\": 32}";
        let value: Value = serde_json::from_str(json).unwrap();
        let dt = DataType::from(&value).unwrap();
        assert_eq!(DataType::Int32, dt);
    }

    #[test]
    fn schema_json() {
        // Add some custom metadata
        let metadata: HashMap<String, String> =
            [("Key".to_string(), "Value".to_string())]
                .iter()
                .cloned()
                .collect();

        let schema = Schema::new_with_metadata(
            vec![
                Field::new("c1", DataType::Utf8, false),
                Field::new("c2", DataType::Binary, false),
                Field::new("c3", DataType::FixedSizeBinary(3), false),
                Field::new("c4", DataType::Boolean, false),
                Field::new("c5", DataType::Date32, false),
                Field::new("c6", DataType::Date64, false),
                Field::new("c7", DataType::Time32(TimeUnit::Second), false),
                Field::new("c8", DataType::Time32(TimeUnit::Millisecond), false),
                Field::new("c9", DataType::Time32(TimeUnit::Microsecond), false),
                Field::new("c10", DataType::Time32(TimeUnit::Nanosecond), false),
                Field::new("c11", DataType::Time64(TimeUnit::Second), false),
                Field::new("c12", DataType::Time64(TimeUnit::Millisecond), false),
                Field::new("c13", DataType::Time64(TimeUnit::Microsecond), false),
                Field::new("c14", DataType::Time64(TimeUnit::Nanosecond), false),
                Field::new("c15", DataType::Timestamp(TimeUnit::Second, None), false),
                Field::new(
                    "c16",
                    DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".to_string())),
                    false,
                ),
                Field::new(
                    "c17",
                    DataType::Timestamp(
                        TimeUnit::Microsecond,
                        Some("Africa/Johannesburg".to_string()),
                    ),
                    false,
                ),
                Field::new(
                    "c18",
                    DataType::Timestamp(TimeUnit::Nanosecond, None),
                    false,
                ),
                Field::new("c19", DataType::Interval(IntervalUnit::DayTime), false),
                Field::new("c20", DataType::Interval(IntervalUnit::YearMonth), false),
                Field::new(
                    "c21",
                    DataType::List(Box::new(Field::new("item", DataType::Boolean, true))),
                    false,
                ),
                Field::new(
                    "c22",
                    DataType::FixedSizeList(
                        Box::new(Field::new("bools", DataType::Boolean, false)),
                        5,
                    ),
                    false,
                ),
                Field::new(
                    "c23",
                    DataType::List(Box::new(Field::new(
                        "inner_list",
                        DataType::List(Box::new(Field::new(
                            "struct",
                            DataType::Struct(vec![]),
                            true,
                        ))),
                        false,
                    ))),
                    true,
                ),
                Field::new(
                    "c24",
                    DataType::Struct(vec![
                        Field::new("a", DataType::Utf8, false),
                        Field::new("b", DataType::UInt16, false),
                    ]),
                    false,
                ),
                Field::new("c25", DataType::Interval(IntervalUnit::YearMonth), true),
                Field::new("c26", DataType::Interval(IntervalUnit::DayTime), true),
                Field::new("c27", DataType::Duration(TimeUnit::Second), false),
                Field::new("c28", DataType::Duration(TimeUnit::Millisecond), false),
                Field::new("c29", DataType::Duration(TimeUnit::Microsecond), false),
                Field::new("c30", DataType::Duration(TimeUnit::Nanosecond), false),
                Field::new_dict(
                    "c31",
                    DataType::Dictionary(
                        Box::new(DataType::Int32),
                        Box::new(DataType::Utf8),
                    ),
                    true,
                    123,
                    true,
                ),
                Field::new("c32", DataType::LargeBinary, true),
                Field::new("c33", DataType::LargeUtf8, true),
                Field::new(
                    "c34",
                    DataType::LargeList(Box::new(Field::new(
                        "inner_large_list",
                        DataType::LargeList(Box::new(Field::new(
                            "struct",
                            DataType::Struct(vec![]),
                            false,
                        ))),
                        true,
                    ))),
                    true,
                ),
            ],
            metadata,
        );

        let expected = schema.to_json();
        let json = r#"{
                "fields": [
                    {
                        "name": "c1",
                        "nullable": false,
                        "type": {
                            "name": "utf8"
                        },
                        "children": []
                    },
                    {
                        "name": "c2",
                        "nullable": false,
                        "type": {
                            "name": "binary"
                        },
                        "children": []
                    },
                    {
                        "name": "c3",
                        "nullable": false,
                        "type": {
                            "name": "fixedsizebinary",
                            "byteWidth": 3
                        },
                        "children": []
                    },
                    {
                        "name": "c4",
                        "nullable": false,
                        "type": {
                            "name": "bool"
                        },
                        "children": []
                    },
                    {
                        "name": "c5",
                        "nullable": false,
                        "type": {
                            "name": "date",
                            "unit": "DAY"
                        },
                        "children": []
                    },
                    {
                        "name": "c6",
                        "nullable": false,
                        "type": {
                            "name": "date",
                            "unit": "MILLISECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c7",
                        "nullable": false,
                        "type": {
                            "name": "time",
                            "bitWidth": 32,
                            "unit": "SECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c8",
                        "nullable": false,
                        "type": {
                            "name": "time",
                            "bitWidth": 32,
                            "unit": "MILLISECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c9",
                        "nullable": false,
                        "type": {
                            "name": "time",
                            "bitWidth": 32,
                            "unit": "MICROSECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c10",
                        "nullable": false,
                        "type": {
                            "name": "time",
                            "bitWidth": 32,
                            "unit": "NANOSECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c11",
                        "nullable": false,
                        "type": {
                            "name": "time",
                            "bitWidth": 64,
                            "unit": "SECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c12",
                        "nullable": false,
                        "type": {
                            "name": "time",
                            "bitWidth": 64,
                            "unit": "MILLISECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c13",
                        "nullable": false,
                        "type": {
                            "name": "time",
                            "bitWidth": 64,
                            "unit": "MICROSECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c14",
                        "nullable": false,
                        "type": {
                            "name": "time",
                            "bitWidth": 64,
                            "unit": "NANOSECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c15",
                        "nullable": false,
                        "type": {
                            "name": "timestamp",
                            "unit": "SECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c16",
                        "nullable": false,
                        "type": {
                            "name": "timestamp",
                            "unit": "MILLISECOND",
                            "timezone": "UTC"
                        },
                        "children": []
                    },
                    {
                        "name": "c17",
                        "nullable": false,
                        "type": {
                            "name": "timestamp",
                            "unit": "MICROSECOND",
                            "timezone": "Africa/Johannesburg"
                        },
                        "children": []
                    },
                    {
                        "name": "c18",
                        "nullable": false,
                        "type": {
                            "name": "timestamp",
                            "unit": "NANOSECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c19",
                        "nullable": false,
                        "type": {
                            "name": "interval",
                            "unit": "DAY_TIME"
                        },
                        "children": []
                    },
                    {
                        "name": "c20",
                        "nullable": false,
                        "type": {
                            "name": "interval",
                            "unit": "YEAR_MONTH"
                        },
                        "children": []
                    },
                    {
                        "name": "c21",
                        "nullable": false,
                        "type": {
                            "name": "list"
                        },
                        "children": [
                            {
                                "name": "item",
                                "nullable": true,
                                "type": {
                                    "name": "bool"
                                },
                                "children": []
                            }
                        ]
                    },
                    {
                        "name": "c22",
                        "nullable": false,
                        "type": {
                            "name": "fixedsizelist",
                            "listSize": 5
                        },
                        "children": [
                            {
                                "name": "bools",
                                "nullable": false,
                                "type": {
                                    "name": "bool"
                                },
                                "children": []
                            }
                        ]
                    },
                    {
                        "name": "c23",
                        "nullable": true,
                        "type": {
                            "name": "list"
                        },
                        "children": [
                            {
                                "name": "inner_list",
                                "nullable": false,
                                "type": {
                                    "name": "list"
                                },
                                "children": [
                                    {
                                        "name": "struct",
                                        "nullable": true,
                                        "type": {
                                            "name": "struct"
                                        },
                                        "children": []
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "name": "c24",
                        "nullable": false,
                        "type": {
                            "name": "struct"
                        },
                        "children": [
                            {
                                "name": "a",
                                "nullable": false,
                                "type": {
                                    "name": "utf8"
                                },
                                "children": []
                            },
                            {
                                "name": "b",
                                "nullable": false,
                                "type": {
                                    "name": "int",
                                    "bitWidth": 16,
                                    "isSigned": false
                                },
                                "children": []
                            }
                        ]
                    },
                    {
                        "name": "c25",
                        "nullable": true,
                        "type": {
                            "name": "interval",
                            "unit": "YEAR_MONTH"
                        },
                        "children": []
                    },
                    {
                        "name": "c26",
                        "nullable": true,
                        "type": {
                            "name": "interval",
                            "unit": "DAY_TIME"
                        },
                        "children": []
                    },
                    {
                        "name": "c27",
                        "nullable": false,
                        "type": {
                            "name": "duration",
                            "unit": "SECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c28",
                        "nullable": false,
                        "type": {
                            "name": "duration",
                            "unit": "MILLISECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c29",
                        "nullable": false,
                        "type": {
                            "name": "duration",
                            "unit": "MICROSECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c30",
                        "nullable": false,
                        "type": {
                            "name": "duration",
                            "unit": "NANOSECOND"
                        },
                        "children": []
                    },
                    {
                        "name": "c31",
                        "nullable": true,
                        "children": [],
                        "type": {
                          "name": "utf8"
                        },
                        "dictionary": {
                          "id": 123,
                          "indexType": {
                            "name": "int",
                            "bitWidth": 32,
                            "isSigned": true
                          },
                          "isOrdered": true
                        }
                    },
                    {
                        "name": "c32",
                        "nullable": true,
                        "type": {
                          "name": "largebinary"
                        },
                        "children": []
                    },
                    {
                        "name": "c33",
                        "nullable": true,
                        "type": {
                          "name": "largeutf8"
                        },
                        "children": []
                    },
                    {
                        "name": "c34",
                        "nullable": true,
                        "type": {
                          "name": "largelist"
                        },
                        "children": [
                            {
                                "name": "inner_large_list",
                                "nullable": true,
                                "type": {
                                    "name": "largelist"
                                },
                                "children": [
                                    {
                                        "name": "struct",
                                        "nullable": false,
                                        "type": {
                                            "name": "struct"
                                        },
                                        "children": []
                                    }
                                ]
                            }
                        ]
                    }
                ],
                "metadata" : {
                    "Key": "Value"
                }
            }"#;
        let value: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(expected, value);

        // convert back to a schema
        let value: Value = serde_json::from_str(&json).unwrap();
        let schema2 = Schema::from(&value).unwrap();

        assert_eq!(schema, schema2);

        // Check that empty metadata produces empty value in JSON and can be parsed
        let json = r#"{
                "fields": [
                    {
                        "name": "c1",
                        "nullable": false,
                        "type": {
                            "name": "utf8"
                        },
                        "children": []
                    }
                ],
                "metadata": {}
            }"#;
        let value: Value = serde_json::from_str(&json).unwrap();
        let schema = Schema::from(&value).unwrap();
        assert!(schema.metadata.is_empty());

        // Check that metadata field is not required in the JSON.
        let json = r#"{
                "fields": [
                    {
                        "name": "c1",
                        "nullable": false,
                        "type": {
                            "name": "utf8"
                        },
                        "children": []
                    }
                ]
            }"#;
        let value: Value = serde_json::from_str(&json).unwrap();
        let schema = Schema::from(&value).unwrap();
        assert!(schema.metadata.is_empty());
    }

    #[test]
    fn create_schema_string() {
        let schema = person_schema();
        assert_eq!(schema.to_string(),
        "Field { name: \"first_name\", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: Some({\"k\": \"v\"}) }, \
        Field { name: \"last_name\", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: None }, \
        Field { name: \"address\", data_type: Struct([\
            Field { name: \"street\", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: None }, \
            Field { name: \"zip\", data_type: UInt16, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: None }\
        ]), nullable: false, dict_id: 0, dict_is_ordered: false, metadata: None }, \
        Field { name: \"interests\", data_type: Dictionary(Int32, Utf8), nullable: true, dict_id: 123, dict_is_ordered: true, metadata: None }")
    }

    #[test]
    fn schema_field_accessors() {
        let schema = person_schema();

        // test schema accessors
        assert_eq!(schema.fields().len(), 4);

        // test field accessors
        let first_name = &schema.fields()[0];
        assert_eq!(first_name.name(), "first_name");
        assert_eq!(first_name.data_type(), &DataType::Utf8);
        assert_eq!(first_name.is_nullable(), false);
        assert_eq!(first_name.dict_id(), None);
        assert_eq!(first_name.dict_is_ordered(), None);

        let metadata = first_name.metadata();
        assert!(metadata.is_some());
        let md = metadata.as_ref().unwrap();
        assert_eq!(md.len(), 1);
        let key = md.get("k");
        assert!(key.is_some());
        assert_eq!(key.unwrap(), "v");

        let interests = &schema.fields()[3];
        assert_eq!(interests.name(), "interests");
        assert_eq!(
            interests.data_type(),
            &DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8))
        );
        assert_eq!(interests.dict_id(), Some(123));
        assert_eq!(interests.dict_is_ordered(), Some(true));
    }

    #[test]
    #[should_panic(
        expected = "Unable to get field named \\\"nickname\\\". Valid fields: [\\\"first_name\\\", \\\"last_name\\\", \\\"address\\\", \\\"interests\\\"]"
    )]
    fn schema_index_of() {
        let schema = person_schema();
        assert_eq!(schema.index_of("first_name").unwrap(), 0);
        assert_eq!(schema.index_of("last_name").unwrap(), 1);
        schema.index_of("nickname").unwrap();
    }

    #[test]
    #[should_panic(
        expected = "Unable to get field named \\\"nickname\\\". Valid fields: [\\\"first_name\\\", \\\"last_name\\\", \\\"address\\\", \\\"interests\\\"]"
    )]
    fn schema_field_with_name() {
        let schema = person_schema();
        assert_eq!(
            schema.field_with_name("first_name").unwrap().name(),
            "first_name"
        );
        assert_eq!(
            schema.field_with_name("last_name").unwrap().name(),
            "last_name"
        );
        schema.field_with_name("nickname").unwrap();
    }

    #[test]
    fn schema_field_with_dict_id() {
        let schema = person_schema();

        let fields_dict_123: Vec<_> = schema
            .fields_with_dict_id(123)
            .iter()
            .map(|f| f.name())
            .collect();
        assert_eq!(fields_dict_123, vec!["interests"]);

        assert!(schema.fields_with_dict_id(456).is_empty());
    }

    #[test]
    fn schema_equality() {
        let schema1 = Schema::new(vec![
            Field::new("c1", DataType::Utf8, false),
            Field::new("c2", DataType::Float64, true),
            Field::new("c3", DataType::LargeBinary, true),
        ]);
        let schema2 = Schema::new(vec![
            Field::new("c1", DataType::Utf8, false),
            Field::new("c2", DataType::Float64, true),
            Field::new("c3", DataType::LargeBinary, true),
        ]);

        assert_eq!(schema1, schema2);

        let schema3 = Schema::new(vec![
            Field::new("c1", DataType::Utf8, false),
            Field::new("c2", DataType::Float32, true),
        ]);
        let schema4 = Schema::new(vec![
            Field::new("C1", DataType::Utf8, false),
            Field::new("C2", DataType::Float64, true),
        ]);

        assert!(schema1 != schema3);
        assert!(schema1 != schema4);
        assert!(schema2 != schema3);
        assert!(schema2 != schema4);
        assert!(schema3 != schema4);

        let mut f = Field::new("c1", DataType::Utf8, false);
        f.set_metadata(Some(
            [("foo".to_string(), "bar".to_string())]
                .iter()
                .cloned()
                .collect(),
        ));
        let schema5 = Schema::new(vec![
            f,
            Field::new("c2", DataType::Float64, true),
            Field::new("c3", DataType::LargeBinary, true),
        ]);
        assert!(schema1 != schema5);
    }

    #[test]
    fn test_arrow_native_type_to_json() {
        assert_eq!(Some(Bool(true)), true.into_json_value());
        assert_eq!(Some(VNumber(Number::from(1))), 1i8.into_json_value());
        assert_eq!(Some(VNumber(Number::from(1))), 1i16.into_json_value());
        assert_eq!(Some(VNumber(Number::from(1))), 1i32.into_json_value());
        assert_eq!(Some(VNumber(Number::from(1))), 1i64.into_json_value());
        assert_eq!(Some(VNumber(Number::from(1))), 1u8.into_json_value());
        assert_eq!(Some(VNumber(Number::from(1))), 1u16.into_json_value());
        assert_eq!(Some(VNumber(Number::from(1))), 1u32.into_json_value());
        assert_eq!(Some(VNumber(Number::from(1))), 1u64.into_json_value());
        assert_eq!(
            Some(VNumber(Number::from_f64(0.01f64).unwrap())),
            0.01.into_json_value()
        );
        assert_eq!(
            Some(VNumber(Number::from_f64(0.01f64).unwrap())),
            0.01f64.into_json_value()
        );
        assert_eq!(None, NAN.into_json_value());
    }

    fn person_schema() -> Schema {
        let kv_array = [("k".to_string(), "v".to_string())];
        let field_metadata: BTreeMap<String, String> = kv_array.iter().cloned().collect();
        let mut first_name = Field::new("first_name", DataType::Utf8, false);
        first_name.set_metadata(Some(field_metadata));

        Schema::new(vec![
            first_name,
            Field::new("last_name", DataType::Utf8, false),
            Field::new(
                "address",
                DataType::Struct(vec![
                    Field::new("street", DataType::Utf8, false),
                    Field::new("zip", DataType::UInt16, false),
                ]),
                false,
            ),
            Field::new_dict(
                "interests",
                DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8)),
                true,
                123,
                true,
            ),
        ])
    }

    #[test]
    fn test_try_merge_field_with_metadata() {
        // 1. Different values for the same key should cause error.
        let metadata1: BTreeMap<String, String> =
            [("foo".to_string(), "bar".to_string())]
                .iter()
                .cloned()
                .collect();
        let mut f1 = Field::new("first_name", DataType::Utf8, false);
        f1.set_metadata(Some(metadata1));

        let metadata2: BTreeMap<String, String> =
            [("foo".to_string(), "baz".to_string())]
                .iter()
                .cloned()
                .collect();
        let mut f2 = Field::new("first_name", DataType::Utf8, false);
        f2.set_metadata(Some(metadata2));

        assert!(
            Schema::try_merge(vec![Schema::new(vec![f1]), Schema::new(vec![f2])])
                .is_err()
        );

        // 2. None + Some
        let mut f1 = Field::new("first_name", DataType::Utf8, false);
        let metadata2: BTreeMap<String, String> =
            [("missing".to_string(), "value".to_string())]
                .iter()
                .cloned()
                .collect();
        let mut f2 = Field::new("first_name", DataType::Utf8, false);
        f2.set_metadata(Some(metadata2));

        assert!(f1.try_merge(&f2).is_ok());
        assert!(f1.metadata().is_some());
        assert_eq!(
            f1.metadata().clone().unwrap(),
            f2.metadata().clone().unwrap()
        );

        // 3. Some + Some
        let mut f1 = Field::new("first_name", DataType::Utf8, false);
        f1.set_metadata(Some(
            [("foo".to_string(), "bar".to_string())]
                .iter()
                .cloned()
                .collect(),
        ));
        let mut f2 = Field::new("first_name", DataType::Utf8, false);
        f2.set_metadata(Some(
            [("foo2".to_string(), "bar2".to_string())]
                .iter()
                .cloned()
                .collect(),
        ));

        assert!(f1.try_merge(&f2).is_ok());
        assert!(f1.metadata().is_some());
        assert_eq!(
            f1.metadata().clone().unwrap(),
            [
                ("foo".to_string(), "bar".to_string()),
                ("foo2".to_string(), "bar2".to_string())
            ]
            .iter()
            .cloned()
            .collect()
        );

        // 4. Some + None.
        let mut f1 = Field::new("first_name", DataType::Utf8, false);
        f1.set_metadata(Some(
            [("foo".to_string(), "bar".to_string())]
                .iter()
                .cloned()
                .collect(),
        ));
        let f2 = Field::new("first_name", DataType::Utf8, false);
        assert!(f1.try_merge(&f2).is_ok());
        assert!(f1.metadata().is_some());
        assert_eq!(
            f1.metadata().clone().unwrap(),
            [("foo".to_string(), "bar".to_string())]
                .iter()
                .cloned()
                .collect()
        );

        // 5. None + None.
        let mut f1 = Field::new("first_name", DataType::Utf8, false);
        let f2 = Field::new("first_name", DataType::Utf8, false);
        assert!(f1.try_merge(&f2).is_ok());
        assert!(f1.metadata().is_none());
    }

    #[test]
    fn test_schema_merge() -> Result<()> {
        let merged = Schema::try_merge(vec![
            Schema::new(vec![
                Field::new("first_name", DataType::Utf8, false),
                Field::new("last_name", DataType::Utf8, false),
                Field::new(
                    "address",
                    DataType::Struct(vec![Field::new("zip", DataType::UInt16, false)]),
                    false,
                ),
            ]),
            Schema::new_with_metadata(
                vec![
                    // nullable merge
                    Field::new("last_name", DataType::Utf8, true),
                    Field::new(
                        "address",
                        DataType::Struct(vec![
                            // add new nested field
                            Field::new("street", DataType::Utf8, false),
                            // nullable merge on nested field
                            Field::new("zip", DataType::UInt16, true),
                        ]),
                        false,
                    ),
                    // new field
                    Field::new("number", DataType::Utf8, true),
                ],
                [("foo".to_string(), "bar".to_string())]
                    .iter()
                    .cloned()
                    .collect::<HashMap<String, String>>(),
            ),
        ])?;

        assert_eq!(
            merged,
            Schema::new_with_metadata(
                vec![
                    Field::new("first_name", DataType::Utf8, false),
                    Field::new("last_name", DataType::Utf8, true),
                    Field::new(
                        "address",
                        DataType::Struct(vec![
                            Field::new("zip", DataType::UInt16, true),
                            Field::new("street", DataType::Utf8, false),
                        ]),
                        false,
                    ),
                    Field::new("number", DataType::Utf8, true),
                ],
                [("foo".to_string(), "bar".to_string())]
                    .iter()
                    .cloned()
                    .collect::<HashMap<String, String>>()
            )
        );

        // support merge union fields
        assert_eq!(
            Schema::try_merge(vec![
                Schema::new(vec![Field::new(
                    "c1",
                    DataType::Union(vec![
                        Field::new("c11", DataType::Utf8, true),
                        Field::new("c12", DataType::Utf8, true),
                    ]),
                    false
                ),]),
                Schema::new(vec![Field::new(
                    "c1",
                    DataType::Union(vec![
                        Field::new("c12", DataType::Utf8, true),
                        Field::new("c13", DataType::Time64(TimeUnit::Second), true),
                    ]),
                    false
                ),])
            ])?,
            Schema::new(vec![Field::new(
                "c1",
                DataType::Union(vec![
                    Field::new("c11", DataType::Utf8, true),
                    Field::new("c12", DataType::Utf8, true),
                    Field::new("c13", DataType::Time64(TimeUnit::Second), true),
                ]),
                false
            ),]),
        );

        // incompatible field should throw error
        assert!(Schema::try_merge(vec![
            Schema::new(vec![
                Field::new("first_name", DataType::Utf8, false),
                Field::new("last_name", DataType::Utf8, false),
            ]),
            Schema::new(vec![Field::new("last_name", DataType::Int64, false),])
        ])
        .is_err());

        // incompatible metadata should throw error
        assert!(Schema::try_merge(vec![
            Schema::new_with_metadata(
                vec![Field::new("first_name", DataType::Utf8, false)],
                [("foo".to_string(), "bar".to_string()),]
                    .iter()
                    .cloned()
                    .collect::<HashMap<String, String>>()
            ),
            Schema::new_with_metadata(
                vec![Field::new("last_name", DataType::Utf8, false)],
                [("foo".to_string(), "baz".to_string()),]
                    .iter()
                    .cloned()
                    .collect::<HashMap<String, String>>()
            )
        ])
        .is_err());

        Ok(())
    }
}

#[cfg(all(test, simd_x86))]
mod arrow_numeric_type_tests {
    use crate::datatypes::{
        ArrowNumericType, Float32Type, Float64Type, Int32Type, Int64Type, Int8Type,
        UInt16Type,
    };
    use packed_simd::*;
    use FromCast;

    /// calculate the expected mask by iterating over all bits
    macro_rules! expected_mask {
        ($T:ty, $MASK:expr) => {{
            let mask = $MASK;
            // simd width of all types is currently 64 bytes -> 512 bits
            let lanes = 64 / std::mem::size_of::<$T>();
            // translate each set bit into a value of all ones (-1) of the correct type
            (0..lanes)
                .map(|i| (if (mask & (1 << i)) != 0 { -1 } else { 0 }))
                .collect::<Vec<$T>>()
        }};
    }

    #[test]
    fn test_mask_f64() {
        let mask = 0b10101010;
        let actual = Float64Type::mask_from_u64(mask);
        let expected = expected_mask!(i64, mask);
        let expected = m64x8::from_cast(i64x8::from_slice_unaligned(expected.as_slice()));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mask_u64() {
        let mask = 0b01010101;
        let actual = Int64Type::mask_from_u64(mask);
        let expected = expected_mask!(i64, mask);
        let expected = m64x8::from_cast(i64x8::from_slice_unaligned(expected.as_slice()));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mask_f32() {
        let mask = 0b10101010_10101010;
        let actual = Float32Type::mask_from_u64(mask);
        let expected = expected_mask!(i32, mask);
        let expected =
            m32x16::from_cast(i32x16::from_slice_unaligned(expected.as_slice()));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mask_i32() {
        let mask = 0b01010101_01010101;
        let actual = Int32Type::mask_from_u64(mask);
        let expected = expected_mask!(i32, mask);
        let expected =
            m32x16::from_cast(i32x16::from_slice_unaligned(expected.as_slice()));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mask_u16() {
        let mask = 0b01010101_01010101_10101010_10101010;
        let actual = UInt16Type::mask_from_u64(mask);
        let expected = expected_mask!(i16, mask);
        dbg!(&expected);
        let expected =
            m16x32::from_cast(i16x32::from_slice_unaligned(expected.as_slice()));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mask_i8() {
        let mask =
            0b01010101_01010101_10101010_10101010_01010101_01010101_10101010_10101010;
        let actual = Int8Type::mask_from_u64(mask);
        let expected = expected_mask!(i8, mask);
        let expected = m8x64::from_cast(i8x64::from_slice_unaligned(expected.as_slice()));

        assert_eq!(expected, actual);
    }
}
