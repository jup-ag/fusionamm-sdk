use solana_program::pubkey::Pubkey;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{setup_ata_with_amount, setup_fusion_pool, setup_mint_with_decimals, setup_position, RpcContext};
    use crate::{decrease_liquidity_instructions, increase_liquidity_instructions, DecreaseLiquidityParam, IncreaseLiquidityParam};
    use fusionamm_client::{
        get_position_address, get_tick_array_address, ClosePosition, ConvertTickArray, FixedTickArray, SparseTickArray, Tick,
        FIXED_TICK_ARRAY_DISCRIMINATOR, SPARSE_TICK_ARRAY_DISCRIMINATOR,
    };
    use fusionamm_core::get_tick_array_start_tick_index;
    use serial_test::serial;
    use solana_program_test::tokio;
    use solana_signer::Signer;
    use spl_associated_token_account::get_associated_token_address_with_program_id;

    struct TestContext {
        ctx: RpcContext,
        //mint_a: Pubkey,
        //mint_b: Pubkey,
        concentrated_pool: Pubkey,
    }

    impl TestContext {
        async fn new() -> Result<Self, Box<dyn Error>> {
            let ctx = RpcContext::new().await;
            let mint_a = setup_mint_with_decimals(&ctx, 9).await?;
            let mint_b = setup_mint_with_decimals(&ctx, 9).await?;

            setup_ata_with_amount(&ctx, mint_a, 500_000_000_000).await?;
            setup_ata_with_amount(&ctx, mint_b, 500_000_000_000).await?;

            // Setup all pools
            let concentrated_pool = setup_fusion_pool(&ctx, mint_a, mint_b, 64, 300).await?;

            Ok(Self {
                ctx,
                //mint_a,
                //mint_b,
                concentrated_pool,
            })
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_convert_tick_array() {
        let test_ctx = TestContext::new().await.unwrap();
        let ctx = test_ctx.ctx;
        let pool_address = test_ctx.concentrated_pool;

        let start_tick_index = get_tick_array_start_tick_index(0, 64);

        let lower_tick = 0;
        let upper_tick = 128;
        let position_mint = setup_position(&ctx, test_ctx.concentrated_pool, Some((lower_tick, upper_tick)), None)
            .await
            .unwrap();

        let tick_array_address = get_tick_array_address(&pool_address, start_tick_index).unwrap();
        let authority = ctx.signer.pubkey();

        let param = IncreaseLiquidityParam::Liquidity(10_000);
        let inc_ix = increase_liquidity_instructions(
            &ctx.rpc,
            position_mint,
            param,
            Some(100), // slippage
            Some(authority),
        )
        .await
        .unwrap();
        ctx.send_transaction(inc_ix.instructions).await.unwrap();

        let fixed_tick_array_account = ctx.rpc.get_account(&tick_array_address.0).await.unwrap();
        let fixed_tick_array = FixedTickArray::from_bytes(&fixed_tick_array_account.data).unwrap();
        assert_eq!(fixed_tick_array.discriminator, FIXED_TICK_ARRAY_DISCRIMINATOR);
        assert_eq!(fixed_tick_array.start_tick_index, start_tick_index);
        assert_eq!(fixed_tick_array.fusion_pool, pool_address);

        let balance_before = ctx.rpc.get_balance(&authority).await.unwrap();

        ctx.send_transaction(vec![ConvertTickArray {
            tick_array: tick_array_address.0,
            funder: authority,
        }
        .instruction()])
            .await
            .unwrap();

        let balance_after = ctx.rpc.get_balance(&authority).await.unwrap();
        assert_eq!(balance_after - balance_before, 67033720);

        let tick_array_account = ctx.rpc.get_account(&tick_array_address.0).await.unwrap();
        let sparse_tick_array = SparseTickArray::from_bytes(&tick_array_account.data).unwrap();
        assert_eq!(sparse_tick_array.discriminator, SPARSE_TICK_ARRAY_DISCRIMINATOR);
        assert_eq!(sparse_tick_array.start_tick_index, start_tick_index);
        assert_eq!(sparse_tick_array.fusion_pool, pool_address);

        let tick0: Tick = sparse_tick_array.ticks[0].clone().into();
        let tick1: Tick = sparse_tick_array.ticks[1].clone().into();
        let tick2: Tick = sparse_tick_array.ticks[2].clone().into();
        assert_eq!(tick0.initialized, true);
        assert_eq!(tick1.initialized, false);
        assert_eq!(tick2.initialized, true);

        let position_token_account_address = get_associated_token_address_with_program_id(&authority, &position_mint, &spl_token_2022::ID);

        let param = DecreaseLiquidityParam::Liquidity(10_000);
        let inc_ix = decrease_liquidity_instructions(
            &ctx.rpc,
            position_mint,
            param,
            Some(100), // slippage
            Some(authority),
        )
        .await
        .unwrap();
        ctx.send_transaction(inc_ix.instructions).await.unwrap();

        ctx.send_transaction(vec![ClosePosition {
            position_authority: authority,
            receiver: authority,
            position: get_position_address(&position_mint).unwrap().0,
            position_mint,
            position_token_account: position_token_account_address,
            token2022_program: spl_token_2022::ID,
        }
        .instruction()])
            .await
            .unwrap();
    }
}
