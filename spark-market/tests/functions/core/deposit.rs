use crate::setup::{setup, Defaults};
use rand::Rng;

mod success {

    use super::*;
    use crate::setup::create_account;
    use spark_market_sdk::DepositEvent;

    #[tokio::test]
    async fn base_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 100;
        let expected_account = create_account(0, 0, 0, 0);

        // Precondition enforces empty account
        assert_eq!(
            contract.account(owner.identity()).await?.value,
            expected_account
        );
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let user_balance = owner.balance(&assets.base.id).await;
        let response = contract.deposit(deposit_amount, assets.base.id).await?;
        let new_balance = owner.balance(&assets.base.id).await;
        assert_eq!(new_balance, user_balance - deposit_amount);

        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            DepositEvent {
                amount: deposit_amount,
                asset: assets.base.id,
                user: owner.identity(),
                account: expected_account.clone(),
                caller: owner.identity(),
            }
        );

        let user_account = contract.account(owner.identity()).await?.value;

        assert_eq!(user_account, expected_account);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn fuzz_base_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();

        for _ in 0..100 {
            let (contract, owner, _user, _, _, assets) = setup(
                defaults.base_decimals,
                defaults.quote_decimals,
                defaults.price_decimals,
            )
            .await?;

            // Generate a random deposit amount
            let deposit_amount = rand::thread_rng().gen_range(1..1_000_000_000_000);
            let expected_account = create_account(0, 0, 0, 0);

            assert_eq!(
                contract.account(owner.identity()).await?.value,
                expected_account
            );
            let expected_account = create_account(deposit_amount, 0, 0, 0);

            let user_balance = owner.balance(&assets.base.id).await;
            let response = contract.deposit(deposit_amount, assets.base.id).await?;
            let new_balance = owner.balance(&assets.base.id).await;
            assert_eq!(new_balance, user_balance - deposit_amount);

            let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
            let event = log.first().unwrap();
            assert_eq!(
                *event,
                DepositEvent {
                    amount: deposit_amount,
                    asset: assets.base.id,
                    user: owner.identity(),
                    account: expected_account.clone(),
                    caller: owner.identity(),
                }
            );

            let user_account = contract.account(owner.identity()).await?.value;

            assert_eq!(user_account, expected_account);
        }

        Ok(())
    }

    #[tokio::test]
    async fn quote_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 100;
        let expected_account = create_account(0, 0, 0, 0);

        // Precondition enforces empty account
        assert_eq!(
            contract.account(owner.identity()).await?.value,
            expected_account
        );
        let expected_account = create_account(0, deposit_amount, 0, 0);

        let user_balance = owner.balance(&assets.quote.id).await;
        let response = contract.deposit(deposit_amount, assets.quote.id).await?;
        let new_balance = owner.balance(&assets.quote.id).await;
        assert_eq!(new_balance, user_balance - deposit_amount);

        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            DepositEvent {
                amount: deposit_amount,
                asset: assets.quote.id,
                user: owner.identity(),
                account: expected_account.clone(),
                caller: owner.identity(),
            }
        );

        let user_account = contract.account(owner.identity()).await?.value;

        assert_eq!(user_account, expected_account);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn fuzz_quote_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();

        for _ in 0..100 {
            let (contract, owner, _user, _, _, assets) = setup(
                defaults.base_decimals,
                defaults.quote_decimals,
                defaults.price_decimals,
            )
            .await?;

            // Generate a random deposit amount
            let deposit_amount = rand::thread_rng().gen_range(1..1_000_000_000_000);
            let expected_account = create_account(0, 0, 0, 0);

            // Precondition enforces empty account
            assert_eq!(
                contract.account(owner.identity()).await?.value,
                expected_account
            );
            let expected_account = create_account(0, deposit_amount, 0, 0);

            let user_balance = owner.balance(&assets.quote.id).await;
            let response = contract.deposit(deposit_amount, assets.quote.id).await?;
            let new_balance = owner.balance(&assets.quote.id).await;
            assert_eq!(new_balance, user_balance - deposit_amount);

            let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
            let event = log.first().unwrap();
            assert_eq!(
                *event,
                DepositEvent {
                    amount: deposit_amount,
                    asset: assets.quote.id,
                    user: owner.identity(),
                    account: expected_account.clone(),
                    caller: owner.identity(),
                }
            );

            let user_account = contract.account(owner.identity()).await?.value;

            assert_eq!(user_account, expected_account);
        }

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_invalid_asset() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let deposit_amount = 100;

        // Revert
        contract
            .deposit(deposit_amount, assets.random.id)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Paused")]
    async fn when_paused() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        contract.pause().await.unwrap();

        let deposit_amount = 100;

        // Revert
        contract
            .deposit(deposit_amount, assets.random.id)
            .await
            .unwrap();
    }
}
