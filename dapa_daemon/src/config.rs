use lazy_static::lazy_static;
use dapa_common::{
    api::daemon::{DevFeeThreshold, HardFork},
    block::BlockVersion,
    config::MAX_BLOCK_SIZE,
    crypto::{
        Address,
        Hash,
        PublicKey
    },
    difficulty::Difficulty,
    network::Network,
    time::TimestampSeconds
};

// In case of potential forks, have a unique network id to not connect to others compatible chains
pub const NETWORK_ID_SIZE: usize = 16;
pub const NETWORK_ID: [u8; NETWORK_ID_SIZE] = [0x21, 0x4c, 0x4, 0x31, 0x11, 0x8f, 0x46, 0x65, 0x9c, 0x39, 0x65, 0x4f, 0x78, 0x6c, 0x8f, 0x44];

// bind addresses
pub const DEFAULT_P2P_BIND_ADDRESS: &str = "0.0.0.0:20100";
pub const DEFAULT_RPC_BIND_ADDRESS: &str = "0.0.0.0:20101";

// Default cache size for storage DB
pub const DEFAULT_CACHE_SIZE: usize = 1024;

// Block rules
// Millis per second, it is used to prevent having random 1000 values anywhere
pub const MILLIS_PER_SECOND: u64 = 1000;
// Block Time in milliseconds
pub const BLOCK_TIME_MILLIS: u64 = 15 * MILLIS_PER_SECOND; // 15s block time
// Minimum difficulty (each difficulty point is in H/s)
// Current: BLOCK TIME in millis * 20 = 20 KH/s minimum
// This is to prevent spamming the network with low difficulty blocks
// This is active only on mainnet mode
pub const MAINNET_MINIMUM_DIFFICULTY: Difficulty = Difficulty::from_u64(BLOCK_TIME_MILLIS * 20);
// Testnet & Devnet minimum difficulty
pub const OTHER_MINIMUM_DIFFICULTY: Difficulty = Difficulty::from_u64(BLOCK_TIME_MILLIS * 2);
// This is also used as testnet and devnet minimum difficulty
pub const GENESIS_BLOCK_DIFFICULTY: Difficulty = Difficulty::from_u64(1);
// 2 seconds maximum in future (prevent any attack on reducing difficulty but keep margin for unsynced devices)
pub const TIMESTAMP_IN_FUTURE_LIMIT: TimestampSeconds = 2 * 1000;

// keep at least last N blocks until top topoheight when pruning the chain
// WARNING: This must be at least 50 blocks for difficulty adjustement
pub const PRUNE_SAFETY_LIMIT: u64 = STABLE_LIMIT * 10;

// BlockDAG rules
pub const STABLE_LIMIT: u64 = 8; // in how many height we consider the block stable

// Pre-mine configuration providing for exchange liquidity
// 50 million coins pre-mined at genesis to provide adequate liquidity for exchange listings
// This ensures stable price discovery and reduces volatility during early adoption phase
pub const PREMINE_AMOUNT: u64 = 50_000_000 * 100_000_000; // 50M coins in atomic units
pub const PREMINE_ADDRESS: &str = "dap:vnytpjcq8z84prl46wtn7hdrqvu823dzvss7aj6fm4jjxqtxsa9qqz9lrvx"; // Using existing dev address

// Emission rules
// New emission rules are: 25% for 14.545 blocks then 3% for the rest
// This is the same for the project but reduce a bit the mining cost as they earn 5% more
pub const DEV_FEES: [DevFeeThreshold; 2] = [
    // Activated for 
    DevFeeThreshold {
        height: 0,
        fee_percentage: 25
    },
    // Activated for the rest
    DevFeeThreshold {
        // areduced to 3%
        //* 15s of block time 
        height: 14_545, 
        fee_percentage: 3
    }
];
// only 30% of reward for side block
// This is to prevent spamming side blocks
// and also give rewards for miners with valid work on main chain
pub const SIDE_BLOCK_REWARD_PERCENT: u64 = 30;
// maximum 3 blocks for side block reward
// Each side block reward will be divided by the number of side blocks * 2
// With a configuration of 3 blocks, we have the following percents:
// 1 block: 30%
// 2 blocks: 15%
// 3 blocks: 7%
// 4 blocks: minimum percentage set below
pub const SIDE_BLOCK_REWARD_MAX_BLOCKS: u64 = 3;
// minimum 5% of block reward for side block
// This is the minimum given for all others valid side blocks
pub const SIDE_BLOCK_REWARD_MIN_PERCENT: u64 = 5;
// Emission speed factor for the emission curve
// It is used to calculate based on the supply the block reward
pub const EMISSION_SPEED_FACTOR: u64 = 23;

// Developer address for paying dev fees until Smart Contracts integration
// (testnet/mainnet format is converted lazily later)
pub const DEV_ADDRESS: &str = "dap:vnytpjcq8z84prl46wtn7hdrqvu823dzvss7aj6fm4jjxqtxsa9qqz9lrvx";

// Chain sync config
// minimum X seconds between each chain sync request per peer
pub const CHAIN_SYNC_DELAY: u64 = 5;
// wait maximum between each chain sync request to peers
pub const CHAIN_SYNC_TIMEOUT_SECS: u64 = CHAIN_SYNC_DELAY * 3;
// first 30 blocks are sent in linear way, then it's exponential
pub const CHAIN_SYNC_REQUEST_EXPONENTIAL_INDEX_START: usize = 30;
// allows up to X blocks id (hash + height) sent for request
pub const CHAIN_SYNC_REQUEST_MAX_BLOCKS: usize = 64;
// minimum X blocks hashes sent for response
pub const CHAIN_SYNC_RESPONSE_MIN_BLOCKS: usize = 512;
// Default response blocks sent/accepted
pub const CHAIN_SYNC_DEFAULT_RESPONSE_BLOCKS: usize = 4096;
// allows up to X blocks hashes sent for response
pub const CHAIN_SYNC_RESPONSE_MAX_BLOCKS: usize = 16384;
// send last 10 heights
pub const CHAIN_SYNC_TOP_BLOCKS: usize = 10;

// P2p rules
// time between each ping
pub const P2P_PING_DELAY: u64 = 10;
// time in seconds between each update of peerlist
pub const P2P_PING_PEER_LIST_DELAY: u64 = 60 * 5;
// maximum number of addresses to be send
pub const P2P_PING_PEER_LIST_LIMIT: usize = 16;
// default number of maximum peers
pub const P2P_DEFAULT_MAX_PEERS: usize = 32;
// time in seconds between each time we try to connect to a new peer
pub const P2P_EXTEND_PEERLIST_DELAY: u64 = 60;
// time in seconds between each time we try to connect to a outgoing peer
// At least 5 minutes of countdown to retry to connect to the same peer
// This will be multiplied by the number of fails
pub const P2P_PEERLIST_RETRY_AFTER: u64 = 60 * 15;
// Peer wait on error accept new p2p connections in seconds
pub const P2P_PEER_WAIT_ON_ERROR: u64 = 15;
// Delay in second to connect to priority nodes
pub const P2P_AUTO_CONNECT_PRIORITY_NODES_DELAY: u64 = 5;
// Default number of concurrent tasks for incoming p2p connections
pub const P2P_DEFAULT_CONCURRENCY_TASK_COUNT_LIMIT: usize = 4;
// Heartbeat interval in seconds to check if peer is still alive
pub const P2P_HEARTBEAT_INTERVAL: u64 = P2P_PING_DELAY / 2;
// Timeout in seconds
// If we didn't receive any packet from a peer during this time, we disconnect it
pub const P2P_PING_TIMEOUT: u64 = P2P_PING_DELAY * 6;

// Peer rules
// number of seconds to reset the counter
// Set to 30 minutes
pub const PEER_FAIL_TIME_RESET: u64 = 30 * 60;
// number of fail to disconnect the peer
pub const PEER_FAIL_LIMIT: u8 = 50;
// number of fail during handshake before temp ban
pub const PEER_FAIL_TO_CONNECT_LIMIT: u8 = 3;
// number of seconds to temp ban the peer in case of fail reached during handshake
// It is only used for incoming connections
// Set to 1 minute
pub const PEER_TEMP_BAN_TIME_ON_CONNECT: u64 = 60;
// number of seconds to temp ban the peer in case of fail count limit (`PEER_FAIL_LIMIT`) reached
// Set to 15 minutes
pub const PEER_TEMP_BAN_TIME: u64 = 15 * 60;
// millis until we timeout
pub const PEER_TIMEOUT_REQUEST_OBJECT: u64 = 15_000;
// millis until we timeout during a bootstrap request
pub const PEER_TIMEOUT_BOOTSTRAP_STEP: u64 = 60_000;
// millis until we timeout during a handshake
pub const PEER_TIMEOUT_INIT_CONNECTION: u64 = 5_000;
// millis until we timeout during outgoing connection try
pub const PEER_TIMEOUT_INIT_OUTGOING_CONNECTION: u64 = 30_000;
// millis until we timeout during a handshake
pub const PEER_TIMEOUT_DISCONNECT: u64 = 1_500;
// 16 additional bytes are for AEAD from ChaCha20Poly1305
pub const PEER_MAX_PACKET_SIZE: u32 = MAX_BLOCK_SIZE as u32 + 16;
// Peer TX cache size
// This is how many elements are stored in the LRU cache at maximum
pub const PEER_TX_CACHE_SIZE: usize = 10240;
// How many peers propagated are stored per peer in the LRU cache at maximum
pub const PEER_PEERS_CACHE_SIZE: usize = 1024;
// Peer Block cache size
pub const PEER_BLOCK_CACHE_SIZE: usize = 1024;
// Peer packet channel size
pub const PEER_PACKET_CHANNEL_SIZE: usize = 1024;
// Peer timeout for packet channel
// Millis
pub const PEER_SEND_BYTES_TIMEOUT: u64 = 3_000;

// Hard Forks configured
const HARD_FORKS: [HardFork; 3] = [
    HardFork {
        height: 0,
        version: BlockVersion::V0,
        changelog: "Initial version",
        version_requirement: None
    },
    HardFork {
        // Expected date: 10/07/2024 12am UTC
        height: 434_100,
        version: BlockVersion::V1,
        changelog: "xelis-hash v2",
        version_requirement: Some(">=1.13.0")
    },
    HardFork {
        // Expected date: 30/12/2024 9pm UTC
        height: 1_376_000,
        version: BlockVersion::V2,
        changelog: "MultiSig, P2P",
        version_requirement: Some(">=1.16.0")
    }
];

// Testnet / Devnet hard forks
const TESTNET_HARD_FORKS: [HardFork; 4] = [
    HardFork {
        height: 0,
        version: BlockVersion::V0,
        changelog: "Initial version",
        version_requirement: None
    },
    HardFork {
        height: 5,
        version: BlockVersion::V1,
        changelog: "xelis-hash v2",
        version_requirement: Some(">=1.13.0")
    },
    HardFork {
        height: 10,
        version: BlockVersion::V2,
        changelog: "MultiSig, P2P",
        version_requirement: Some(">=1.16.0")
    },
    HardFork {
        height: 15,
        version: BlockVersion::V3,
        changelog: "Smart Contracts",
        version_requirement: Some(">=1.16.0")
    }
];

// Mainnet seed nodes
const MAINNET_SEED_NODES: [&str; 3] = [
     // France
    "185.198.27.165:20100",
    // Germany
    "45.84.138.159:20100",
    //singapore
    "194.163.189.149:20100"
];

// Testnet seed nodes
const TESTNET_SEED_NODES: [&str; 0] = [
   
];

// Genesis block to have the same starting point for every nodes
// Genesis block in hexadecimal format
const MAINNET_GENESIS_BLOCK: &str = "00000000000000000000000197eda17a440000000000000000000000000000000000000000000000000000000000000000000000000000000000000064c8b0cb00388f508ff5d3973f5da303387545a26421eecb49dd65230166874a";
const TESTNET_GENESIS_BLOCK: &str = "0000000000000000000000018f116b47cf000000000000000000000000000000000000000000000000000000000000000000000000000000000000006423b4908e5bd32241e3443fccfb7bab86a899a8cca12b3fedf255634d156d66";

// Genesis block hash for both networks
// It must be the same as the hash of the genesis block
const MAINNET_GENESIS_BLOCK_HASH: Hash = Hash::new([200, 136, 173, 12, 139, 40, 70, 152, 29, 39, 9, 185, 197, 2, 58, 96, 155, 61, 229, 48, 134, 20, 120, 148, 88, 45, 254, 210, 219, 250, 173, 26]);
const TESTNET_GENESIS_BLOCK_HASH: Hash = Hash::new([145, 152, 150, 21, 160, 79, 96, 50, 124, 213, 35, 97, 129, 129, 175, 9, 145, 52, 98, 164, 191, 88, 87, 152, 45, 178, 119, 134, 215, 2, 177, 180]);

// Genesis block getter
// This is necessary to prevent having the same Genesis Block for differents network
// Dev returns none to generate a new genesis block each time it starts a chain
pub fn get_hex_genesis_block(network: &Network) -> Option<&str> {
    match network {
        Network::Mainnet => Some(MAINNET_GENESIS_BLOCK),
        Network::Testnet => Some(TESTNET_GENESIS_BLOCK),
        Network::Dev => None
    }
}

lazy_static! {
    // Developer public key is lazily converted from address to support any network
    pub static ref DEV_PUBLIC_KEY: PublicKey = Address::from_string(&DEV_ADDRESS).unwrap().to_public_key();
    
    // Pre-mine public key is lazily converted from address to support any network
    pub static ref PREMINE_PUBLIC_KEY: PublicKey = Address::from_string(&PREMINE_ADDRESS).unwrap().to_public_key();
}

// Genesis block hash based on network selected
pub fn get_genesis_block_hash(network: &Network) -> Option<&'static Hash> {
    match network {
        Network::Mainnet => Some(&MAINNET_GENESIS_BLOCK_HASH),
        Network::Testnet => Some(&TESTNET_GENESIS_BLOCK_HASH),
        Network::Dev => None
    }
}

// Get seed nodes based on the network used
pub const fn get_seed_nodes(network: &Network) -> &[&str] {
    match network {
        Network::Mainnet => &MAINNET_SEED_NODES,
        Network::Testnet => &TESTNET_SEED_NODES,
        Network::Dev => &[],
    }
}

// Get minimum difficulty based on the network
// Mainnet has a minimum difficulty to prevent spamming the network
// Testnet has a lower difficulty to allow faster block generation
pub fn get_minimum_difficulty(network: &Network) -> Difficulty {
    match network {
        Network::Mainnet => MAINNET_MINIMUM_DIFFICULTY,
        _ => OTHER_MINIMUM_DIFFICULTY,
    }
}

pub fn get_difficulty_at_hard_fork(network: &Network, block_version: BlockVersion) -> Difficulty {
    match network {
        Network::Mainnet => match block_version {
            BlockVersion::V0 | BlockVersion::V1 => MAINNET_MINIMUM_DIFFICULTY,
            // 20 KH/s * 100 000 = 2 GH/s
            _ => MAINNET_MINIMUM_DIFFICULTY * Difficulty::from_u64(100_000),
        },
        _ => OTHER_MINIMUM_DIFFICULTY,
    }
}

// Get hard forks based on the network
pub const fn get_hard_forks(network: &Network) -> &[HardFork] {
    match network {
        Network::Mainnet => &HARD_FORKS,
        _ => &TESTNET_HARD_FORKS,
    }
}

// Get the pre-mine amount
pub const fn get_premine_amount() -> u64 {
    PREMINE_AMOUNT
}

// Get the pre-mine address
pub const fn get_premine_address() -> &'static str {
    PREMINE_ADDRESS
}
