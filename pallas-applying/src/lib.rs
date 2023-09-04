use std::collections::HashMap;
use std::hash::Hash;

use pallas_codec::minicbor::bytes::ByteVec;
use pallas_primitives::byron::{
    Tx,
    TxId,
    TxIn,
    TxOut,
    Witnesses
};
use pallas_traverse::{
    MultiEraTx,
    MultiEraTx::Byron as Byron,
    MultiEraTx::AlonzoCompatible as AlonzoCompatible,
    MultiEraTx::Babbage as Babbage
};


pub struct ProtocolParams;

#[derive(Debug)]
pub enum ValidationError {
    UnsupportedEra(String),
    TxInsEmpty,
    TxOutsEmpty,
    InputMissingFromUTxO,
    IllFormedInput,
    OutputWithoutLovelace,
}

pub type ValidationResult = Result<(), ValidationError>;

pub fn err_result(validation_error: ValidationError) -> ValidationResult {
    Err(validation_error)
}

pub type UTxOs = HashMap<UTxOTxIn, TxOut>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum UTxOTxIn {
    Variant0(TxId, u32),
    OtherVariant(u8, ByteVec),
}

pub fn to_utxo_tx_in(tx_in: &TxIn) -> Option<UTxOTxIn> {
    match tx_in {
        TxIn::Variant0(cbor_wrap) => {
            let (tx_id, index): (TxId, u32) = cbor_wrap.clone().unwrap();
            Some(UTxOTxIn::Variant0(tx_id, index))
        },
        TxIn::Other(_, _) => None,
    }
}

pub fn validate(
    tx: &MultiEraTx,
    witnesses: &Witnesses,
    utxos: &UTxOs,
    prot_params: ProtocolParams
) -> ValidationResult {
    match tx {
        Byron(mtxp) => validate_byron_tx(&*(mtxp.transaction), witnesses, utxos, prot_params),
        AlonzoCompatible(_, _) => err_result(
            ValidationError::UnsupportedEra("Alonzo-compatible eras not supported.".to_string())
        ),
        Babbage(_) => err_result(
            ValidationError::UnsupportedEra("Babbage era not supported.".to_string())
        ),
        _ => err_result(ValidationError::UnsupportedEra("Era not supported.".to_string()))
    }
}

// Perform all checks on tx according to the Byron era, succeeding only if all checks succeed.
pub fn validate_byron_tx(
    tx: &Tx,
    witnesses: &Witnesses,
    utxos: &UTxOs,
    protocol_params: ProtocolParams
) -> ValidationResult {
    check_ins_not_empty(&tx)?;
    check_ins_in_utxos(&tx, &utxos)?;
    check_outs_not_empty(&tx)?;
    check_outputs_not_zero_lovelace(&tx)?;
    check_fees(&tx, &protocol_params)?;
    check_min_fees(&tx, &protocol_params)?;
    check_size(&tx, &protocol_params)?;
    check_witnesses(&tx, &witnesses)
}

// The set of transaction inputs is not empty.
pub fn check_ins_not_empty(tx: &Tx) -> ValidationResult {
    if tx.inputs.clone().to_vec().len() == 0 {
        err_result(ValidationError::TxInsEmpty)
    } else {
        Ok(())
    }
}

// All transaction inputs are in the set of UTxO's.
pub fn check_ins_in_utxos(tx: &Tx, utxos: &UTxOs) -> ValidationResult {
    for input in tx.inputs.iter() {
        match to_utxo_tx_in(input) {
            Some(utxo_in) => {
                if !(utxos.contains_key(&utxo_in)) {
                    return err_result(ValidationError::InputMissingFromUTxO)
                }
            },
            None => return err_result(ValidationError::IllFormedInput)
        }
    }
    Ok(())
}

// The set of transaction outputs is not empty.
pub fn check_outs_not_empty(tx: &Tx) -> ValidationResult {
    if tx.outputs.clone().to_vec().len() == 0 {
        err_result(ValidationError::TxOutsEmpty)
    } else {
        Ok(())
    }
}

// All transaction outputs contain a non-zero number of lovelace.
pub fn check_outputs_not_zero_lovelace(tx: &Tx) -> ValidationResult {
    for output in tx.outputs.iter() {
        if output.amount == 0 {
            return err_result(ValidationError::OutputWithoutLovelace)
        }
    }
    Ok(())
}

// The transaction fees are correctly computed.
pub fn check_fees(_tx: &Tx, _protocol_params: &ProtocolParams) -> ValidationResult {
    Ok(())
}

// The transaction fees are greater to or equal than min fees.
pub fn check_min_fees(_tx: &Tx, _protocol_params: &ProtocolParams) -> ValidationResult {
    Ok(())
}

// The transaction size does not exceed the maximum size allowed by the protocol.
pub fn check_size(_tx: &Tx, _protocol_params: &ProtocolParams) -> ValidationResult {
    Ok(())
}

// The expected witnessses have signed the transaction.
pub fn check_witnesses(_tx: &Tx, _witnesses: &Witnesses) -> ValidationResult {
    Ok(())
}

