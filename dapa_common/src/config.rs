use crate::crypto::Hash;

pub const VERSION: &str = env!("BUILD_VERSION");
pub const DAPA_ASSET: Hash = Hash::zero();

// Lowest fee per KB possible on the network
// 0.00010000 DAPA COIN per KB
pub const FEE_PER_KB: u64 = 10000;
// 0.00100000 DAPA COIN per account creation
// User can create an account with 0.000 DAPA COIN
// Or can mine a block to be registered for free
pub const FEE_PER_ACCOUNT_CREATION: u64 = 100000;
// 0.00005000 DAPA COIN per KB
// Each transfer has a overhead of 1000 atomic units
pub const FEE_PER_TRANSFER: u64 = 1000;

// 0.000500 DAPA COIN per multisig signature
// Each signature of a multisig has a overhead of 500 atomic units
pub const FEE_PER_MULTISIG_SIGNATURE: u64 = 500;

// 1 DAPA Coin per contract deployed
// Each contract deployed has a overhead of 1 DAPA COIN
// This amount is burned and is needed for safety of the chain
// Otherwise people could bloat the chain by deploying contracts
// And could make the chain unusable or slow
// Note that a dependence on fees only, miners could do such attacks for free
// by mining their own transactions and getting the fees back
pub const BURN_PER_CONTRACT: u64 = COIN_VALUE;
// 30% of the transaction fee is burned
// This is to reduce the supply over time
// and also to prevent spamming the network with low fee transactions
// or free tx from miners
// This should be enabled once Smart Contracts are released
pub const TRANSACTION_FEE_BURN_PERCENT: u64 = 0;
// Fee per store operation in a contract
// Each store operation has a fixed cost of 0.000001 DAPA COIN
pub const FEE_PER_STORE_CONTRACT: u64 = 100;
// Fee per byte of data stored in a contract
// Each byte of data stored (key + value) in a contract has a fixed cost
// 0.00000005 DAPA COIN per byte
pub const FEE_PER_BYTE_STORED_CONTRACT: u64 = 5;

// 8 decimals numbers
pub const COIN_DECIMALS: u8 = 8;
// 100 000 000 to represent 1 DAPA COIN
pub const COIN_VALUE: u64 = 10u64.pow(COIN_DECIMALS as u32);
// 800M full coins 
pub const MAXIMUM_SUPPLY: u64 = 800_000_000 * COIN_VALUE;

// Addresses format
// mainnet prefix address
pub const PREFIX_ADDRESS: &str = "dap";
// testnet prefix address
pub const TESTNET_PREFIX_ADDRESS: &str = "xel";

// Proof prefix
pub const PREFIX_PROOF: &str = "proof";

// 1 KB = 1024 bytes
pub const BYTES_PER_KB: usize = 1024;

// Max transaction size in bytes
pub const MAX_TRANSACTION_SIZE: usize = BYTES_PER_KB * BYTES_PER_KB; // 1 MB

// Max block size in bytes
// 1024 * 1024 + (256 * 1024) bytes = 1.25 MB maximum size per block with txs
pub const MAX_BLOCK_SIZE: usize = (BYTES_PER_KB * BYTES_PER_KB) + (256 * BYTES_PER_KB);

// BlockDAG rules
pub const TIPS_LIMIT: usize = 3; // maximum 3 TIPS per block