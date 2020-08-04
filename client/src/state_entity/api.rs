//! API
//!
//! API calls availble for Client to State Entity

use super::super::Result;
use shared_lib::structs::{
    SmtProofMsgAPI, StateChainDataAPI, StateEntityFeeInfoAPI, TransferBatchDataAPI,
};
use shared_lib::Root;

use super::super::utilities::requests;
use crate::ClientShim;

use monotree::Proof;
use uuid::Uuid;

/// Get state chain fee
pub fn get_statechain_fee_info(client_shim: &ClientShim) -> Result<StateEntityFeeInfoAPI> {
    requests::get(client_shim, &format!("info/fee"))
}

/// Get state chain by ID
pub fn get_statechain(
    client_shim: &ClientShim,
    state_chain_id: &Uuid,
) -> Result<StateChainDataAPI> {
    requests::get(client_shim, &format!("info/statechain/{}", state_chain_id))
}

/// Get state entity's sparse merkle tree root
pub fn get_smt_root(client_shim: &ClientShim) -> Result<Option<Root>> {
    requests::get(&client_shim, &format!("info/root"))
}

/// Get state entity's sparse merkle tree root that has been confirmed by mainstay
pub fn get_confirmed_smt_root(client_shim: &ClientShim) -> Result<Option<Root>> {
    requests::get(&client_shim, &format!("info/confirmed_root"))
}

/// Get state chain inclusion proof
pub fn get_smt_proof(
    client_shim: &ClientShim,
    root: &Root,
    funding_txid: &String,
) -> Result<Option<Proof>> {
    let smt_proof_msg = SmtProofMsgAPI {
        root: root.clone(),
        funding_txid: funding_txid.clone(),
    };
    requests::postb(&client_shim, &format!("info/proof"), smt_proof_msg)
}

/// Get transaction batch session status
pub fn get_transfer_batch_status(
    client_shim: &ClientShim,
    batch_id: &Uuid,
) -> Result<TransferBatchDataAPI> {
    requests::post(client_shim, &format!("info/transfer-batch/{}", batch_id))
}
