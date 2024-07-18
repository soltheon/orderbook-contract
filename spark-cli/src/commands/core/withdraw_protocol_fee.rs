use crate::utils::{setup, validate_contract_id, AccountType};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use fuels::types::{Address, ContractId, Identity};
use spark_market_sdk::MarketContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Withdraw the protocol fee")]
pub(crate) struct WithdrawProtocolFeeCommand {
    /// The type of account
    #[clap(long)]
    pub(crate) account_to_type: AccountType,

    /// The b256 id of the account
    #[clap(long)]
    pub(crate) account_to_id: String,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl WithdrawProtocolFeeCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        let _ = match self.account_to_type {
            AccountType::Address => {
                let address = Address::from_str(&self.account_to_id).expect("Invalid address");
                contract
                    .withdraw_protocol_fee(Identity::Address(address))
                    .await?
            }
            AccountType::Contract => {
                let address =
                    ContractId::from_str(&self.account_to_id).expect("Invalid contract id");
                contract
                    .withdraw_protocol_fee(Identity::ContractId(address))
                    .await?
            }
        };

        // Balance post-call
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // TODO: replace println with tracing
        println!("\nWithdrawn protocol fee: {}", new_balance - balance);

        Ok(())
    }
}
