use async_trait::async_trait;
use dapa_common::block::TopoHeight;

use crate::core::error::BlockchainError;

#[async_trait]
pub trait VersionedContractEventCallbackProvider {
    // delete versioned contract event callbacks at topoheight
    async fn delete_versioned_contract_event_callbacks_at_topoheight(&mut self, topoheight: TopoHeight) -> Result<(), BlockchainError>;

    // delete versioned contract event callbacks above topoheight
    async fn delete_versioned_contract_event_callbacks_above_topoheight(&mut self, topoheight: TopoHeight) -> Result<(), BlockchainError>;

    // delete versioned contract event callbacks below topoheight
    async fn delete_versioned_contract_event_callbacks_below_topoheight(&mut self, topoheight: TopoHeight, keep_last: bool) -> Result<(), BlockchainError>;
}