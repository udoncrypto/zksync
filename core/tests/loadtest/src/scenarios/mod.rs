//! Module with different scenarios for a `loadtest`.
//! A scenario is basically is a behavior policy for sending the transactions.
//! A simplest scenario will be: "get a bunch of accounts and just spawn a lot of transfer
//! operations between them".

pub use self::{
    executor::ScenarioExecutor, transfers::TransferScenarioConfig, withdraw::WithdrawScenarioConfig,
};

// Built-in uses
use std::fmt::{Debug, Display};
// External uses
use async_trait::async_trait;
use num::BigUint;
use serde::{Deserialize, Serialize};
// Workspace uses
// Local uses
use self::{transfers::TransferScenario, withdraw::WithdrawScenario};
use crate::{monitor::Monitor, test_wallet::TestWallet};

mod executor;
mod transfers;
mod withdraw;

/// Resources that are needed from the scenario executor to perform the scenario.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ScenarioResources {
    /// Total amount of non-empty wallets.
    pub wallets_amount: u64,
    /// Wei balance in each wallet.
    pub balance_per_wallet: BigUint,
}

#[async_trait]
pub trait Scenario: Debug + Display {
    /// Returns resources that should be provided by the scenario executor.
    fn requested_resources(&self, sufficient_fee: &BigUint) -> ScenarioResources;

    async fn prepare(
        &mut self,
        monitor: &Monitor,
        sufficient_fee: &BigUint,
        wallets: &[TestWallet],
    ) -> anyhow::Result<()>;

    async fn run(
        &mut self,
        monitor: &Monitor,
        sufficient_fee: &BigUint,
        wallets: &[TestWallet],
    ) -> anyhow::Result<()>;

    async fn finalize(
        &mut self,
        monitor: &Monitor,
        sufficient_fee: &BigUint,
        wallets: &[TestWallet],
    ) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum ScenarioConfig {
    Transfer(TransferScenarioConfig),
    Withdraw(WithdrawScenarioConfig),
}

impl ScenarioConfig {
    pub fn into_scenario(self) -> Box<dyn Scenario> {
        match self {
            Self::Transfer(cfg) => Box::new(TransferScenario::from(cfg)),
            Self::Withdraw(cfg) => Box::new(WithdrawScenario::from(cfg)),
        }
    }
}

impl From<TransferScenarioConfig> for ScenarioConfig {
    fn from(cfg: TransferScenarioConfig) -> Self {
        Self::Transfer(cfg)
    }
}
