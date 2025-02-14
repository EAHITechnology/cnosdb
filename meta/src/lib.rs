use crate::service::connection::Connections;
use crate::store::command::WriteCommand;
use crate::store::state_machine::CommandResp;
use crate::store::Store;
use openraft::Config;
use openraft::Raft;
use std::fmt::Display;
use std::sync::Arc;
pub mod client;
pub mod error;
pub mod limiter;
pub mod meta_client;
pub mod meta_client_mock;
pub mod service;
pub mod store;
pub mod tenant_manager;
pub mod user_manager;

pub type ClusterNodeId = u64;
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct ClusterNode {
    pub rpc_addr: String,
    pub api_addr: String,
}

impl Display for ClusterNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClusterNode {{ rpc_addr: {}, api_addr: {} }}",
            self.rpc_addr, self.api_addr
        )
    }
}

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig: D = WriteCommand, R = CommandResp, NodeId = ClusterNodeId, Node = ClusterNode
);

pub type RaftStore = Raft<TypeConfig, Connections, Arc<Store>>;
pub struct MetaApp {
    pub id: ClusterNodeId,
    pub http_addr: String,
    pub rpc_addr: String,
    pub raft: RaftStore,
    pub store: Arc<Store>,
    pub config: Arc<Config>,
}
