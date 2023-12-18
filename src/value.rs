use std::fmt;
use std::sync::Arc;

use hex_conservative::DisplayHex;
use simplicity::dag::{Dag, DagLike, NoSharing};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bits {
    bits: Arc<Vec<bool>>,
    start: usize,
    len: usize,
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0b")?;

        for i in self.start..self.start + self.len {
            let bit = if self.bits[7 - i] { '1' } else { '0' };
            write!(f, "{}", bit)?;
        }

        Ok(())
    }
}

impl Bits {
    pub fn from_bits(bits: Vec<bool>) -> Self {
        assert!(
            bits.len().is_power_of_two(),
            "Length of bit sequence must be a power of two"
        );
        Self {
            len: bits.len(),
            bits: Arc::new(bits),
            start: 0,
        }
    }

    pub fn from_bit(bit: bool) -> Self {
        Self {
            len: 1,
            bits: Arc::new(vec![bit]),
            start: 0,
        }
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            bits: Arc::new((0..8).map(|i| byte & (1 << i) != 0).collect()),
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
        debug_assert!(self.len.is_power_of_two());
        if self.len == 1 {
            Some(self.bits[self.start])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Bytes {
    bytes: Arc<Vec<u8>>,
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
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        assert!(
            bytes.len().is_power_of_two(),
            "Length of byte sequence must be a power of two"
        );
        Self {
            len: bytes.len(),
            bytes: Arc::new(bytes),
            start: 0,
        }
    }
}

impl Bytes {
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

    pub fn split_left(&self) -> Option<Arc<Self>> {
        match self {
            Self::Left(left) => Some(left.clone()),
            Self::Bits(bits) => bits.get_bit().and_then(|b| (!b).then(ExtValue::unit)),
            _ => None,
        }
    }

    pub fn split_right(&self) -> Option<Arc<Self>> {
        match self {
            Self::Right(right) => Some(right.clone()),
            Self::Bits(bits) => bits.get_bit().and_then(|b| b.then(ExtValue::unit)),
            _ => None,
        }
    }

    pub fn split_product(&self) -> Option<(Arc<Self>, Arc<Self>)> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
    fn get_bit() {
        assert_eq!(Some(false), Bits::from_bit(false).get_bit());
        assert_eq!(Some(true), Bits::from_bit(true).get_bit());
        assert_eq!(None, Bits::from_bits(vec![false, false]).get_bit());
    }

    #[test]
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
}
