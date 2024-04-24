use std::fmt;
use std::sync::Arc;

use hex_conservative::DisplayHex;
use simplicity::dag::{Dag, DagLike, NoSharing};
use simplicity::types::Final;
use simplicity::Value;

use crate::simplicity;
use crate::util;

/// Immutable sequence of bits whose length is a power of two.
///
/// The sequence can be split in half to produce (pointers) to the front and to the rear.
///
/// All methods assume big Endian (of the implied byte sequence).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bits {
    bits: Arc<[bool]>,
    start: usize,
    len: usize,
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0b")?;

        for i in self.start..self.start + self.len {
            let bit = if self.bits[i] { '1' } else { '0' };
            write!(f, "{}", bit)?;
        }

        Ok(())
    }
}

impl Bits {
    pub fn from_bits<A: AsRef<[bool]>>(bits: A) -> Self {
        assert!(
            bits.as_ref().len().is_power_of_two(),
            "Length of bit sequence must be a power of two"
        );
        Self {
            len: bits.as_ref().len(),
            bits: Arc::from(bits.as_ref()),
            start: 0,
        }
    }

    pub fn from_bit(bit: bool) -> Self {
        Self {
            len: 1,
            bits: Arc::new([bit]),
            start: 0,
        }
    }

    pub fn from_byte(byte: u8) -> Self {
        let bits: Vec<bool> = (0..8).map(|i| byte & (1 << (7 - i)) != 0).collect();
        Self {
            bits: Arc::from(bits.as_ref()),
            start: 0,
            len: 8,
        }
    }

    pub fn split(&self) -> Option<(Self, Self)> {
        debug_assert!(self.len.is_power_of_two());
        if self.len == 1 {
            None
        } else {
            let left = Self {
                bits: self.bits.clone(),
                start: self.start,
                len: self.len / 2,
            };
            let right = Self {
                bits: self.bits.clone(),
                start: self.start + self.len / 2,
                len: self.len / 2,
            };
            Some((left, right))
        }
    }

    pub fn get_bit(&self) -> Option<bool> {
        if self.len == 1 {
            Some(self.bits[self.start])
        } else {
            None
        }
    }

    pub fn bit_length(&self) -> usize {
        self.len
    }

    pub fn iter_bits(&self) -> impl ExactSizeIterator<Item = bool> + '_ {
        self.bits.iter().copied()
    }
}

/// Immutable sequence of bytes whose length is a power of two.
///
/// The sequence can be split in half to produce (pointers) to the front and to the rear.
///
/// All methods assume big Endian.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bytes {
    bytes: Arc<[u8]>,
    start: usize,
    len: usize,
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "0x{}",
            DisplayHex::as_hex(&self.bytes[self.start..self.start + self.len])
        )
    }
}

impl Bytes {
    pub fn from_bytes<A: AsRef<[u8]>>(bytes: A) -> Self {
        assert!(
            bytes.as_ref().len().is_power_of_two(),
            "Length of byte sequence must be a power of 2"
        );
        Self {
            len: bytes.as_ref().len(),
            bytes: Arc::from(bytes.as_ref()),
            start: 0,
        }
    }

    pub fn byte_length(&self) -> usize {
        self.len
    }

    pub fn iter_bytes(&self) -> impl ExactSizeIterator<Item = u8> + '_ {
        self.bytes.iter().copied()
    }

    pub fn iter_bits(&self) -> impl Iterator<Item = bool> + '_ {
        self.iter_bytes()
            .flat_map(|byte| (0..8).map(move |i| byte & (1 << (7 - i)) != 0))
    }

    pub fn split(&self) -> Result<(Self, Self), (Bits, Bits)> {
        debug_assert!(self.len.is_power_of_two());
        if self.len == 1 {
            let byte = self.bytes[self.start];
            let bits = Bits::from_byte(byte);
            Err(bits.split().unwrap())
        } else {
            let left = Self {
                bytes: self.bytes.clone(),
                start: self.start,
                len: self.len / 2,
            };
            let right = Self {
                bytes: self.bytes.clone(),
                start: self.start + self.len / 2,
                len: self.len / 2,
            };
            Ok((left, right))
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum ExtValue {
    Unit,
    Left(Arc<Self>),
    Right(Arc<Self>),
    Product(Arc<Self>, Arc<Self>),
    Bits(Bits),
    Bytes(Bytes),
}

impl fmt::Display for ExtValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for data in self.verbose_pre_order_iter::<NoSharing>() {
            match data.node {
                ExtValue::Unit => f.write_str("●")?,
                ExtValue::Left(..) => {
                    match data.n_children_yielded {
                        0 => f.write_str("L")?,
                        _ => continue,
                    };
                }
                ExtValue::Right(..) => match data.n_children_yielded {
                    0 => {
                        f.write_str("R")?;
                    }
                    _ => continue,
                },
                ExtValue::Product(..) => match data.n_children_yielded {
                    0 => f.write_str("(")?,
                    1 => f.write_str(", ")?,
                    _ => f.write_str(")")?,
                },
                ExtValue::Bits(bits) => write!(f, "{}", bits)?,
                ExtValue::Bytes(bytes) => write!(f, "{}", bytes)?,
            }
        }

        Ok(())
    }
}

impl fmt::Debug for ExtValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl ExtValue {
    pub fn unit() -> Arc<Self> {
        Arc::new(Self::Unit)
    }

    pub fn left(left: Arc<Self>) -> Arc<Self> {
        Arc::new(Self::Left(left))
    }

    pub fn right(right: Arc<Self>) -> Arc<Self> {
        Arc::new(Self::Right(right))
    }

    pub fn product(left: Arc<Self>, right: Arc<Self>) -> Arc<Self> {
        Arc::new(Self::Product(left, right))
    }

    pub fn bits(bits: Bits) -> Arc<Self> {
        Arc::new(Self::Bits(bits))
    }

    pub fn bytes(bytes: Bytes) -> Arc<Self> {
        Arc::new(Self::Bytes(bytes))
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, ExtValue::Unit)
    }

    pub fn to_left(&self) -> Option<Arc<Self>> {
        match self {
            Self::Left(left) => Some(left.clone()),
            Self::Bits(bits) => bits.get_bit().and_then(|b| (!b).then(ExtValue::unit)),
            _ => None,
        }
    }

    pub fn to_right(&self) -> Option<Arc<Self>> {
        match self {
            Self::Right(right) => Some(right.clone()),
            Self::Bits(bits) => bits.get_bit().and_then(|b| b.then(ExtValue::unit)),
            _ => None,
        }
    }

    pub fn to_product(&self) -> Option<(Arc<Self>, Arc<Self>)> {
        match self {
            Self::Product(left, right) => Some((left.clone(), right.clone())),
            Self::Bits(bits) => bits
                .split()
                .map(|(left, right)| (Self::bits(left), Self::bits(right))),
            Self::Bytes(bytes) => match bytes.split() {
                Ok((left, right)) => Some((Self::bytes(left), Self::bytes(right))),
                Err((left, right)) => Some((Self::bits(left), Self::bits(right))),
            },
            _ => None,
        }
    }

    pub fn bit_width(&self) -> usize {
        self.pre_order_iter::<NoSharing>()
            .map(|inner| match inner {
                ExtValue::Unit | ExtValue::Product(..) => 0,
                ExtValue::Left(..) | ExtValue::Right(..) => 1,
                ExtValue::Bits(bits) => bits.bit_length(),
                ExtValue::Bytes(bytes) => bytes.byte_length() * 8,
            })
            .sum()
    }

    pub fn iter_bits(&self) -> impl Iterator<Item = bool> + '_ {
        self.pre_order_iter::<NoSharing>()
            .flat_map(|inner| match inner {
                ExtValue::Unit | ExtValue::Product(..) => {
                    Box::new(std::iter::empty()) as Box<dyn Iterator<Item = bool>>
                }
                ExtValue::Left(..) => Box::new(std::iter::once(false)),
                ExtValue::Right(..) => Box::new(std::iter::once(true)),
                ExtValue::Bits(bits) => Box::new(bits.iter_bits()),
                ExtValue::Bytes(bytes) => Box::new(bytes.iter_bits()),
            })
    }

    pub fn iter_bits_with_padding(self: Arc<Self>, ty: &Final) -> impl Iterator<Item = bool> {
        let mut bits = Vec::with_capacity(ty.bit_width());
        let mut stack = vec![(self, ty)];

        while let Some((value, ty)) = stack.pop() {
            if ty.is_unit() {
                assert!(
                    value.is_unit(),
                    "Value {value} is not of expected type unit"
                );
            } else if let Some((l_ty, r_ty)) = ty.as_sum() {
                if let Some(l_value) = value.to_left() {
                    bits.push(false);
                    bits.extend(std::iter::repeat(false).take(util::pad_left(l_ty, r_ty)));
                    stack.push((l_value, l_ty));
                } else if let Some(r_value) = value.to_right() {
                    bits.push(true);
                    bits.extend(std::iter::repeat(false).take(util::pad_right(l_ty, r_ty)));
                    stack.push((r_value, r_ty));
                } else {
                    panic!("Value {value} is not of expected type {ty}");
                }
            } else if let Some((l_ty, r_ty)) = ty.as_product() {
                if let Some((l_value, r_value)) = value.to_product() {
                    stack.push((r_value, r_ty));
                    stack.push((l_value, l_ty));
                } else {
                    panic!("Value {value} is not of expected type {ty}");
                }
            }
        }

        bits.into_iter()
    }
}

fn bits_to_byte<A: AsRef<[bool]>>(bits: A) -> u8 {
    assert_eq!(
        bits.as_ref().len(),
        8,
        "Length of bit sequence must be exactly 8"
    );

    let mut byte: u8 = 0;

    for bit in bits.as_ref().iter().copied() {
        byte = byte << 1 | if bit { 1 } else { 0 };
    }

    byte
}

/// Stack item for conversions of bits to `ExtValue`.
enum Item {
    Value(ExtValue),
    Bits(Vec<bool>),
    Bytes(Vec<u8>),
}

impl Item {
    pub fn into_extvalue(self) -> ExtValue {
        match self {
            Item::Value(left) => left,
            Item::Bits(left) => ExtValue::Bits(Bits::from_bits(left)),
            Item::Bytes(left) => ExtValue::Bytes(Bytes::from_bytes(left)),
        }
    }

    pub fn into_left(self) -> Self {
        match self {
            Item::Value(ExtValue::Unit) => Item::Bits(vec![false]),
            _ => Item::Value(ExtValue::Left(Arc::new(self.into_extvalue()))),
        }
    }

    pub fn into_right(self) -> Self {
        match self {
            Item::Value(ExtValue::Unit) => Item::Bits(vec![true]),
            _ => Item::Value(ExtValue::Right(Arc::new(self.into_extvalue()))),
        }
    }

    pub fn into_product(self, left: Self) -> Self {
        match (self, left) {
            (Item::Bits(r_bits), Item::Bits(mut l_bits)) if r_bits.len() == l_bits.len() => {
                l_bits.extend(r_bits);
                if l_bits.len() == 8 {
                    Item::Bytes(vec![bits_to_byte(l_bits)])
                } else {
                    Item::Bits(l_bits)
                }
            }
            (Item::Bytes(r_bytes), Item::Bytes(mut l_bytes)) if r_bytes.len() == l_bytes.len() => {
                l_bytes.extend(r_bytes);
                Item::Bytes(l_bytes)
            }
            (right, left) => {
                let left = Arc::new(left.into_extvalue());
                let right = Arc::new(right.into_extvalue());
                Item::Value(ExtValue::Product(left, right))
            }
        }
    }
}

impl ExtValue {
    // FIXME: Take &Final
    // Requires split_{sum,product} method of Final that returns references
    pub fn from_bits_with_padding<I: Iterator<Item = bool>>(
        ty: &Final,
        it: &mut I,
    ) -> Result<Arc<Self>, &'static str> {
        enum Task<'a> {
            ReadType(&'a Final),
            MakeLeft,
            MakeRight,
            MakeProduct,
        }

        let mut task_stack = vec![Task::ReadType(ty)];
        let mut result_stack = vec![];

        while let Some(task) = task_stack.pop() {
            match task {
                Task::ReadType(ty) => {
                    if ty.is_unit() {
                        result_stack.push(Item::Value(ExtValue::Unit));
                    } else if let Some((l_ty, r_ty)) = ty.as_sum() {
                        if !it.next().ok_or("Not enough bits")? {
                            for _ in 0..util::pad_left(l_ty, r_ty) {
                                let _padding = it.next().ok_or("Not enough bits")?;
                            }
                            task_stack.push(Task::MakeLeft);
                            task_stack.push(Task::ReadType(l_ty));
                        } else {
                            for _ in 0..util::pad_right(l_ty, r_ty) {
                                let _padding = it.next().ok_or("Not enough bits")?;
                            }
                            task_stack.push(Task::MakeRight);
                            task_stack.push(Task::ReadType(r_ty));
                        }
                    } else if let Some((l_ty, r_ty)) = ty.as_product() {
                        task_stack.push(Task::MakeProduct);
                        task_stack.push(Task::ReadType(r_ty));
                        task_stack.push(Task::ReadType(l_ty));
                    }
                }
                // borrowck at its best :/
                Task::MakeLeft => {
                    let tmp = result_stack.pop().unwrap().into_left();
                    result_stack.push(tmp);
                }
                Task::MakeRight => {
                    let tmp = result_stack.pop().unwrap().into_right();
                    result_stack.push(tmp);
                }
                Task::MakeProduct => {
                    let tmp = result_stack
                        .pop()
                        .unwrap()
                        .into_product(result_stack.pop().unwrap());
                    result_stack.push(tmp);
                }
            }
        }

        debug_assert!(result_stack.len() == 1);
        Ok(Arc::new(result_stack.pop().unwrap().into_extvalue()))
    }
}

impl<'a> DagLike for &'a ExtValue {
    type Node = Self;

    fn data(&self) -> &Self::Node {
        self
    }

    fn as_dag_node(&self) -> Dag<Self> {
        match self {
            ExtValue::Unit | ExtValue::Bits(..) | ExtValue::Bytes(..) => Dag::Nullary,
            ExtValue::Left(child) | ExtValue::Right(child) => Dag::Unary(child),
            ExtValue::Product(left, right) => Dag::Binary(left, right),
        }
    }
}

// I would like to implement for Arc<ExtValue> but I can't
impl<'a> From<&'a Value> for ExtValue {
    fn from(value: &'a Value) -> Self {
        let mut stack = vec![];
        for data in value.post_order_iter::<NoSharing>() {
            match data.node {
                Value::Unit => stack.push(Item::Value(ExtValue::Unit)),
                Value::SumL(..) => {
                    let tmp = stack.pop().unwrap().into_left();
                    stack.push(tmp)
                }
                Value::SumR(..) => {
                    let tmp = stack.pop().unwrap().into_right();
                    stack.push(tmp);
                }
                Value::Prod(..) => {
                    let tmp = stack.pop().unwrap().into_product(stack.pop().unwrap());
                    stack.push(tmp);
                }
            }
        }

        debug_assert!(stack.len() == 1);
        stack.pop().unwrap().into_extvalue()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simplicity;
    use simplicity::jet::type_name::TypeName;
    use simplicity::Cmr;

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn split_bits() {
        let bits = Bits::from_byte(0b01101111);
        assert_eq!("0b01101111", bits.to_string().as_str());

        let (a, b) = bits.split().unwrap();
        assert_eq!("0b0110", a.to_string().as_str());
        assert_eq!("0b1111", b.to_string().as_str());

        let (c, d) = a.split().unwrap();
        assert_eq!("0b01", c.to_string().as_str());
        assert_eq!("0b10", d.to_string().as_str());

        let (e, f) = c.split().unwrap();
        assert_eq!("0b0", e.to_string().as_str());
        assert_eq!("0b1", f.to_string().as_str());

        assert!(e.split().is_none());
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn get_bit() {
        assert_eq!(Some(false), Bits::from_bit(false).get_bit());
        assert_eq!(Some(true), Bits::from_bit(true).get_bit());
        assert_eq!(None, Bits::from_bits(vec![false, false]).get_bit());
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn split_bytes() {
        let bytes = Bytes::from_bytes(vec![0b01101111, 0xff, 0xff, 0xff]);
        assert_eq!("0x6fffffff", bytes.to_string().as_str());

        let (a, b) = bytes.split().unwrap();
        assert_eq!("0x6fff", a.to_string().as_str());
        assert_eq!("0xffff", b.to_string().as_str());

        let (c, d) = a.split().unwrap();
        assert_eq!("0x6f", c.to_string().as_str());
        assert_eq!("0xff", d.to_string().as_str());

        let (e, f) = c.split().unwrap_err();
        assert_eq!("0b0110", e.to_string().as_str());
        assert_eq!("0b1111", f.to_string().as_str());
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn display_value() {
        let value = ExtValue::product(
            ExtValue::left(ExtValue::right(ExtValue::unit())),
            ExtValue::product(
                ExtValue::bits(Bits::from_byte(0b01101111)),
                ExtValue::bytes(Bytes::from_bytes(vec![0xde, 0xad, 0xbe, 0xef])),
            ),
        );
        assert_eq!(
            "(LR●, (0b01101111, 0xdeadbeef))",
            value.to_string().as_str()
        );
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn iter_bits() {
        let value_bits = [
            (ExtValue::unit(), vec![]),
            (ExtValue::left(ExtValue::unit()), vec![false]),
            (ExtValue::right(ExtValue::unit()), vec![true]),
            (
                ExtValue::bits(Bits::from_bits(vec![false, true])),
                vec![false, true],
            ),
            (
                ExtValue::bytes(Bytes::from_bytes(vec![0b01011111])),
                vec![false, true, false, true, true, true, true, true],
            ),
        ];

        for (value, expected_bits) in value_bits {
            let bits: Vec<_> = value.iter_bits().collect();
            assert_eq!(bits, expected_bits);
        }
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn extvalue_from_value() {
        let output_input = vec![
            (ExtValue::unit(), Value::unit()),
            (
                ExtValue::bits(Bits::from_bit(false)),
                Value::sum_l(Value::unit()),
            ),
            (
                ExtValue::bits(Bits::from_bit(true)),
                Value::sum_r(Value::unit()),
            ),
            (
                ExtValue::left(ExtValue::bits(Bits::from_bit(true))),
                Value::sum_l(Value::sum_r(Value::unit())),
            ),
            (
                ExtValue::product(ExtValue::unit(), ExtValue::unit()),
                Value::prod(Value::unit(), Value::unit()),
            ),
            (
                ExtValue::bytes(Bytes::from_bytes(Cmr::unit())),
                Value::u256_from_slice(Cmr::unit().as_ref()),
            ),
            (
                ExtValue::bytes(Bytes::from_bytes(vec![0b01010101])),
                Value::u8(0b01010101),
            ),
            (
                ExtValue::bytes(Bytes::from_bytes(vec![0xab, 0xcd])),
                Value::prod(Value::u8(0xab), Value::u8(0xcd)),
            ),
        ];

        for (expected_output, input) in output_input {
            assert_eq!(expected_output.as_ref(), &ExtValue::from(input.as_ref()));
        }
    }

    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn bits_iter_from_bits_roundtrip() {
        let value_typename = [
            (ExtValue::unit(), TypeName(b"1")),
            (
                ExtValue::left(ExtValue::bits(Bits::from_bit(false))),
                TypeName(b"+21"),
            ),
            (
                ExtValue::right(ExtValue::bits(Bits::from_bit(true))),
                TypeName(b"+12"),
            ),
            (
                ExtValue::product(ExtValue::unit(), ExtValue::unit()),
                TypeName(b"*11"),
            ),
            (
                ExtValue::bits(Bits::from_bits(vec![false, true])),
                TypeName(b"*22"),
            ),
            (
                ExtValue::bytes(Bytes::from_bytes(Cmr::unit())),
                TypeName(b"h"),
            ),
        ];

        for (value, typename) in value_typename {
            let ty = typename.to_final();
            let mut bits = value.clone().iter_bits_with_padding(&ty);
            let value_from_bits = ExtValue::from_bits_with_padding(&ty, &mut bits).unwrap();
            assert_eq!(value, value_from_bits);
        }
    }
}
