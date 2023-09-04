use std::vec::Vec;
use rand::Rng;

use pallas_applying::{ProtocolParams, to_utxo_tx_in, validate_byron_tx, ValidationError, UTxOs};
use pallas_codec::{minicbor::bytes::ByteVec, utils::{CborWrap, EmptyMap, MaybeIndefArray, TagWrap}};
use pallas_crypto::hash::Hash;
use pallas_primitives::byron::{Address, Attributes, Twit, Tx, TxId, TxIn, TxOut};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_case() {
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 0);
        insert_tx_in(&mut tx_ins, &tx_in);
        let mut tx_outs: TxOuts = new_tx_outs();
        let tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 1000);
        insert_tx_out(&mut tx_outs, &tx_out);
        let tx: Tx = mk_transaction(
            tx_ins,
            tx_outs,
            new_attributes()
        );
        let tx_wits: Witnesses = new_witnesses();
        let mut utxos: UTxOs = new_utxos();
        insert_utxo(
            &mut utxos,
            &tx_in,
            new_tx_out(new_address(random_address_payload(), 0), 1000)
        );
        let protocol_params: ProtocolParams = new_protocol_params();

        match validate_byron_tx(&tx, &tx_wits, &utxos, protocol_params) {
            Ok(_)    => (),
            Err(err) => assert!(false, "ERROR: {:?}", err),
            }
    }

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
                ValidationError::TxInsEmpty => (),
                wrong_error                 =>
                    assert!(false, "Wrong error type (empty_ins - {:?}).", wrong_error),
            }
        }
    }

    #[test]
    fn empty_outs() {
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 0);
        insert_tx_in(&mut tx_ins, &tx_in);
        let tx: Tx = mk_transaction(
            tx_ins,
            new_tx_outs(),
            new_attributes()
        );
        let tx_wits: Witnesses = new_witnesses();
        let mut utxos: UTxOs = new_utxos();
        insert_utxo(
            &mut utxos,
            &tx_in,
            new_tx_out(new_address(random_address_payload(), 0), 1000)
        );
        let protocol_params: ProtocolParams = new_protocol_params();

        match validate_byron_tx(&tx, &tx_wits, &utxos, protocol_params) {
            Ok(_) => assert!(false, "Outputs set cannot be empty."),
            Err(err) => match err {
                ValidationError::TxOutsEmpty => (),
                wrong_error                  =>
                    assert!(false, "Wrong error type (empty_outs - {:?}).", wrong_error),
            }
        }
    }

    #[test]
    fn unfound_utxo() {
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 0);
        insert_tx_in(&mut tx_ins, &tx_in);
        let mut tx_outs: TxOuts = new_tx_outs();
        let tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 1000);
        insert_tx_out(&mut tx_outs, &tx_out);
        let tx: Tx = mk_transaction(
            tx_ins,
            tx_outs,
            new_attributes()
        );
        let tx_wits: Witnesses = new_witnesses();
        let utxos: UTxOs = new_utxos();
        let protocol_params: ProtocolParams = new_protocol_params();

        match validate_byron_tx(&tx, &tx_wits, &utxos, protocol_params) {
            Ok(_)    => assert!(false, "Input must be in the set of UTxOs."),
            Err(err) => match err {
                ValidationError::InputMissingFromUTxO => (),
                wrong_error                           =>
                    assert!(false, "Wrong error type (unfound_utxo - {:?}).", wrong_error),
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

fn insert_tx_in(ins: &mut TxIns, new_in: &TxIn) {
    match ins {
        MaybeIndefArray::Def(vec) | MaybeIndefArray::Indef(vec) => vec.push(new_in.clone())
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

fn insert_tx_out(outs: &mut TxOuts, new_out: &TxOut) {
    match outs {
        MaybeIndefArray::Def(vec) | MaybeIndefArray::Indef(vec) => vec.push(new_out.clone())
    }
}

fn insert_utxo(utxos: &mut UTxOs, tx_in: &TxIn, tx_out: TxOut) -> Option<TxOut> {
    match to_utxo_tx_in(tx_in) {
        Some(utxo_tx_in) => utxos.insert(utxo_tx_in, tx_out),
        None => None
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
    UTxOs::new()
}

fn new_protocol_params() -> ProtocolParams {
    ProtocolParams
}