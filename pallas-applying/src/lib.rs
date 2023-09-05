use std::collections::HashMap;
use std::hash::Hash;

use pallas_codec::minicbor::{
    encode,
    bytes::ByteVec
};
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


pub struct ProtocolParams {
    pub minimum_fee_constant: u64,
    pub minimum_fee_factor: u64,
    pub max_tx_size: u64,
}

#[derive(Debug)]
pub enum ValidationError {
    UnsupportedEra(String),
    TxSizeUnavailable,
    TxInsEmpty,
    TxOutsEmpty,
    InputNotUTxO,
    IllFormedInput,
    OutputWithoutLovelace,
    WrongFees(u64, u64),
    FeesBelowMin,
    MaxTxSizeExceeded(u64, u64),
}

pub type ValidationResult = Result<(), ValidationError>;

pub fn err_result(validation_error: ValidationError) -> ValidationResult {
    Err(validation_error)
}

pub type UTxOs = HashMap<UTxOTxIn, TxOut>;

fn get_tx_out_from_tx_in<'a>(input: &TxIn, utxos: &'a UTxOs) -> Option<&'a TxOut> {
    let utox_tx_in: UTxOTxIn = to_utxo_tx_in(input)?;
    utxos.get(&utox_tx_in)
}

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

pub struct AnnotatedTx {
    pub tx: Tx,
    pub tx_size: u64,
}

pub fn annotate_tx(tx: &Tx) -> Option<AnnotatedTx>{
    let mut buffer: Vec<u8> = Vec::new();
    match encode(tx.clone(), &mut buffer) {
        Ok(()) => Some(AnnotatedTx{
                        tx: tx.clone(),
                        tx_size: buffer.len() as u64,
                  }),
        Err(_) => None
    }
}

pub fn validate(
    metx: &MultiEraTx,
    witnesses: &Witnesses,
    utxos: &UTxOs,
    prot_params: ProtocolParams
) -> ValidationResult {
    match metx {
        Byron(mtxp) => {
            match annotate_tx(&*(mtxp.transaction)) {
                None => err_result(ValidationError::TxSizeUnavailable),
                Some(atx) => validate_byron_tx(&atx, witnesses, utxos, &prot_params)
            }
        },
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
    atx: &AnnotatedTx,
    witnesses: &Witnesses,
    utxos: &UTxOs,
    protocol_params: &ProtocolParams
) -> ValidationResult {
    check_ins_not_empty(&atx.tx)?;
    check_ins_in_utxos(&atx.tx, &utxos)?;
    check_outs_not_empty(&atx.tx)?;
    check_outputs_not_zero_lovelace(&atx.tx)?;
    check_fees(&atx, &utxos, &protocol_params)?;
    check_size(&atx, &protocol_params)?;
    check_witnesses(&atx.tx, &witnesses)
}

// The set of transaction inputs is not empty.
fn check_ins_not_empty(tx: &Tx) -> ValidationResult {
    if tx.inputs.clone().to_vec().len() == 0 {
        err_result(ValidationError::TxInsEmpty)
    } else {
        Ok(())
    }
}

// All transaction inputs are in the set of UTxO's.
fn check_ins_in_utxos(tx: &Tx, utxos: &UTxOs) -> ValidationResult {
    for input in tx.inputs.iter() {
        match to_utxo_tx_in(input) {
            Some(utxo_in) => {
                if !(utxos.contains_key(&utxo_in)) {
                    return err_result(ValidationError::InputNotUTxO)
                }
            },
            None => return err_result(ValidationError::IllFormedInput)
        }
    }
    Ok(())
}

// The set of transaction outputs is not empty.
fn check_outs_not_empty(tx: &Tx) -> ValidationResult {
    if tx.outputs.clone().to_vec().len() == 0 {
        err_result(ValidationError::TxOutsEmpty)
    } else {
        Ok(())
    }
}

// All transaction outputs contain a non-zero number of lovelace.
fn check_outputs_not_zero_lovelace(tx: &Tx) -> ValidationResult {
    for output in tx.outputs.iter() {
        if output.amount == 0 {
            return err_result(ValidationError::OutputWithoutLovelace)
        }
    }
    Ok(())
}

// The transaction fees are correctly computed, and they are greater than or equal to the minimum
// number of fees according to the protocol parameters.
fn check_fees(
    atx: &AnnotatedTx,
    utxos: &UTxOs,
    protocol_params: &ProtocolParams
) -> ValidationResult {
    let mut input_balance: u64 = 0;
    for tx_in in atx.tx.inputs.iter() {
        match get_tx_out_from_tx_in(&tx_in, &utxos) {
            Some(tx_out) => input_balance += tx_out.amount,
            None => ()
        }
    }
    let mut output_balance: u64 = 0;
    for tx_out in atx.tx.outputs.iter() {
        output_balance += tx_out.amount;
    }
    let paid_fees: u64 = input_balance - output_balance;
    let computed_fees: u64 =
        protocol_params.minimum_fee_constant + protocol_params.minimum_fee_factor * atx.tx_size;

    if paid_fees != computed_fees {
        err_result(ValidationError::WrongFees(paid_fees, computed_fees))
    } else if computed_fees < protocol_params.max_tx_size{
        err_result(ValidationError::FeesBelowMin)
    } else {
        Ok(())
    }
}

// The transaction size does not exceed the maximum size allowed by the protocol.
fn check_size(atx: &AnnotatedTx, protocol_params: &ProtocolParams) -> ValidationResult {
    if atx.tx_size > protocol_params.max_tx_size {
        err_result(ValidationError::MaxTxSizeExceeded(atx.tx_size, protocol_params.max_tx_size))
    } else {
        Ok(())
    }
}

// The expected witnessses have signed the transaction.
fn check_witnesses(_tx: &Tx, _witnesses: &Witnesses) -> ValidationResult {
    Ok(())
}
