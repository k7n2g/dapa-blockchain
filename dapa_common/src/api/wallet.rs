use std::{borrow::Cow, collections::HashMap};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::{
    account::CiphertextCache,
    block::TopoHeight,
    crypto::{elgamal::CompressedCiphertext, Address, Hash, PrivateKey},
    serializer::Hexable,
    transaction::{
        builder::{FeeBuilder, TransactionTypeBuilder, UnsignedTransaction},
        extra_data::{PlaintextExtraData, UnknownExtraDataFormat},
        multisig::SignatureId,
        Reference,
        Role,
        Transaction,
        TxVersion
    }
};
use super::{
    DataHash,
    DataElement,
    DataValue,
    query::Query,
    default_false_value,
    default_true_value,
    daemon
};

// Signer ID to use for signing the transaction
#[derive(Serialize, Deserialize)]
pub struct SignerId {
    pub id: u8,
    pub private_key: PrivateKey
}

#[derive(Serialize, Deserialize)]
pub struct BuildTransactionParams {
    #[serde(flatten)]
    pub tx_type: TransactionTypeBuilder,
    // Fee to use, if value is fixed,
    // it will be used as is, otherwise it will be calculated
    pub fee: Option<FeeBuilder>,
    // Nonce to use for the transaction
    // If not present, it will be generated by the wallet
    pub nonce: Option<u64>,
    // Version to use for the TX
    // By default, grab the version from wallet
    pub tx_version: Option<TxVersion>,
    // Cannot be broadcasted if set to false
    #[serde(default = "default_true_value")]
    pub broadcast: bool,
    // Returns the TX in HEX format also
    #[serde(default = "default_false_value")]
    pub tx_as_hex: bool,
    // List of private keys to sign the transaction
    // This allow a person to directly sign in the wallet
    #[serde(default)]
    pub signers: Vec<SignerId>,
}

#[derive(Serialize, Deserialize)]
pub struct BuildTransactionOfflineParams {
    #[serde(flatten)]
    pub tx_type: TransactionTypeBuilder,
    // Fixed fee is required and must be checked before calling this
    #[serde(default)]
    pub fee: FeeBuilder,
    // Version to use for the TX
    // By default, grab the version from wallet
    pub tx_version: Option<TxVersion>,
    // Returns the TX in HEX format also
    #[serde(default = "default_false_value")]
    pub tx_as_hex: bool,
    // Encrypted balances to use
    // Assets spent in the transaction must be present
    pub balances: HashMap<Hash, CiphertextCache>,
    // Reference to use for the transaction
    // This must point to the most up-to-date topoheight/block hash
    pub reference: Reference,
    // Nonce to use for the transaction
    pub nonce: u64,
    // List of private keys to sign the transaction
    // This allow a person to directly sign in the wallet
    #[serde(default)]
    pub signers: Vec<SignerId>,
}

#[derive(Serialize, Deserialize)]
pub struct BuildUnsignedTransactionParams {
    #[serde(flatten)]
    pub tx_type: TransactionTypeBuilder,
    // Fee to use, if value is fixed,
    // it will be used as is, otherwise it will be calculated
    pub fee: Option<FeeBuilder>,
    // Nonce to use for the transaction
    // If not present, it will be generated by the wallet
    pub nonce: Option<u64>,
    // Version to use for the TX
    // By default, grab the version from wallet
    pub tx_version: Option<TxVersion>,
    // Returns the TX in HEX format also
    #[serde(default = "default_false_value")]
    pub tx_as_hex: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FinalizeUnsignedTransactionParams {
    // Unsigned transaction to finalize
    pub unsigned: Hexable<UnsignedTransaction>,
    // Signatures to append in the transaction
    // In case it wasn't added in the unsigned already
    #[serde(default)]
    pub signatures: Vec<SignatureId>,
    // Cannot be broadcasted if set to false
    #[serde(default = "default_true_value")]
    pub broadcast: bool,
    // Returns the TX in HEX format also
    #[serde(default = "default_false_value")]
    pub tx_as_hex: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SignUnsignedTransactionParams {
    // Unsigned transaction hash
    pub hash: Hash,
    // Signer ID to use for signing the transaction
    pub signer_id: u8
}

#[derive(Serialize, Deserialize)]
pub struct UnsignedTransactionResponse {
    #[serde(flatten)]
    pub inner: UnsignedTransaction,
    // Unsigned TX hash for signing for multisig signers
    pub hash: Hash,
    // Multisig threshold, zero if not active
    pub threshold: Option<u8>,
    // Unsigned transaction in hex format
    pub tx_as_hex: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct EstimateFeesParams {
    #[serde(flatten)]
    pub tx_type: TransactionTypeBuilder,
}

#[derive(Serialize, Deserialize)]
pub struct ListTransactionsParams {
    // Filter by asset
    pub asset: Option<Hash>,
    // Filter by topoheight range (inclusive)
    pub min_topoheight: Option<TopoHeight>,
    pub max_topoheight: Option<TopoHeight>,
    /// Receiver address for outgoing txs, and owner/sender for incoming
    pub address: Option<Address>,
    #[serde(default = "default_true_value")]
    pub accept_incoming: bool,
    #[serde(default = "default_true_value")]
    pub accept_outgoing: bool,
    #[serde(default = "default_true_value")]
    pub accept_coinbase: bool,
    #[serde(default = "default_true_value")]
    pub accept_burn: bool,
    // Filter by extra data
    pub query: Option<Query>,
    // Limit the number of entries returned
    pub limit: Option<usize>,
    // Skip the first N entries
    pub skip: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponse<'a> {
    #[serde(flatten)]
    pub inner: DataHash<'a, Transaction>,
    pub tx_as_hex: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct GetAssetPrecisionParams<'a> {
    pub asset: Cow<'a, Hash>
}

#[derive(Serialize, Deserialize)]
pub struct GetAddressParams {
    // Data to use for creating an integrated address
    // Returned address will contains all the data provided here
    pub integrated_data: Option<DataElement>
}

#[derive(Serialize, Deserialize)]
pub struct RescanParams {
    pub until_topoheight: Option<TopoHeight>,
    #[serde(default = "default_false_value")]
    pub auto_reconnect: bool
}

#[derive(Serialize, Deserialize)]
pub struct SetOnlineModeParams {
    pub daemon_address: String,
    #[serde(default = "default_false_value")]
    pub auto_reconnect: bool,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkInfoResult {
    #[serde(flatten)]
    pub inner: daemon::GetInfoResult,
    pub connected_to: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBalanceParams {
    pub asset: Option<Hash>
}

#[derive(Serialize, Deserialize)]
pub struct GetTransactionParams {
    pub hash: Hash
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BalanceChanged {
    pub asset: Hash,
    pub balance: u64
}

#[derive(Serialize, Deserialize)]
pub struct GetValueFromKeyParams {
    pub tree: String,
    pub key: DataValue
}

#[derive(Serialize, Deserialize)]
pub struct HasKeyParams {
    pub tree: String,
    pub key: DataValue
}

#[derive(Serialize, Deserialize)]
pub struct GetMatchingKeysParams {
    pub tree: String,
    pub query: Option<Query>,
    pub limit: Option<usize>,
    pub skip: Option<usize>
}

#[derive(Serialize, Deserialize)]
pub struct CountMatchingEntriesParams {
    pub tree: String,
    pub key: Option<Query>,
    pub value: Option<Query>
}

#[derive(Serialize, Deserialize)]
pub struct StoreParams {
    pub tree: String,
    pub key: DataValue,
    pub value: DataElement
}

#[derive(Serialize, Deserialize)]
pub struct DeleteParams {
    pub tree: String,
    pub key: DataValue
}

#[derive(Serialize, Deserialize)]
pub struct DeleteTreeEntriesParams {
    pub tree: String
}

#[derive(Serialize, Deserialize)]
pub struct QueryDBParams {
    pub tree: String,
    pub key: Option<Query>,
    pub value: Option<Query>,
    pub limit: Option<usize>,
    pub skip: Option<usize>
}

#[derive(Serialize, Deserialize)]
pub struct DecryptExtraDataParams<'a> {
    // Encrypted data to decrypt
    pub extra_data: Cow<'a, UnknownExtraDataFormat>,
    // The role we have in the transaction
    // This is needed to select the correct handle
    pub role: Role,
}

#[derive(Serialize, Deserialize)]
pub struct DecryptCiphertextParams<'a> {
    // Ciphertext with the correct handle to use
    pub ciphertext: Cow<'a, CompressedCiphertext>
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotifyEvent {
    // When a new topoheight is detected by wallet
    // it contains the topoheight (u64) as value
    // It may be lower than the previous one, based on how the DAG reacts
    NewTopoHeight,
    // When a new asset is added to wallet
    // Contains a Hash as value
    NewAsset,
    // When a new transaction is added to wallet
    // Contains TransactionEntry struct as value
    NewTransaction,
    // When a balance is changed
    // Contains a BalanceChanged as value
    BalanceChanged,
    // When a rescan happened on the wallet
    // Contains a topoheight as value to indicate until which topoheight transactions got deleted
    Rescan,
    // When the history has been synced again
    // Contains current topoheight as value
    HistorySynced,
    // When network state changed
    Online,
    // Same here
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOut {
    // Destination address
    pub destination: Address,
    // Asset spent
    pub asset: Hash,
    // Plaintext amount
    pub amount: u64,
    // extra data
    pub extra_data: Option<PlaintextExtraData>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIn {
    // Asset spent
    pub asset: Hash,
    // Plaintext amount
    pub amount: u64,
    // extra data
    pub extra_data: Option<PlaintextExtraData>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryType {
    // Coinbase is only DAPA_ASSET
    Coinbase {
        reward: u64
    },
    Burn {
        asset: Hash,
        amount: u64,
        fee: u64,
        nonce: u64
    },
    Incoming {
        from: Address,
        transfers: Vec<TransferIn>
    },
    Outgoing {
        transfers: Vec<TransferOut>,
        // Fee paid
        fee: u64,
        // Nonce used
        nonce: u64
    },
    MultiSig {
        // List of participants
        participants: Vec<Address>,
        // Number of signatures required
        threshold: u8,
        // Fee paid
        fee: u64,
        // Nonce used
        nonce: u64
    },
    InvokeContract {
        // Contract address
        contract: Hash,
        // Deposits made
        deposits: IndexMap<Hash, u64>,
        // Chunk id invoked
        chunk_id: u16,
        // Fee paid
        fee: u64,
        // Max gas allowed
        max_gas: u64,
        // Nonce used
        nonce: u64
    },
    DeployContract {
        // Fee paid
        fee: u64,
        // Nonce used
        nonce: u64
    }
}

// This struct is used to represent a transaction entry like in wallet
// But we replace every PublicKey to use Address instead
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEntry {
    pub hash: Hash,
    pub topoheight: TopoHeight,
    pub timestamp: u64,
    #[serde(flatten)]
    pub entry: EntryType,
}

impl std::hash::Hash for TransactionEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl std::cmp::PartialEq for TransactionEntry {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl std::cmp::Eq for TransactionEntry {}

#[derive(Serialize, Deserialize)]
pub struct EstimateExtraDataSizeParams {
    pub destinations: Vec<Address>,
}

#[derive(Serialize, Deserialize)]
pub struct EstimateExtraDataSizeResult {
    // Integrated data size
    pub size: usize,
}