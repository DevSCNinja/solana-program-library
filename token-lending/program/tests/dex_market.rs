#![cfg(feature = "test-bpf")]

mod helpers;

use helpers::*;
use solana_program_test::*;
use solana_sdk::{
    pubkey::Pubkey,
};
use spl_token_lending::{
    processor::process_instruction,
};
use std::str::FromStr;
use spl_token_lending::dex_market::DexMarket;
use solana_program::account_info::IntoAccountInfo;

#[tokio::test]
async fn test_success() {
    let mut test = ProgramTest::new(
        "spl_token_lending",
        spl_token_lending::id(),
        processor!(process_instruction),
    );

    let sol_usdt_pubkey = Pubkey::from_str("8RJA4WhY2Ei48c4xANSgPoqw7DU7mRgvg6eqJS3tvLEN").unwrap();
    let srm_usdt_pubkey = Pubkey::from_str("CRLpSnSf7JkoJi9tUnz55R2FoTCrDDkWxQMU6uSVBQgc").unwrap();

    test.add_account_with_file_data(
        sol_usdt_pubkey,
        u32::MAX as u64,
        Pubkey::new(&[0; 32]),
        &format!("{}_dex_market.bin", "sol_usdt"),
    );

    test.add_account_with_file_data(
        srm_usdt_pubkey,
        u32::MAX as u64,
        Pubkey::new(&[0; 32]),
        &format!("{}_dex_market.bin", "srm_usdt"),
    );

    let (mut banks_client, _payer, _recent_blockhash) = test.start().await;

    let mut sol_usdt_account = banks_client
        .get_account(sol_usdt_pubkey)
        .await
        .unwrap()
        .unwrap();

    let mut srm_usdt_account = banks_client
        .get_account(srm_usdt_pubkey)
        .await
        .unwrap()
        .unwrap();

    let sol_usdt_account_info = (&sol_usdt_pubkey, false, &mut sol_usdt_account).into_account_info();

    let sol_usdt_market = DexMarket::new(&sol_usdt_account_info);

    let srm_usdt_account_info = (&sol_usdt_pubkey, false, &mut srm_usdt_account).into_account_info();

    let srm_usdt_market = DexMarket::new(&srm_usdt_account_info);

    println!("sol_usdt_bids: {}", &sol_usdt_market.bids);
    println!("sol_usdt_asks: {}", &sol_usdt_market.asks);
    println!("srm_usdt_bids: {}", &srm_usdt_market.bids);
    println!("srm_usdt_asks: {}", &srm_usdt_market.asks);
}
