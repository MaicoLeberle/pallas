use std::collections::HashMap;
use std::vec::Vec;
use rand::Rng;

use pallas_applying::{ProtocolParams, validate_byron_tx, ValidationError, UTxOs};
use pallas_codec::{minicbor::bytes::ByteVec, utils::{CborWrap, EmptyMap, MaybeIndefArray, TagWrap}};
use pallas_crypto::hash::Hash;
use pallas_primitives::byron::{Address, Attributes, Twit, Tx, TxId, TxIn, TxOut};


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
                _                              => assert!(false, "Wrong error type (empty_ins)."),
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
            Ok(_)    => assert!(true),
            Err(err) => match err {
                ValidationError::TxInsEmpty(_) =>
                    assert!(false, "Non-empty set of inputs but still failed."),
                _                              => assert!(true)
            }
        }
    }

    #[test]
    fn empty_outs() {
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
            Ok(_) => assert!(false, "Outputs set cannot be empty."),
            Err(err) => match err {
                ValidationError::TxOutsEmpty(_) => (),
                _                               => assert!(false, "Wrong error type (empty_outs)."),
            }
        }
    }

    #[test]
    fn non_empty_outs() {
        let mut tx_ins: TxIns = new_tx_ins();
        insert_tx_in(&mut tx_ins, new_tx_in(random_tx_id(), 0));
        let mut tx_outs: TxOuts = new_tx_outs();
        insert_tx_out(&mut tx_outs, new_tx_out(new_address(random_address_payload(), 0), 1000));
        let tx: Tx = mk_transaction(
            tx_ins,
            tx_outs,
            new_attributes()
        );
        let tx_wits: Witnesses = new_witnesses();
        let utxos: UTxOs = new_utxos();
        let protocol_params: ProtocolParams = new_protocol_params();

        match validate_byron_tx(&tx, &tx_wits, &utxos, protocol_params) {
            Ok(_)    => assert!(true),
            Err(err) => match err {
                ValidationError::TxOutsEmpty(_) =>
                    assert!(false, "Non-empty set of outputs but still failed."),
                _                              => assert!(true)
            }
        }
    }
}

// Helper types.
type TxIns = MaybeIndefArray<TxIn>;
type TxOuts = MaybeIndefArray<TxOut>;
type Witnesses = MaybeIndefArray<Twit>;

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

fn random_address_payload() -> TagWrap<ByteVec, 24> {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 24];
    for elem in bytes.iter_mut() {
        *elem = rng.gen();
    }
    TagWrap::<ByteVec, 24>::new(ByteVec::from(bytes.to_vec()))
}

fn new_address(payload: TagWrap<ByteVec, 24>, crc: u32) -> Address {
    Address {
        payload: payload,
        crc: crc,
    }
}

fn new_tx_out(address: Address, amount: u64) -> TxOut {
    TxOut {
        address: address,
        amount: amount,
    }
}

fn insert_tx_out(outs: &mut TxOuts, new_out: TxOut) {
    match outs {
        MaybeIndefArray::Def(vec) | MaybeIndefArray::Indef(vec) => vec.push(new_out)
    }
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
    HashMap::<TxIn, TxOut>::new()
}

fn new_protocol_params() -> ProtocolParams {
    ProtocolParams
}