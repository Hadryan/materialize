// Copyright 2019 Timely Data, Inc. All rights reserved.
//
// This file is part of Materialize. Materialize may not be used or
// distributed without the express permission of Timely Data, Inc.

use serde::{Deserialize, Serialize};

use crate::repr::Datum;

pub fn and(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_bool() && b.unwrap_bool())
}

pub fn or(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_bool() || b.unwrap_bool())
}

pub fn not(a: Datum) -> Datum {
    Datum::from(!a.unwrap_bool())
}

pub fn add_int32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int32() + b.unwrap_int32())
}

pub fn add_int64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int64() + b.unwrap_int64())
}

pub fn add_float32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float32() + b.unwrap_float32())
}

pub fn add_float64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float64() + b.unwrap_float64())
}

pub fn sub_int32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int32() - b.unwrap_int32())
}

pub fn sub_int64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int64() - b.unwrap_int64())
}

pub fn sub_float32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float32() - b.unwrap_float32())
}

pub fn sub_float64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float64() - b.unwrap_float64())
}

pub fn mul_int32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int32() * b.unwrap_int32())
}

pub fn mul_int64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int64() * b.unwrap_int64())
}

pub fn mul_float32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float32() * b.unwrap_float32())
}

pub fn mul_float64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float64() * b.unwrap_float64())
}

pub fn div_int32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int32() / b.unwrap_int32())
}

pub fn div_int64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int64() / b.unwrap_int64())
}

pub fn div_float32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float32() / b.unwrap_float32())
}

pub fn div_float64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float64() / b.unwrap_float64())
}

pub fn mod_int32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int32() % b.unwrap_int32())
}

pub fn mod_int64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_int64() % b.unwrap_int64())
}

pub fn mod_float32(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float32() % b.unwrap_float32())
}

pub fn mod_float64(a: Datum, b: Datum) -> Datum {
    Datum::from(a.unwrap_float64() % b.unwrap_float64())
}

pub fn neg_int32(a: Datum) -> Datum {
    Datum::from(-a.unwrap_int32())
}

pub fn neg_int64(a: Datum) -> Datum {
    Datum::from(-a.unwrap_int64())
}

pub fn neg_float32(a: Datum) -> Datum {
    Datum::from(-a.unwrap_float32())
}

pub fn neg_float64(a: Datum) -> Datum {
    Datum::from(-a.unwrap_float64())
}

pub fn eq(a: Datum, b: Datum) -> Datum {
    Datum::from(a == b)
}

pub fn lt(a: Datum, b: Datum) -> Datum {
    Datum::from(a < b)
}

pub fn lte(a: Datum, b: Datum) -> Datum {
    Datum::from(a <= b)
}

pub fn gt(a: Datum, b: Datum) -> Datum {
    Datum::from(a > b)
}

pub fn gte(a: Datum, b: Datum) -> Datum {
    Datum::from(a >= b)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BinaryFunc {
    And,
    Or,
    AddInt32,
    AddInt64,
    AddFloat32,
    AddFloat64,
    SubInt32,
    SubInt64,
    SubFloat32,
    SubFloat64,
    MulInt32,
    MulInt64,
    MulFloat32,
    MulFloat64,
    DivInt32,
    DivInt64,
    DivFloat32,
    DivFloat64,
    ModInt32,
    ModInt64,
    ModFloat32,
    ModFloat64,
    Eq,
    Lt,
    Lte,
    Gt,
    Gte,
}

impl BinaryFunc {
    pub fn func(self) -> fn(Datum, Datum) -> Datum {
        match self {
            BinaryFunc::And => and,
            BinaryFunc::Or => or,
            BinaryFunc::AddInt32 => add_int32,
            BinaryFunc::AddInt64 => add_int64,
            BinaryFunc::AddFloat32 => add_float32,
            BinaryFunc::AddFloat64 => add_float64,
            BinaryFunc::SubInt32 => sub_int32,
            BinaryFunc::SubInt64 => sub_int64,
            BinaryFunc::SubFloat32 => sub_float32,
            BinaryFunc::SubFloat64 => sub_float64,
            BinaryFunc::MulInt32 => mul_int32,
            BinaryFunc::MulInt64 => mul_int64,
            BinaryFunc::MulFloat32 => mul_float32,
            BinaryFunc::MulFloat64 => mul_float64,
            BinaryFunc::DivInt32 => div_int32,
            BinaryFunc::DivInt64 => div_int64,
            BinaryFunc::DivFloat32 => div_float32,
            BinaryFunc::DivFloat64 => div_float64,
            BinaryFunc::ModInt32 => mod_int32,
            BinaryFunc::ModInt64 => mod_int64,
            BinaryFunc::ModFloat32 => mod_float32,
            BinaryFunc::ModFloat64 => mod_float64,
            BinaryFunc::Eq => eq,
            BinaryFunc::Lt => lt,
            BinaryFunc::Lte => lte,
            BinaryFunc::Gt => gt,
            BinaryFunc::Gte => gte,
        }
    }
}

pub fn is_null(a: Datum) -> Datum {
    Datum::from(a == Datum::Null)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UnaryFunc {
    Not,
    IsNull,
    NegInt32,
    NegInt64,
    NegFloat32,
    NegFloat64,
}

impl UnaryFunc {
    pub fn func(self) -> fn(Datum) -> Datum {
        match self {
            UnaryFunc::Not => not,
            UnaryFunc::IsNull => is_null,
            UnaryFunc::NegInt32 => neg_int32,
            UnaryFunc::NegInt64 => neg_int64,
            UnaryFunc::NegFloat32 => neg_float32,
            UnaryFunc::NegFloat64 => neg_float64,
        }
    }
}

pub fn sum_int32<I>(datums: I) -> Datum where I: IntoIterator<Item = Datum> {
    let x: i32 = datums.into_iter().map(|d| d.unwrap_int32()).sum();
    Datum::from(x)
}

pub fn sum_int64<I>(datums: I) -> Datum where I: IntoIterator<Item = Datum> {
    let x: i64 = datums.into_iter().map(|d| d.unwrap_int64()).sum();
    Datum::from(x)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AggregateFunc {
    AvgInt32,
    AvgInt64,
    AvgFloat32,
    AvgFloat64,
    MaxInt32,
    MaxInt64,
    MaxFloat32,
    MaxFloat64,
    MinInt32,
    MinInt64,
    MinFloat32,
    MinFloat64,
    SumInt32,
    SumInt64,
    SumFloat32,
    SumFloat64,
    Count,
}

impl AggregateFunc {
    pub fn func<I>(self) -> fn(I) -> Datum where I: IntoIterator<Item = Datum> {
        match self {
            AggregateFunc::SumInt32 => sum_int32,
            AggregateFunc::SumInt64 => sum_int64,
            _ => unimplemented!(),
        }
    }
}