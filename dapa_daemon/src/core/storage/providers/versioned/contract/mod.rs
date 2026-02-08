mod data;
mod balance;
mod scheduled_execution;
mod event_callback;

use async_trait::async_trait;
use dapa_common::block::TopoHeight;
use crate::core::error::BlockchainError;

pub use data::*;
pub use balance::*;
pub use scheduled_execution::VersionedScheduledExecutionsProvider;
pub use event_callback::*;

#[async_trait]
pub trait VersionedContractProvider: VersionedContractDataProvider + VersionedContractBalanceProvider + VersionedScheduledExecutionsProvider + VersionedContractEventCallbackProvider {
    // delete versioned contracts at topoheight
    async fn delete_versioned_contracts_at_topoheight(&mut self, topoheight: TopoHeight) -> Result<(), BlockchainError>;

    // delete versioned contracts above topoheight
    async fn delete_versioned_contracts_above_topoheight(&mut self, topoheight: TopoHeight) -> Result<(), BlockchainError>;

    // delete versioned contracts below topoheight
    async fn delete_versioned_contracts_below_topoheight(&mut self, topoheight: TopoHeight, keep_last: bool) -> Result<(), BlockchainError>;
}