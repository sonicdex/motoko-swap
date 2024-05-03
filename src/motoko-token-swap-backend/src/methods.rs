use candid::{export_service, Principal};
use ic_cdk::{caller, query, update};

use crate::ledger::Ledger;

#[update]
pub async fn swap(from: Principal, to: Principal) -> Result<String, String> {
    // initalize the FROM ledger
    let mut from_ledger = Ledger::new(from);
    from_ledger.set_fee(from_ledger.get_fee().await?);

    // initalize the TO ledger
    let mut to_ledger = Ledger::new(to);
    to_ledger.set_fee(to_ledger.get_fee().await?);

    // Check if the caller has set the correct allowance for the SWAP canister on the FROM ledger
    let allowance = from_ledger.get_allowance(caller()).await?;

    // Check if the TO ledger has enough balance to transfer to the caller
    if to_ledger.get_balance(None).await? < allowance.allowance {
        return Err("Transaction cancelled: TO ledger does not have enough balance".to_string());
    }

    // Transfer the FROM tokens to this canister under the caller's subaccount
    let _blockheight = from_ledger
        .transfer_from_to_swap_subaccount(caller(), allowance.allowance.clone())
        .await?;

    // Check the balance of the caller's subaccount on this canister
    let from_balance = from_ledger.get_balance(Some(caller())).await?;

    // transfer the FROM token amount to the caller
    to_ledger
        .from_canister_to_caller_transaction(caller(), from_balance.clone())
        .await?;

    // Finally transfer old tokens from caller's subaccount to the canister default subaccount
    // This way we can keep track of the old tokens that are swapped
    from_ledger
        .internal_transaction(caller(), from_balance.clone())
        .await?;

    Ok("Transaction successful".to_string())
}

#[update]
async fn _dev_transfer_to_caller(from: Principal) -> Result<(), String> {
    let mut from_ledger = Ledger::new(from);
    from_ledger.set_fee(from_ledger.get_fee().await?);

    let from_balance = from_ledger.get_balance(None).await?;
    from_ledger
        .from_canister_to_caller_transaction(caller(), from_balance.clone())
        .await?;

    Ok(())
}

#[update]
async fn _dev_transfer_to_subaccount_to_caller(from: Principal) -> Result<(), String> {
    let mut from_ledger = Ledger::new(from);
    from_ledger.set_fee(from_ledger.get_fee().await?);
    let from_balance = from_ledger.get_balance(Some(caller())).await?;
    from_ledger
        .from_canister_to_subaccount_to_caller_transaction(caller(), from_balance.clone())
        .await?;

    Ok(())
}

#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    export_service!();
    __export_service()
}

#[test]
pub fn candid() {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // let dir = dir.parent().unwrap();
    write(
        dir.join(format!("motoko-token-swap-backend.did")),
        __export_did_tmp_(),
    )
    .expect("Write failed.");
}
