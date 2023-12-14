use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use simplicity::dag::{Dag, DagLike, NoSharing};
use simplicity::jet::Elements;
use simplicity::node::Inner;
use simplicity::{RedeemNode, Value};

pub fn value_to_bitstring(value: &Value) -> Vec<bool> {
    let mut bitstring = Vec::new();
    value.do_each_bit(|bit| {
        bitstring.push(bit);
    });
    bitstring
}

pub fn bytes_to_bitstring<A: AsRef<[u8]>>(bytes: A) -> Vec<bool> {
    let mut bitstring = Vec::new();
    for byte in bytes.as_ref() {
        for i in (0..8).rev() {
            let bit = byte & (1 << i) != 0;
            bitstring.push(bit);
        }
    }
    bitstring
}

pub fn fmt_bitstring(bitstring: &[bool]) -> String {
    let mut s = "".to_string();
    for bit in bitstring {
        s.push_str(if !bit { "0" } else { "1" });
    }
    s
}

pub fn parse_bitstring(s: &str) -> Result<Vec<bool>, String> {
    let mut bitstring = Vec::new();
    for c in s.chars() {
        match c {
            '0' => bitstring.push(false),
            '1' => bitstring.push(true),
            _ => return Err(format!("Illegal character: {c}")),
        }
    }
    Ok(bitstring)
}

pub fn program_from_string(s: &str) -> Result<Arc<RedeemNode<Elements>>, String> {
    let empty_witness = HashMap::new();
    let forest = simplicity::human_encoding::Forest::parse(s).map_err(|e| e.to_string())?;
    forest
        .to_witness_node(&empty_witness)
        .ok_or("Main root is missing".to_string())?
        .finalize()
        .map_err(|e| e.to_string())
}

#[derive(Clone, Eq, PartialEq)]
pub struct Expression(pub Arc<RedeemNode<Elements>>);

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for data in self.0.clone().verbose_pre_order_iter::<NoSharing>() {
            match data.n_children_yielded {
                1 => {
                    match data.node.inner().as_dag() {
                        Dag::Nullary => {}
                        Dag::Unary(..) => f.write_str(")")?,
                        Dag::Binary(..) => f.write_str(" ")?,
                    }
                    continue;
                }
                2 => {
                    debug_assert!(matches!(data.node.inner().as_dag(), Dag::Binary(..)));
                    f.write_str(")")?;
                    continue;
                }
                n => {
                    debug_assert!(n == 0, "Combinators are nullary, unary or binary");
                }
            };

            // FIXME: Print assert{l,r} hashes
            // FIXME: Print fail entropy?
            // FIXME: Print witness value??
            match data.node.inner() {
                Inner::Iden => f.write_str("iden")?,
                Inner::Unit => f.write_str("unit")?,
                Inner::InjL(_) => f.write_str("injl (")?,
                Inner::InjR(_) => f.write_str("injr (")?,
                Inner::Take(_) => f.write_str("take (")?,
                Inner::Drop(_) => f.write_str("drop (")?,
                Inner::Comp(_, _) => f.write_str("comp (")?,
                Inner::Case(_, _) => f.write_str("case (")?,
                Inner::AssertL(_, _) => f.write_str("assertl (")?,
                Inner::AssertR(_, _) => f.write_str("assertl (")?,
                Inner::Pair(_, _) => f.write_str("pair")?,
                Inner::Disconnect(_, _) => f.write_str("disconnect (")?,
                Inner::Witness(_) => f.write_str("witness")?,
                Inner::Fail(_) => f.write_str("fail")?,
                Inner::Jet(jet) => write!(f, "jet_{}", jet)?,
                Inner::Word(value) => {
                    let bitstring = value_to_bitstring(&value);
                    write!(f, "const {}", fmt_bitstring(&bitstring))?;
                }
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
