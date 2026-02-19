//! Boing node — wires consensus, execution, state, and P2P together.

use boing_primitives::{Account, AccountId, AccountState, Block, Hash, SignedTransaction};
use boing_consensus::ConsensusEngine;
use boing_execution::{BlockExecutor, TransactionScheduler, Vm};
use boing_p2p::{P2pEvent, P2pNode};
use boing_state::StateStore;
use tokio::sync::mpsc;

use crate::block_producer::BlockProducer;
use crate::block_validation::import_block;
use crate::chain::ChainState;
use crate::dapp_registry::DappRegistry;
use crate::intent_pool::IntentPool;
use crate::mempool::{Mempool, MempoolError};
use crate::persistence::{Persistence, PersistenceError};

/// Wraps ChainState to implement BlockProvider for P2P block requests.
pub struct ChainBlockProvider(pub ChainState);

impl boing_p2p::BlockProvider for ChainBlockProvider {
    fn get_block_by_hash(&self, hash: &Hash) -> Option<Block> {
        self.0.get_block_by_hash(hash)
    }
    fn get_block_by_height(&self, height: u64) -> Option<Block> {
        self.0.get_block_by_height(height)
    }
}

/// Full Boing node.
#[allow(dead_code)]
pub struct BoingNode {
    pub chain: ChainState,
    pub consensus: ConsensusEngine,
    pub state: StateStore,
    pub executor: BlockExecutor,
    pub producer: BlockProducer,
    pub vm: Vm,
    pub scheduler: TransactionScheduler,
    pub mempool: Mempool,
    pub p2p: P2pNode,
    pub dapp_registry: DappRegistry,
    pub intent_pool: IntentPool,
    /// Persistence backend; None for in-memory only (e.g. tests).
    pub persistence: Option<Persistence>,
}

impl BoingNode {
    /// Create a node with inert P2P (for tests).
    pub fn new() -> Self {
        let proposer = AccountId([1u8; 32]);
        let genesis = ChainState::genesis(proposer);
        let chain = ChainState::from_genesis(genesis.clone());
        let mut consensus = ConsensusEngine::single_validator(proposer);
        let _ = consensus.propose_and_commit(genesis);

        let mut state = StateStore::new();
        state.insert(Account {
            id: proposer,
            state: AccountState {
                balance: 1_000_000,
                nonce: 0,
                stake: 0,
            },
        });

        Self {
            chain,
            consensus,
            state,
            executor: BlockExecutor::new(),
            producer: BlockProducer::new(proposer).with_max_txs(100),
            vm: Vm::new(),
            scheduler: TransactionScheduler::new(),
            mempool: Mempool::new(),
            p2p: P2pNode::default(),
            dapp_registry: DappRegistry::new(),
            intent_pool: IntentPool::new(),
            persistence: None,
        }
    }

    /// Create a node with optional data directory for persistence.
    /// If data_dir is Some and contains persisted data, loads from disk. Otherwise starts fresh.
    pub fn with_data_dir(
        data_dir: Option<impl AsRef<std::path::Path>>,
    ) -> Result<Self, PersistenceError> {
        let mut node = Self::new();

        if let Some(ref path) = data_dir {
            let path = path.as_ref();
            let persistence = Persistence::new(path);

            if persistence.has_persisted_data() {
                if let Some(chain) = persistence.load_chain()? {
                    node.chain = chain;
                }
                if let Some(state) = persistence.load_state()? {
                    node.state = state;
                }
                let height = node.chain.height();
                node.consensus.sync_round(height);
            }

            node.persistence = Some(persistence);
        }

        Ok(node)
    }

    /// Create a node with live P2P. Returns the node and a receiver for incoming blocks/txs.
    /// Enables block request/response so peers can fetch blocks from us.
    /// When data_dir is Some, enables disk persistence.
    pub fn with_p2p(
        p2p_listen: &str,
        data_dir: Option<impl AsRef<std::path::Path>>,
    ) -> Result<(Self, mpsc::Receiver<P2pEvent>), boing_p2p::P2pError> {
        let mut node = Self::with_data_dir(data_dir).map_err(|e| boing_p2p::P2pError::Network(e.to_string()))?;
        let chain = node.chain.clone();
        let (p2p, event_rx) = P2pNode::new(p2p_listen, Some(std::sync::Arc::new(ChainBlockProvider(chain))))?;
        node.p2p = p2p;
        Ok((node, event_rx))
    }

    fn persist_block_and_state(&self, block: &boing_primitives::Block) {
        if let Some(ref p) = self.persistence {
            if let Err(e) = p.save_block(block) {
                tracing::warn!("Persistence: failed to save block: {}", e);
            }
            if let Err(e) = p.save_chain_meta(block.header.height, block.hash()) {
                tracing::warn!("Persistence: failed to save chain meta: {}", e);
            }
            if let Err(e) = p.save_state(&self.state) {
                tracing::warn!("Persistence: failed to save state: {}", e);
            }
        }
    }

    /// Import a block from the network if it chains to our tip.
    pub fn import_network_block(&mut self, block: &boing_primitives::Block) -> Result<(), crate::block_validation::BlockValidationError> {
        let (latest_hash, height) = (self.chain.latest_hash(), self.chain.height());
        let new_state = import_block(
            block,
            latest_hash,
            height,
            &self.state,
            &self.consensus,
            &self.executor,
        )?;
        self.state = new_state;
        self.chain.append(block.clone()).expect("block chains (validated by import_block)");
        self.consensus.sync_round(block.header.height);
        self.persist_block_and_state(block);
        Ok(())
    }

    /// Submit a signed intent for solver fulfillment.
    pub fn submit_intent(&self, signed: boing_primitives::SignedIntent) -> Result<boing_primitives::Hash, crate::intent_pool::IntentPoolError> {
        self.intent_pool.submit(signed)
    }

    /// Submit a signed transaction to the mempool.
    pub fn submit_transaction(&self, signed: SignedTransaction) -> Result<(), MempoolError> {
        self.mempool.insert(signed)
    }

    /// Produce one block from mempool if there are pending txs.
    /// Broadcasts the block via P2P on success.
    pub fn produce_block_if_ready(&mut self) -> Option<boing_primitives::Hash> {
        let hash = self.producer.produce_block(
            &self.chain,
            &self.mempool,
            &mut self.state,
            &self.executor,
            &mut self.consensus,
        )?;
        if let Some(block) = self.chain.get_block_by_hash(&hash) {
            self.persist_block_and_state(&block);
            let _ = self.p2p.broadcast_block(&block);
        }
        Some(hash)
    }
}

impl Default for BoingNode {
    fn default() -> Self {
        Self::new()
    }
}
