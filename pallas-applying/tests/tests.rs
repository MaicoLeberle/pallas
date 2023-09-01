use std::vec::Vec;
use rand::Rng;

use pallas_applying::{ProtocolParams, validate_byron_tx, ValidationError};
use pallas_codec::utils::{CborWrap, MaybeIndefArray, EmptyMap};
use pallas_crypto::hash::Hash;
use pallas_primitives::byron::{Attributes, Twit, Tx, TxId, TxIn, TxOut};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_ins() {
        let tx: Tx = mk_transaction(
            new_tx_ins(),
            new_tx_outs(),
            new_attributes()
        );
        let tx_wits: Witnesses = new_witnesses();
        let utxos: UTxOs = new_utxos();
        let protocol_params: ProtocolParams = new_protocol_params();

        match validate_byron_tx(&tx, &tx_wits, &utxos, protocol_params) {
            Ok(_) => assert!(false, "Inputs set cannot be empty."),
            Err(err) => match err {
                ValidationError::TxInsEmpty(_) => (),
                _                              => assert!(false, "Wrong error type."),
            }
        }
    }

    #[test]
    fn non_empty_ins() {
        let mut tx_ins: TxIns = new_tx_ins();
        insert_tx_in(&mut tx_ins, new_tx_in(random_tx_id(), 0));
        let tx: Tx = mk_transaction(
            tx_ins,
            new_tx_outs(),
            new_attributes()
        );
        let tx_wits: Witnesses = new_witnesses();
        let utxos: UTxOs = new_utxos();
        let protocol_params: ProtocolParams = new_protocol_params();

        match validate_byron_tx(&tx, &tx_wits, &utxos, protocol_params) {
            Ok(_)  => assert!(true),
            Err(_) => assert!(false, "Non-empty set of inputs but still failed.")
        }
    }
}

// Helper types.
type TxIns = MaybeIndefArray<TxIn>;
type TxOuts = MaybeIndefArray<TxOut>;
type Witnesses = MaybeIndefArray<Twit>;
type UTxOs = Vec<TxOut>;

// Helper functions.
fn new_tx_ins() -> TxIns {
    MaybeIndefArray::Def(Vec::new())
}

fn random_tx_id() -> TxId {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    for elem in bytes.iter_mut() {
        *elem = rng.gen();
    }
    Hash::new(bytes)
}

fn new_tx_in(tx_id: TxId, index: u32) -> TxIn {
    TxIn::Variant0(CborWrap((tx_id, index)))
}

fn insert_tx_in(ins: &mut TxIns, new_in: TxIn) {
    match ins {
        MaybeIndefArray::Def(vec) | MaybeIndefArray::Indef(vec) => vec.push(new_in)
    }
}

fn new_tx_outs() -> TxOuts {
    MaybeIndefArray::Def(Vec::new())
}

fn new_attributes() -> Attributes {
    EmptyMap
}

fn mk_transaction(ins: TxIns, outs: TxOuts, attrs: Attributes) -> Tx {
    Tx {
        inputs: ins,
        outputs: outs,
        attributes: attrs,
    }
}

fn new_witnesses() -> Witnesses {
    MaybeIndefArray::Def(Vec::new())
}

fn new_utxos() -> UTxOs {
    Vec::new()
}

fn new_protocol_params() -> ProtocolParams {
    ProtocolParams
}