use async_trait::async_trait;
use dapa_common::block::TopoHeight;
use log::trace;

use crate::core::{
    error::{BlockchainError, DiskContext},
    storage::{SledStorage, VersionedContractEventCallbackProvider}
};

#[async_trait]
impl VersionedContractEventCallbackProvider for SledStorage {
    // delete versioned contract event callbacks at topoheight
    async fn delete_versioned_contract_event_callbacks_at_topoheight(&mut self, topoheight: TopoHeight) -> Result<(), BlockchainError> {
        trace!("delete versioned contract event callbacks at topoheight {}", topoheight);
        Self::delete_versioned_tree_at_topoheight(&mut self.snapshot, &self.contracts_event_callbacks, &self.versioned_contracts_event_callbacks, topoheight)
    }

    // delete versioned contract event callbacks above topoheight
    async fn delete_versioned_contract_event_callbacks_above_topoheight(&mut self, topoheight: TopoHeight) -> Result<(), BlockchainError> {
        trace!("delete versioned contract event callbacks above topoheight {}", topoheight);
        Self::delete_versioned_tree_above_topoheight(&mut self.snapshot, &self.contracts_event_callbacks, &self.versioned_contracts_event_callbacks, topoheight, DiskContext::ContractEventCallback)
    }

    // delete versioned contract event callbacks below topoheight
    async fn delete_versioned_contract_event_callbacks_below_topoheight(&mut self, topoheight: TopoHeight, keep_last: bool) -> Result<(), BlockchainError> {
        trace!("delete versioned contract event callbacks below topoheight {}", topoheight);
        Self::delete_versioned_tree_below_topoheight(&mut self.snapshot, &self.contracts_event_callbacks, &self.versioned_contracts_event_callbacks, topoheight, keep_last, DiskContext::ContractEventCallback)
    }
}