use async_trait::async_trait;
use log::trace;
use dapa_common::{
    account::{BalanceType, VersionedBalance},
    block::TopoHeight,
    crypto::{Hash, PublicKey},
    serializer::Serializer
};
use crate::core::{
    error::{BlockchainError, DiskContext},
    storage::{
        BalanceProvider,
        SledStorage
    }
};

#[async_trait]
pub trait VersionedBalanceProvider {
    // delete versioned balances at topoheight
    async fn delete_versioned_balances_at_topoheight(&mut self, topoheight: TopoHeight) -> Result<(), BlockchainError>;

    // delete versioned balances above topoheight
    async fn delete_versioned_balances_above_topoheight(&mut self, topoheight: TopoHeight) -> Result<(), BlockchainError>;

    // delete versioned balances below topoheight
    async fn delete_versioned_balances_below_topoheight(&mut self, topoheight: TopoHeight, all: bool) -> Result<(), BlockchainError>;
}


#[async_trait]
impl VersionedBalanceProvider for SledStorage {
    async fn delete_versioned_balances_at_topoheight(&mut self, topoheight: TopoHeight) -> Result<(), BlockchainError> {
        trace!("delete versioned balances at topoheight {}", topoheight);
        // TODO: scan prefix support snapshot
        for el in self.versioned_balances.scan_prefix(&topoheight.to_be_bytes()) {
            let (key, value) = el?;
            // Delete this version from DB
            Self::remove_from_disk_without_reading(self.snapshot.as_mut(), &self.versioned_balances, &key)?;

            // Deserialize keys part
            let asset = Hash::from_bytes(&key[40..72])?;
            let key = PublicKey::from_bytes(&key[8..40])?;

            let last_topoheight = self.get_last_topoheight_for_balance(&key, &asset).await?;
            if last_topoheight >= topoheight {
                // Deserialize value, it is needed to get the previous topoheight
                let versioned_balance = VersionedBalance::from_bytes(&value)?;
    
                // Now records changes, for each balances
                let db_key = self.get_balance_key_for(&key, &asset);
                if let Some(previous_topoheight) = versioned_balance.get_previous_topoheight() {
                    Self::insert_into_disk(self.snapshot.as_mut(), &self.balances, &db_key, &previous_topoheight.to_be_bytes())?;
                } else {
                    // if there is no previous topoheight, it means that this is the first version
                    // so we can delete the balance
                    Self::remove_from_disk_without_reading(self.snapshot.as_mut(), &self.balances, &db_key)?;
                }
            }
        }
        Ok(())
    }

    async fn delete_versioned_balances_above_topoheight(&mut self, topoheight: u64) -> Result<(), BlockchainError> {
        trace!("delete versioned balances above topoheight {}!", topoheight);
        Self::delete_versioned_tree_above_topoheight(&mut self.snapshot, &self.versioned_balances, topoheight)
    }

    async fn delete_versioned_balances_below_topoheight(&mut self, topoheight: u64, keep_last: bool) -> Result<(), BlockchainError> {
        trace!("delete versioned balances (keep last: {}) below topoheight {}!", keep_last, topoheight);
        if !keep_last {
            Self::delete_versioned_tree_below_topoheight(&mut self.snapshot, &self.balances, &self.versioned_balances, topoheight, keep_last, DiskContext::BalanceAtTopoHeight)
        } else {
            // We need to search until we find the latest output version
            // And we delete everything below it

            // We check one account at a time
            for el in self.balances.iter() {
                let (k, value) = el?;
                let topo = TopoHeight::from_bytes(&value)?;

                // We fetch the last version to take its previous topoheight
                // And we loop on it to delete them all until the end of the chained data
                // But before deleting, we need to find if we are below a output balance
                let mut prev_version = self.load_from_disk(&self.versioned_balances, &Self::get_versioned_key(&k, topo), DiskContext::BalanceAtTopoHeight)?;
                let mut delete = false;
                while let Some(prev_topo) = prev_version {
                    let key = Self::get_versioned_key(&k, prev_topo);

                    // Delete this version from DB if its below the threshold
                    if delete {
                        prev_version = Self::remove_from_disk(self.snapshot.as_mut(), &self.versioned_balances, &key)?;
                    } else {
                        let (prev_topo, ty) = self.load_from_disk::<(Option<u64>, BalanceType)>(&self.versioned_balances, &key, DiskContext::BalanceAtTopoHeight)?;
                        // If this version contains an output, that means we can delete all others below
                        delete = ty.contains_output();
                        prev_version = prev_topo;
                    }
                }
            }

            Ok(())
        }
    }
}