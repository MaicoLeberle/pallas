use std::vec::Vec;

use pallas_primitives::byron::{
    Tx,
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

pub enum ValidationError {
    UnsupportedEra(String),
    TxInsEmpty(String),
}

pub type ValidationResult = Result<(), ValidationError>;

pub fn err_result(validation_error: ValidationError) -> ValidationResult {
    Err(validation_error)
}

pub fn validate(
    tx: &MultiEraTx,
    witnesses: &Witnesses,
    utxos: &Vec<TxOut>,
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
    utxos: &Vec<TxOut>,
    protocol_params: ProtocolParams
) -> ValidationResult {
    validate_ins_not_empty(&tx)?;
    validate_ins_in_utxos(&tx, &utxos)?;
    validate_outs_not_empty(&tx)?;
    validate_outputs_not_zero_lovelace(&tx)?;
    validate_fees(&tx, &protocol_params)?;
    validate_min_fees(&tx, &protocol_params)?;
    validate_size(&tx, &protocol_params)?;
    validate_witnesses(&tx, &witnesses)
}

// The set of transaction inputs is not empty.
pub fn validate_ins_not_empty(tx: &Tx) -> ValidationResult {
    if tx.clone().inputs.to_vec().len() == 0 {
        err_result(ValidationError::TxInsEmpty("Inputs set cannot be empty.".to_string()))
    } else {
        Ok(())
    }
}

// All transaction inputs are in the set of UTxO's.
pub fn validate_ins_in_utxos(_tx: &Tx, _utxos: &Vec<TxOut>) -> ValidationResult {
    Ok(())

}

// The set of transaction outputs is not empty.
pub fn validate_outs_not_empty(_tx: &Tx) -> ValidationResult {
    Ok(())
}

// All transaction outputs contain a non-zero number of lovelace.
pub fn validate_outputs_not_zero_lovelace(_tx: &Tx) -> ValidationResult {
    Ok(())
}

// The transaction fees are correctly computed.
pub fn validate_fees(_tx: &Tx, _protocol_params: &ProtocolParams) -> ValidationResult {
    Ok(())
}

// The transaction fees are greater to or equal than min fees.
pub fn validate_min_fees(_tx: &Tx, _protocol_params: &ProtocolParams) -> ValidationResult {
    Ok(())
}

// The transaction size does not exceed the maximum size allowed by the protocol.
pub fn validate_size(_tx: &Tx, _protocol_params: &ProtocolParams) -> ValidationResult {
    Ok(())
}

// The expected witnessses have signed the transaction.
pub fn validate_witnesses(_tx: &Tx, _witnesses: &Witnesses) -> ValidationResult {
    Ok(())
}

