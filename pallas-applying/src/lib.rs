//! Logic for validating and applying new blocks and txs to the chain state

use std::collections::HashMap;

use pallas_codec::minicbor::encode;
use pallas_primitives::byron::{
    MintedTxPayload,
    Tx as ByronTx,
    TxIn,
    TxOut
};
use pallas_traverse::{
    MultiEraTx,
    MultiEraTx::Byron as Byron
};


pub struct ProtocolParams;

#[derive(Debug)]
#[non_exhaustive]
pub enum ValidationError {
    ValidationError
}

pub type ValidationResult = Result<(), ValidationError>;

pub type UTxOs = HashMap<TxIn, TxOut>;

pub type TxSize = u64;

pub fn get_byron_tx_size(tx: &ByronTx) -> Option<TxSize>{
    let mut buffer: Vec<u8> = Vec::new();
    match encode(tx.clone(), &mut buffer) {
        Ok(_) => Some(buffer.len() as u64),
        Err(_) => None
    }
}

pub fn validate(metx: &MultiEraTx, utxos: &UTxOs, prot_pps: &ProtocolParams) -> ValidationResult {
    match metx {
        Byron(mtxp) => validate_byron_tx(mtxp, utxos, prot_pps),
        _ => Ok(())
    }
}

pub fn validate_byron_tx(
    _mtxp: &MintedTxPayload,
    _utxos: &UTxOs,
    _prot_pps: &ProtocolParams
) -> ValidationResult {
    Ok(())
}
