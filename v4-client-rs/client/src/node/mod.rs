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
pub mod dydx_proto;

// Explicit re-exports to simplify usage downstream
pub use builder::TxBuilder;
pub use client::{error::*, Address, Authenticator, NodeClient, Subaccount, TxHash};
pub use config::NodeConfig;
pub use order::*;
pub use sequencer::*;
pub use types::ChainId;
pub use utils::BigIntExt;
pub use wallet::{Account, PublicAccount, Wallet};

pub use dydx_proto::dydxprotocol; // So you can use `dydx::dydxprotocol::clob::Order` consistently
