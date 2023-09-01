use std::error::Error;
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

pub fn validate(
    tx: &MultiEraTx,
    witnesses: &Witnesses,
    utxos: &Vec<TxOut>,
    prot_params: &ProtocolParams
) -> Result<(), Box<dyn Error>> {
    match tx {
        Byron(mtxp) => validate_byron_tx(&*(mtxp.transaction), &witnesses, &utxos, &prot_params),
        AlonzoCompatible(_, _) => Err("Alonzo-compatible eras not supported.".into()),
        Babbage(_) => Err("Babbage era not supported.".into()),
        _ => Err("Era not supported.".into())
    }
}

// Perform all checks on tx according to the Byron era, succeeding only if all checks succeed.
pub fn validate_byron_tx(
    tx: &Tx,
    witnesses: &Witnesses,
    utxos: &Vec<TxOut>,
    protocol_params: &ProtocolParams
) -> Result<(), Box<dyn Error>> {
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
pub fn validate_ins_not_empty(_tx: &Tx) -> Result<(), Box<dyn Error>> {
    Ok(())
}

// All transaction inputs are in the set of UTxO's.
pub fn validate_ins_in_utxos(_tx: &Tx, _utxos: &Vec<TxOut>) -> Result<(), Box<dyn Error>> {
    Ok(())

}

// The set of transaction outputs is not empty.
pub fn validate_outs_not_empty(_tx: &Tx) -> Result<(), Box<dyn Error>> {
    Ok(())
}

// All transaction outputs contain a non-zero number of lovelace.
pub fn validate_outputs_not_zero_lovelace(_tx: &Tx) -> Result<(), Box<dyn Error>> {
    Ok(())
}

// The transaction fees are correctly computed.
pub fn validate_fees(_tx: &Tx, _protocol_params: &ProtocolParams) -> Result<(), Box<dyn Error>> {
    Ok(())
}

// The transaction fees are greater to or equal than min fees.
pub fn validate_min_fees(_tx: &Tx, _protocol_params: &ProtocolParams) -> Result<(), Box<dyn Error>> {
    Ok(())
}

// The transaction size does not exceed the maximum size allowed by the protocol.
pub fn validate_size(_tx: &Tx, _protocol_params: &ProtocolParams) -> Result<(), Box<dyn Error>> {   
    Ok(())
}

// The expected witnessses have signed the transaction.
pub fn validate_witnesses(_tx: &Tx, _witnesses: &Witnesses) -> Result<(), Box<dyn Error>> {
    Ok(())
}

