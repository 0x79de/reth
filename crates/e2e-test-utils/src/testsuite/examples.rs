//! Example tests using the test suite framework.

use crate::testsuite::{
    actions::{AssertMineBlock, CaptureBlock, CreateFork, ProduceBlocks, ReorgTo},
    setup::{NetworkSetup, Setup},
    TestBuilder,
};
use alloy_primitives::{Address, B256};
use alloy_rpc_types_engine::PayloadAttributes;
use eyre::Result;
use reth_chainspec::{ChainSpecBuilder, MAINNET};
use reth_node_ethereum::{EthEngineTypes, EthereumNode};
use std::sync::Arc;

#[tokio::test]
async fn test_testsuite_assert_mine_block() -> Result<()> {
    reth_tracing::init_test_tracing();

    let setup = Setup::default()
        .with_chain_spec(Arc::new(
            ChainSpecBuilder::default()
                .chain(MAINNET.chain)
                .genesis(serde_json::from_str(include_str!("assets/genesis.json")).unwrap())
                .paris_activated()
                .build(),
        ))
        .with_network(NetworkSetup::single_node());

    let test =
        TestBuilder::new().with_setup(setup).with_action(AssertMineBlock::<EthEngineTypes>::new(
            0,
            vec![],
            Some(B256::ZERO),
            // TODO: refactor once we have actions to generate payload attributes.
            PayloadAttributes {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                prev_randao: B256::random(),
                suggested_fee_recipient: Address::random(),
                withdrawals: None,
                parent_beacon_block_root: None,
            },
        ));

    test.run::<EthereumNode>().await?;

    Ok(())
}

#[tokio::test]
async fn test_testsuite_produce_blocks() -> Result<()> {
    reth_tracing::init_test_tracing();

    let setup = Setup::default()
        .with_chain_spec(Arc::new(
            ChainSpecBuilder::default()
                .chain(MAINNET.chain)
                .genesis(serde_json::from_str(include_str!("assets/genesis.json")).unwrap())
                .cancun_activated()
                .build(),
        ))
        .with_network(NetworkSetup::single_node());

    let test =
        TestBuilder::new().with_setup(setup).with_action(ProduceBlocks::<EthEngineTypes>::new(5));

    test.run::<EthereumNode>().await?;

    Ok(())
}

#[tokio::test]
async fn test_testsuite_create_fork() -> Result<()> {
    reth_tracing::init_test_tracing();

    let setup = Setup::default()
        .with_chain_spec(Arc::new(
            ChainSpecBuilder::default()
                .chain(MAINNET.chain)
                .genesis(serde_json::from_str(include_str!("assets/genesis.json")).unwrap())
                .cancun_activated()
                .build(),
        ))
        .with_network(NetworkSetup::single_node());

    let test = TestBuilder::new()
        .with_setup(setup)
        .with_action(ProduceBlocks::<EthEngineTypes>::new(2))
        .with_action(CreateFork::<EthEngineTypes>::new(1, 3));

    test.run::<EthereumNode>().await?;

    Ok(())
}

#[tokio::test]
async fn test_testsuite_reorg_with_tagging() -> Result<()> {
    reth_tracing::init_test_tracing();

    let setup = Setup::default()
        .with_chain_spec(Arc::new(
            ChainSpecBuilder::default()
                .chain(MAINNET.chain)
                .genesis(serde_json::from_str(include_str!("assets/genesis.json")).unwrap())
                .cancun_activated()
                .build(),
        ))
        .with_network(NetworkSetup::single_node());

    let test = TestBuilder::new()
        .with_setup(setup)
        .with_action(ProduceBlocks::<EthEngineTypes>::new(3)) // produce blocks 1, 2, 3
        .with_action(CaptureBlock::new("main_chain_tip")) // tag block 3 as "main_chain_tip"
        .with_action(CreateFork::<EthEngineTypes>::new(1, 2)) // fork from block 1, produce blocks 2', 3'
        .with_action(ReorgTo::<EthEngineTypes>::new_from_tag("main_chain_tip")); // reorg back to tagged block 3

    test.run::<EthereumNode>().await?;

    Ok(())
}
