use std::fmt;

use simfony::simplicity;
use simplicity::dag::{DagLike, MaxSharing, NoSharing};
use simplicity::jet::Elements;
use simplicity::node::Inner;
use simplicity::{node, RedeemNode};

pub type Expression = RedeemNode<Elements>;

pub fn get_compression_factor<M: node::Marker>(node: &node::Node<M>) -> usize {
    let unshared_len = node.pre_order_iter::<NoSharing>().count();
    let shared_len = node.pre_order_iter::<MaxSharing<M>>().count();
    debug_assert!(0 < shared_len);
    debug_assert!(shared_len <= unshared_len);
    unshared_len / shared_len
}

pub struct DisplayInner<'a, M: node::Marker>(&'a node::Node<M>);

impl<'a, M: node::Marker> From<&'a node::Node<M>> for DisplayInner<'a, M> {
    fn from(node: &'a node::Node<M>) -> Self {
        Self(node)
    }
}

impl<'a, M: node::Marker> fmt::Display for DisplayInner<'a, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.inner() {
            Inner::Iden => f.write_str("iden"),
            Inner::Unit => f.write_str("unit"),
            Inner::InjL(_) => f.write_str("injl"),
            Inner::InjR(_) => f.write_str("injr"),
            Inner::Take(_) => f.write_str("take"),
            Inner::Drop(_) => f.write_str("drop"),
            Inner::Comp(_, _) => f.write_str("comp"),
            Inner::Case(_, _) => f.write_str("case"),
            Inner::AssertL(_, _) => f.write_str("assertl"),
            Inner::AssertR(_, _) => f.write_str("assertr"),
            Inner::Pair(_, _) => f.write_str("pair"),
            Inner::Disconnect(_, _) => f.write_str("disconnect"),
            Inner::Witness(_) => f.write_str("witness"),
            Inner::Fail(_) => f.write_str("fail"),
            Inner::Jet(jet) => write!(f, "jet_{}", jet),
            Inner::Word(value) => write!(f, "const {}", value),
        }
    }
}

impl<'a, M: node::Marker> fmt::Debug for DisplayInner<'a, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
