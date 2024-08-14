use anyhow::Result;
use near_sdk::NearToken;
use near_workspaces::Account;
use nitka::{misc::ToNear, near_sdk::json_types::U128};
use sweat_booster_model::api::{InitApiIntegration, SweatBoosterContract};
use sweat_model::{FungibleTokenCoreIntegration, StorageManagementIntegration, SweatApiIntegration, SweatContract};

pub type Context = nitka::context::Context<near_workspaces::network::Sandbox>;

pub const FT_CONTRACT: &str = "sweat";
pub const SWEAT_BOOSTER: &str = "sweat_booster";

pub trait IntegrationContext {
    async fn manager(&mut self) -> Result<Account>;
    async fn alice(&mut self) -> Result<Account>;
    fn sweat_booster(&self) -> SweatBoosterContract;
    fn ft_contract(&self) -> SweatContract;
}

impl IntegrationContext for Context {
    async fn manager(&mut self) -> Result<Account> {
        self.account("manager").await
    }

    async fn alice(&mut self) -> Result<Account> {
        self.account("alice").await
    }

    fn sweat_booster(&self) -> SweatBoosterContract {
        SweatBoosterContract {
            contract: &self.contracts[SWEAT_BOOSTER],
        }
    }

    fn ft_contract(&self) -> SweatContract {
        SweatContract {
            contract: &self.contracts[FT_CONTRACT],
        }
    }
}

pub async fn prepare_contract() -> Result<Context> {
    let mut context = Context::new(&[FT_CONTRACT, SWEAT_BOOSTER], true, "build-integration".into()).await?;

    let alice = context.alice().await?;
    let bob = context.account("bob").await?;
    let manager = context.manager().await?;

    context.ft_contract().new(".u.sweat.testnet".to_string().into()).await?;
    context
        .sweat_booster()
        .new(
            context.ft_contract().contract.as_account().to_near(),
            manager.to_near(),
            None,
        )
        .await?;

    context
        .ft_contract()
        .storage_deposit(context.sweat_booster().contract.as_account().to_near().into(), None)
        .await?;

    context
        .ft_contract()
        .tge_mint(
            &context.sweat_booster().contract.as_account().to_near(),
            U128(100_000_000),
        )
        .await?;

    context
        .ft_contract()
        .storage_deposit(alice.to_near().into(), None)
        .await?;
    context
        .ft_contract()
        .tge_mint(&alice.to_near(), U128(100_000_000))
        .await?;
    context
        .ft_contract()
        .storage_deposit(bob.to_near().into(), None)
        .await?;
    context
        .ft_contract()
        .tge_mint(&bob.to_near(), U128(100_000_000))
        .await?;
    context
        .ft_contract()
        .tge_mint(&manager.to_near(), U128(100_000_000))
        .await?;

    Ok(context)
}

pub trait ContextHelpers {
    async fn account_balance(&self, account: &Account) -> Result<u128>;
}

impl ContextHelpers for Context {
    async fn account_balance(&self, account: &Account) -> Result<u128> {
        let balance = self.ft_contract().ft_balance_of(account.to_near()).await?.0;
        Ok(balance)
    }
}
