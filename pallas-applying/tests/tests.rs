use std::vec::Vec;

use pallas_applying::{ProtocolParams, validate_byron_tx};
use pallas_codec::utils::{MaybeIndefArray, EmptyMap};
use pallas_primitives::byron::{Attributes, Twit, Tx, TxIn, TxOut};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        let tx_ins: MaybeIndefArray<TxIn> = MaybeIndefArray::Def(Vec::new());
        let tx_outs: MaybeIndefArray<TxOut> = MaybeIndefArray::Def(Vec::new());
        let tx_atts: Attributes = EmptyMap;
        let tx: Tx = Tx {
            inputs: tx_ins,
            outputs: tx_outs,
            attributes: tx_atts,
        };
        let tx_wits: MaybeIndefArray<Twit> = MaybeIndefArray::Def(Vec::new());
        let utxos: Vec<TxOut> = Vec::new();
        let protocol_params: ProtocolParams = ProtocolParams;

        assert!(validate_byron_tx(&tx, &tx_wits, &utxos, &protocol_params).is_ok());
    }
}