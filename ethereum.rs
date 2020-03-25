mod cache;
mod web3_connector;

pub use self::{cache::Cache, web3_connector::Web3Connector};
use crate::{
    btsieve::{
        find_relevant_blocks, BlockByHash, BlockHash, LatestBlock, Predates, PreviousBlockHash,
    },
    ethereum::{Address, Bytes, Input, Log, Transaction, TransactionReceipt, H256, U256},
};
use anyhow;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use genawaiter::{sync::Gen, GeneratorState};

type Hash = H256;
type Block = crate::ethereum::Block;

// The software this file comes from aims to solve the problem of atomic swap of
// digital assets across ledgers.  This is some of the Ethereum code.
//
// This file implements the matching logic for finding blocks, on the Ethereum
// ledger, that contain the transactions we are looking for.  As such it
// represents the core functionality of this codebase (at least as far as
// Ethereum is concerned) since looking for transactions on the respective
// ledgers is our primary task.
//
// The main entry points are `watch_for_contract_creation()` and
// `watch_for_event()`.  Both call `matching_transaction_and_receipt()`, note
// the use of the closure to do the actual matching. 

#[async_trait]
pub trait ReceiptByHash: Send + Sync + 'static {
    async fn receipt_by_hash(&self, transaction_hash: Hash) -> anyhow::Result<TransactionReceipt>;
}

impl BlockHash for Block {
    type BlockHash = Hash;

    fn block_hash(&self) -> H256 {
        self.hash
            .expect("Connector returned latest block with null hash")
    }
}

impl PreviousBlockHash for Block {
    type BlockHash = Hash;

    fn previous_block_hash(&self) -> H256 {
        self.parent_hash
    }
}

pub async fn watch_for_contract_creation<C>(
    blockchain_connector: &C,
    start_of_swap: NaiveDateTime,
    bytecode: Bytes,
) -> anyhow::Result<(Transaction, Address)>
where
    C: LatestBlock<Block = Block> + BlockByHash<Block = Block, BlockHash = Hash> + ReceiptByHash,
{
    let (transaction, receipt) =
        matching_transaction_and_receipt(blockchain_connector, start_of_swap, |transaction| {
            // transaction.to address is None if, and only if, the transaction
            // creates a contract.
            transaction.to.is_none() && transaction.input == bytecode
        })
        .await?;

    match receipt.contract_address {
        Some(location) => Ok((transaction, location)),
        None => Err(anyhow::anyhow!("contract address missing from receipt")),
    }
}

pub async fn watch_for_event<C>(
    blockchain_connector: &C,
    start_of_swap: NaiveDateTime,
    event: Event,
) -> anyhow::Result<(Transaction, Log)>
where
    C: LatestBlock<Block = Block> + BlockByHash<Block = Block, BlockHash = Hash> + ReceiptByHash,
{
    matching_transaction_and_log(
        blockchain_connector,
        start_of_swap,
        event.topics.clone(),
        |receipt| find_log_for_event_in_receipt(&event, receipt),
    )
    .await
}

/// Fetch receipt from connector using transaction hash.
async fn fetch_receipt<C>(
    blockchain_connector: &C,
    hash: Hash,
) -> anyhow::Result<TransactionReceipt>
where
    C: ReceiptByHash,
{
    let receipt = blockchain_connector.receipt_by_hash(hash).await?;
    Ok(receipt)
}

fn find_log_for_event_in_receipt(event: &Event, receipt: TransactionReceipt) -> Option<Log> {
    match event {
        Event { topics, .. } if topics.is_empty() => None,
        Event { address, topics } => receipt.logs.into_iter().find(|log| {
            if address != &log.address {
                return false;
            }

            if log.topics.len() != topics.len() {
                return false;
            }

            log.topics.iter().enumerate().all(|(index, tx_topic)| {
                let topic = &topics[index];
                topic.as_ref().map_or(true, |topic| tx_topic == &topic.0)
            })
        }),
    }
}

pub async fn matching_transaction_and_receipt<C, F>(
    connector: &C,
    start_of_swap: NaiveDateTime,
    matcher: F,
) -> anyhow::Result<(Transaction, TransactionReceipt)>
where
    C: LatestBlock<Block = Block> + BlockByHash<Block = Block, BlockHash = Hash> + ReceiptByHash,
    F: Fn(&Transaction) -> bool,
{
    let mut block_generator =
        Gen::new({ |co| async { find_relevant_blocks(connector, co, start_of_swap).await } });

    loop {
        match block_generator.async_resume().await {
            GeneratorState::Yielded(block) => {
                for transaction in block.transactions.into_iter() {
                    if matcher(&transaction) {
                        let receipt = fetch_receipt(connector, transaction.hash).await?;
                        if !receipt.is_status_ok() {
                            // This can be caused by a failed attempt to complete an action,
                            // for example, sending a transaction with low gas.
                            tracing::warn!(
                                "transaction matched {:x} but status was NOT OK",
                                transaction.hash,
                            );
                            continue;
                        }
                        tracing::trace!("transaction matched {:x}", transaction.hash,);
                        return Ok((transaction, receipt));
                    }
                }
            }
            GeneratorState::Complete(Err(e)) => return Err(e),
            // By matching against the never type explicitly, we assert that the `Ok` value of the
            // result is actually the never type and has not been changed since this line was
            // written. The never type can never be constructed, so we can never reach this line.
            GeneratorState::Complete(Ok(never)) => match never {},
        }
    }
}

async fn matching_transaction_and_log<C, F>(
    connector: &C,
    start_of_swap: NaiveDateTime,
    topics: Vec<Option<Topic>>,
    matcher: F,
) -> anyhow::Result<(Transaction, Log)>
where
    C: LatestBlock<Block = Block> + BlockByHash<Block = Block, BlockHash = Hash> + ReceiptByHash,
    F: Fn(TransactionReceipt) -> Option<Log>,
{
    let mut block_generator =
        Gen::new({ |co| async { find_relevant_blocks(connector, co, start_of_swap).await } });

    loop {
        match block_generator.async_resume().await {
            GeneratorState::Yielded(block) => {
                let block_hash = block
                    .hash
                    .ok_or_else(|| anyhow::anyhow!("block without hash"))?;

                let maybe_contains_transaction = topics.iter().all(|topic| {
                    topic.as_ref().map_or(true, |topic| {
                        block
                            .logs_bloom
                            .contains_input(Input::Raw(topic.0.as_ref()))
                    })
                });
                if !maybe_contains_transaction {
                    tracing::trace!(
                        "bloom filter indicates that block does not contain transaction:
                {:x}",
                        block_hash,
                    );
                    continue;
                }

                tracing::trace!(
                    "bloom filter indicates that we should check the block for transactions: {:x}",
                    block_hash,
                );
                for transaction in block.transactions.into_iter() {
                    let receipt = fetch_receipt(connector, transaction.hash).await?;
                    let status_is_ok = receipt.is_status_ok();
                    if let Some(log) = matcher(receipt) {
                        if !status_is_ok {
                            // This can be caused by a failed attempt to complete an action,
                            // for example, sending a transaction with low gas.
                            tracing::warn!(
                                "transaction matched {:x} but status was NOT OK",
                                transaction.hash,
                            );
                            continue;
                        }
                        tracing::trace!("transaction matched {:x}", transaction.hash,);
                        return Ok((transaction, log));
                    }
                }
            }
            GeneratorState::Complete(Err(e)) => return Err(e),
            // By matching against the never type explicitly, we assert that the `Ok` value of the
            // result is actually the never type and has not been changed since this line was
            // written. The never type can never be constructed, so we can never reach this line.
            GeneratorState::Complete(Ok(never)) => match never {},
        }
    }
}

impl Predates for Block {
    fn predates(&self, timestamp: NaiveDateTime) -> bool {
        let unix_timestamp = timestamp.timestamp();

        self.timestamp < U256::from(unix_timestamp)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, serde::Serialize, serdebug::SerDebug)]
#[serde(transparent)]
pub struct Topic(pub H256);

/// Event works similar to web3 filters:
/// https://web3js.readthedocs.io/en/1.0/web3-eth-subscribe.html?highlight=filter#subscribe-logs
/// For example, this `Event` would match this `Log`:
/// ```rust, ignore
/// 
/// Event {
///     address: "0xe46FB33e4DB653De84cB0E0E8b810A6c4cD39d59",
///     topics: [
///         None,
///         Some("0x000000000000000000000000e46fb33e4db653de84cb0e0e8b810a6c4cd39d59"),
///         None,
///     ],
/// }
///
/// Log: {
///     address: "0xe46FB33e4DB653De84cB0E0E8b810A6c4cD39d59",
///     data: "0x123",
///     topics: [
///         "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
///         "0x000000000000000000000000e46fb33e4db653de84cb0e0e8b810a6c4cd39d59",
///         "0x000000000000000000000000d51ecee7414c4445534f74208538683702cbb3e4",
///     ]
///     ...  // Other data omitted
/// }
/// ```
#[derive(Clone, Default, Eq, PartialEq, serde::Serialize, serdebug::SerDebug)]
pub struct Event {
    pub address: Address,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<Option<Topic>>,
}
