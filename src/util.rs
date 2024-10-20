use std::fmt;

use elements::secp256k1_zkp as secp256k1;
use simfony::{elements, simplicity};
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

fn unspendable_internal_key() -> secp256k1::XOnlyPublicKey {
    secp256k1::XOnlyPublicKey::from_slice(&[
        0xf5, 0x91, 0x9f, 0xa6, 0x4c, 0xe4, 0x5f, 0x83, 0x06, 0x84, 0x90, 0x72, 0xb2, 0x6c, 0x1b,
        0xfd, 0xd2, 0x93, 0x7e, 0x6b, 0x81, 0x77, 0x47, 0x96, 0xff, 0x37, 0x2b, 0xd1, 0xeb, 0x53,
        0x62, 0xd2,
    ])
    .expect("key should be valid")
}

fn script_ver(cmr: simplicity::Cmr) -> (elements::Script, elements::taproot::LeafVersion) {
    let script = elements::script::Script::from(cmr.as_ref().to_vec());
    (script, simplicity::leaf_version())
}

fn taproot_spend_info(cmr: simplicity::Cmr) -> elements::taproot::TaprootSpendInfo {
    let builder = elements::taproot::TaprootBuilder::new();
    let (script, version) = script_ver(cmr);
    let builder = builder
        .add_leaf_with_ver(0, script, version)
        .expect("tap tree should be valid");
    builder
        .finalize(secp256k1::SECP256K1, unspendable_internal_key())
        .expect("tap tree should be valid")
}

pub fn liquid_testnet_address(cmr: simplicity::Cmr) -> elements::Address {
    let info = taproot_spend_info(cmr);
    let blinder = None;
    elements::Address::p2tr(
        secp256k1::SECP256K1,
        info.internal_key(),
        info.merkle_root(),
        blinder,
        &elements::AddressParams::LIQUID_TESTNET,
    )
}
