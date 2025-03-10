use dapa_vm::tid;

use crate::{block::TopoHeight, crypto::Hash};

use super::ContractStorage;

pub trait ContractProvider: ContractStorage + 'static {
    // Returns the balance of the contract
    fn get_contract_balance_for_asset(&self, contract: &Hash, asset: &Hash, topoheight: TopoHeight) -> Result<Option<(TopoHeight, u64)>, anyhow::Error>;
}

// This is a wrapper around the storage to allow for the storage to be passed in the Context
pub struct ContractProviderWrapper<'a, S: ContractProvider>(pub &'a mut S);

tid! { impl<'a, S: 'static> TidAble<'a> for ContractProviderWrapper<'a, S> where S: ContractProvider }
