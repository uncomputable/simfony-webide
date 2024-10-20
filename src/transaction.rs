use std::sync::Arc;

use elements::confidential;
use elements::hashes::Hash;
use simfony::{elements, simplicity};
use simplicity::jet::elements::{ElementsEnv, ElementsUtxo};

use crate::util;

#[derive(Clone, Debug)]
pub struct TxParams {
    pub txid: elements::Txid,
    pub vout: u32,
    pub recipient_address: Option<elements::Address>,
    pub fee: u64,
    pub lock_time: elements::LockTime,
    pub sequence: elements::Sequence,
}

impl Default for TxParams {
    fn default() -> Self {
        Self {
            txid: elements::Txid::all_zeros(),
            vout: 0,
            recipient_address: None,
            fee: 0,
            lock_time: elements::LockTime::from_consensus(0),
            sequence: elements::Sequence::from_consensus(0),
        }
    }
}

impl TxParams {
    fn unsatisfied_transaction(&self) -> elements::Transaction {
        elements::Transaction {
            version: 2,
            lock_time: self.lock_time,
            input: vec![elements::TxIn {
                previous_output: elements::OutPoint {
                    txid: self.txid,
                    vout: self.vout,
                },
                is_pegin: false,
                script_sig: elements::Script::new(),
                sequence: self.sequence,
                asset_issuance: elements::AssetIssuance::null(),
                witness: elements::TxInWitness::empty(), // not required here
            }],
            output: vec![
                elements::TxOut {
                    asset: confidential::Asset::Explicit(util::liquid_testnet_bitcoin_asset()),
                    value: confidential::Value::Explicit(100_000u64.saturating_sub(self.fee)),
                    nonce: confidential::Nonce::Null,
                    script_pubkey: self
                        .recipient_address
                        .as_ref()
                        .map(elements::Address::script_pubkey)
                        .unwrap_or_default(),
                    witness: elements::TxOutWitness::empty(),
                },
                elements::TxOut::new_fee(self.fee, util::liquid_testnet_bitcoin_asset()),
            ],
        }
    }

    fn utxo(&self, script_pubkey: elements::Script) -> ElementsUtxo {
        ElementsUtxo {
            script_pubkey,
            asset: confidential::Asset::Explicit(util::liquid_testnet_bitcoin_asset()),
            value: confidential::Value::Explicit(100_000),
        }
    }

    pub fn tx_env(&self, cmr: simplicity::Cmr) -> ElementsEnv<Arc<elements::Transaction>> {
        let index = 0;
        let (script_pubkey, control_block) = util::script_control_block(cmr);
        let annex = None;
        ElementsEnv::new(
            Arc::new(self.unsatisfied_transaction()),
            vec![self.utxo(script_pubkey)],
            index,
            cmr,
            control_block,
            annex,
            util::liquid_testnet_genesis(),
        )
    }
}
