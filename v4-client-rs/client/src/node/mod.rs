mod builder;
mod client;
mod config;
mod fee;
pub mod order;
pub mod sequencer;
mod types;
mod utils;
mod wallet;

// Bring in the proto definitions from the local submodule (forked version)

// Explicit re-exports to simplify usage downstream
pub use builder::TxBuilder;
pub use client::{error::*, Address, Authenticator, NodeClient, Subaccount, TxHash};
pub use config::NodeConfig;
pub use order::*;
pub use sequencer::*;
pub use types::ChainId;
pub use utils::BigIntExt;
pub use wallet::{Account, PublicAccount, Wallet};

