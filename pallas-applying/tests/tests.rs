use std::vec::Vec;
use rand::Rng;

use pallas_applying::{
    annotate_tx,
    ProtocolParams,
    to_utxo_tx_in,
    validate_byron_tx,
    ValidationError,
    UTxOs
};
use pallas_codec::{
    minicbor::bytes::ByteVec,
    utils::{CborWrap, EmptyMap, MaybeIndefArray, TagWrap}
};
use pallas_crypto::hash::Hash;
use pallas_primitives::byron::{
    Address,
    Attributes,
    Twit,
    Tx,
    TxId,
    TxIn,
    TxOut
};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Note that:
    //      i)   the transaction input contains 100000 lovelace, 
    //      ii)  the minimum_fee_constant protocol parameter is 7,
    //      iii) the minimum_fee_factor protocol parameter is 11, and
    //      iv)  the size of the transaction is 82 bytes (it is easy to verify that
    //             atx == pallas_applying::AnnotatedTx {tx: tx, tx_size: 82}).
    // The expected fees are therefore 7 + 11 * 82 = 909 lovelace, which is why the output contains
    // 100000 - 909 = 99091 lovelace.
    fn successful_case() {
        let protocol_params: ProtocolParams = new_protocol_params(7, 11, 100);
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 3);
        add_tx_in(&mut tx_ins, &tx_in);
        let mut tx_outs: TxOuts = new_tx_outs();
        let tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 99091);
        add_tx_out(&mut tx_outs, &tx_out);
        let tx: Tx = new_tx(tx_ins, tx_outs, new_attributes());
        let mut utxos: UTxOs = new_utxos();
        // Note that input_tx_out is the TxOut associated with tx_in.
        let input_tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 100000);
        add_to_utxo(&mut utxos, &tx_in, input_tx_out);
        let tx_wits: Witnesses = new_witnesses();
        
        match annotate_tx(&tx) {
            None => assert!(false, "TxSizeUnavailable (sucessful_case)."),
            Some(atx) =>
                match validate_byron_tx(&atx, &tx_wits, &utxos, &protocol_params) {
                    Ok(_) => (),
                    Err(err) => assert!(false, "Unexpected error (sucessful_case - {:?}).", err),
                }
        }
    }

    #[test]
    fn empty_ins() {
        let protocol_params: ProtocolParams = new_protocol_params(0, 0, 0);
        let tx: Tx = new_tx(new_tx_ins(), new_tx_outs(), new_attributes());
        let utxos: UTxOs = new_utxos();
        let tx_wits: Witnesses = new_witnesses();

        match annotate_tx(&tx) {
            None => assert!(false, "TxSizeUnavailable (sucessful_case)."),
            Some(atx) =>
                match validate_byron_tx(&atx, &tx_wits, &utxos, &protocol_params) {
                    Ok(_) => assert!(false, "Inputs set cannot be empty."),
                    Err(err) => match err {
                        ValidationError::TxInsEmpty => (),
                        wet => assert!(false, "Wrong error type (empty_ins - {:?}).", wet),
                    }
                }
        }
    }

    #[test]
    fn empty_outs() {
        let protocol_params: ProtocolParams = new_protocol_params(0, 0, 0);
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 0);
        add_tx_in(&mut tx_ins, &tx_in);
        let tx: Tx = new_tx( tx_ins, new_tx_outs(), new_attributes());
        let mut utxos: UTxOs = new_utxos();
        let input_tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 100000);
        add_to_utxo(&mut utxos, &tx_in, input_tx_out);
        let tx_wits: Witnesses = new_witnesses();

        match annotate_tx(&tx) {
            None => assert!(false, "TxSizeUnavailable (sucessful_case)."),
            Some(atx) =>
                match validate_byron_tx(&atx, &tx_wits, &utxos, &protocol_params) {
                    Ok(_) => assert!(false, "Outputs set cannot be empty."),
                    Err(err) => match err {
                        ValidationError::TxOutsEmpty => (),
                        wet => assert!(false, "Wrong error type (empty_outs - {:?}).", wet),
                    }
                }
        }
    }

    #[test]
    fn unfound_utxo() {
        let protocol_params: ProtocolParams = new_protocol_params(0, 0, 0);
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 0);
        add_tx_in(&mut tx_ins, &tx_in);
        let mut tx_outs: TxOuts = new_tx_outs();
        let tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 1000);
        add_tx_out(&mut tx_outs, &tx_out);
        let tx: Tx = new_tx(tx_ins, tx_outs, new_attributes());
        let utxos: UTxOs = new_utxos();
        let tx_wits: Witnesses = new_witnesses();

        match annotate_tx(&tx) {
            None => assert!(false, "TxSizeUnavailable (sucessful_case)."),
            Some(atx) =>
                match validate_byron_tx(&atx, &tx_wits, &utxos, &protocol_params) {
                    Ok(_) => assert!(false, "Input must be in the set of UTxOs."),
                    Err(err) => match err {
                        ValidationError::InputNotUTxO => (),
                        wet => assert!(false, "Wrong error type (unfound_utxo - {:?}).", wet),
                    }
                }
        }
    }

    #[test]
    fn no_lovelace_in_output() {
        let protocol_params: ProtocolParams = new_protocol_params(0, 0, 0);
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 0);
        add_tx_in(&mut tx_ins, &tx_in);
        let mut tx_outs: TxOuts = new_tx_outs();
        let tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 0);
        add_tx_out(&mut tx_outs, &tx_out);
        let tx: Tx = new_tx(tx_ins, tx_outs, new_attributes());
        let mut utxos: UTxOs = new_utxos();
        let input_tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 100000);
        add_to_utxo(&mut utxos, &tx_in, input_tx_out);
        let tx_wits: Witnesses = new_witnesses();

        match annotate_tx(&tx) {
            None => assert!(false, "TxSizeUnavailable (sucessful_case)."),
            Some(atx) =>
                match validate_byron_tx(&atx, &tx_wits, &utxos, &protocol_params) {
                    Ok(_) =>
                        assert!(false, "All outputs must have a non-zero number of lovelaces."),
                    Err(err) => match err {
                        ValidationError::OutputWithoutLovelace => (),
                        wet =>
                            assert!(false, "Wrong error type (no_lovelace_in_output - {:?}).", wet),
                    }
                }
        }
    }

    #[test]
    // The case is identical to successful_case in all aspects except for the fact that the output
    // of the transaction has one more lovelace than expected.
    fn wrong_fees() {
        let protocol_params: ProtocolParams = new_protocol_params(7, 11, 0);
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 3);
        add_tx_in(&mut tx_ins, &tx_in);
        let mut tx_outs: TxOuts = new_tx_outs();
        let tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 99092);
        add_tx_out(&mut tx_outs, &tx_out);
        let tx: Tx = new_tx(tx_ins, tx_outs, new_attributes());
        let mut utxos: UTxOs = new_utxos();
        let input_tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 100000);
        add_to_utxo(&mut utxos, &tx_in, input_tx_out);
        let tx_wits: Witnesses = new_witnesses();

        match annotate_tx(&tx) {
            None => assert!(false, "TxSizeUnavailable (sucessful_case)."),
            Some(atx) =>
                match validate_byron_tx(&atx, &tx_wits, &utxos, &protocol_params) {
                    Ok(_) => assert!(false, "Incorrect fees."),
                    Err(err) => match err {
                        ValidationError::WrongFees(_, _) => (),
                        wet => assert!(false, "Wrong error type (wrong_fees - {:?}).", wet),
                    }
                }
        }
    }

    #[test]
    // Unlike in the wrong_fees test case, the fees of this transaction are correct. Nonetheless,
    // their too low compared to the related protocol parameter.
    fn fees_below_minimum() {
        let protocol_params: ProtocolParams = new_protocol_params(7, 11, 1000);
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 3);
        add_tx_in(&mut tx_ins, &tx_in);
        let mut tx_outs: TxOuts = new_tx_outs();
        let tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 99091);
        add_tx_out(&mut tx_outs, &tx_out);
        let tx: Tx = new_tx(tx_ins, tx_outs, new_attributes());
        let mut utxos: UTxOs = new_utxos();
        let input_tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 100000);
        add_to_utxo(&mut utxos, &tx_in, input_tx_out);
        let tx_wits: Witnesses = new_witnesses();

        match annotate_tx(&tx) {
            None => assert!(false, "TxSizeUnavailable (sucessful_case)."),
            Some(atx) =>
                match validate_byron_tx(&atx, &tx_wits, &utxos, &protocol_params) {
                    Ok(_) =>
                        assert!(false, "All outputs must have a non-zero number of lovelaces."),
                    Err(err) => match err {
                        ValidationError::FeesBelowMin => (),
                        wet => assert!(false, "Wrong error type (fees_below_minimum - {:?}).", wet),
                    }
                }
        }
    }

    #[test]
    // The transaction size is 82, but the maximum transaction size allowed by the protocol is 81.
    fn max_tx_size_exceeded() {
        let protocol_params: ProtocolParams = new_protocol_params(7, 11, 81);
        let mut tx_ins: TxIns = new_tx_ins();
        let tx_in: TxIn = new_tx_in(random_tx_id(), 3);
        add_tx_in(&mut tx_ins, &tx_in);
        let mut tx_outs: TxOuts = new_tx_outs();
        let tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 99091);
        add_tx_out(&mut tx_outs, &tx_out);
        let tx: Tx = new_tx(tx_ins, tx_outs, new_attributes());
        let mut utxos: UTxOs = new_utxos();
        let input_tx_out: TxOut = new_tx_out(new_address(random_address_payload(), 0), 100000);
        add_to_utxo(&mut utxos, &tx_in, input_tx_out);
        let tx_wits: Witnesses = new_witnesses();

        match annotate_tx(&tx) {
            None => assert!(false, "TxSizeUnavailable (sucessful_case)."),
            Some(atx) => match validate_byron_tx(&atx, &tx_wits, &utxos, &protocol_params) {
                Ok(_) => assert!(
                            false,
                            "Maximum tx size cannot be exceeded: {} / {}.",
                            atx.tx_size,
                            protocol_params.max_tx_size
                         ),
                Err(err) => match err {
                    ValidationError::MaxTxSizeExceeded(_, _) => (),
                    wet => assert!(false, "Wrong error type (fees_below_minimum - {:?}).", wet),
                }
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

fn add_tx_in(ins: &mut TxIns, new_in: &TxIn) {
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

fn add_tx_out(outs: &mut TxOuts, new_out: &TxOut) {
    match outs {
        MaybeIndefArray::Def(vec) | MaybeIndefArray::Indef(vec) => vec.push(new_out.clone())
    }
}

fn add_to_utxo(utxos: &mut UTxOs, tx_in: &TxIn, tx_out: TxOut) -> Option<TxOut> {
    match to_utxo_tx_in(tx_in) {
        Some(utxo_tx_in) => utxos.insert(utxo_tx_in, tx_out),
        None => None
    }
}

fn new_attributes() -> Attributes {
    EmptyMap
}

fn new_tx(ins: TxIns, outs: TxOuts, attrs: Attributes) -> Tx {
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

fn new_protocol_params(fee_constant: u64, fee_factor: u64, max_tx_size: u64) -> ProtocolParams {
    ProtocolParams {
        minimum_fee_constant: fee_constant,
        minimum_fee_factor:   fee_factor,
        max_tx_size:          max_tx_size,
    }
}
