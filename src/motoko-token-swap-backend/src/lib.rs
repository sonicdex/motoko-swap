use candid::{export_service, Nat, Principal};
use ic_cdk::{api::call::RejectionCode, caller, id, query, update};
use ic_ledger_types::Subaccount;
use icrc_ledger_types::{
    icrc1::{self, account::Account},
    icrc2::{self, allowance::Allowance, transfer_from::TransferFromError},
};

// const OLD_LEDGER_ID: &str = "uf2wh-taaaa-aaaaq-aabna-cai";
const NEW_LEDGER_ID: &str = "zfcdd-tqaaa-aaaaq-aaaga-cai";

#[query]
fn get_subaccount(principal: Principal) -> [u8; 32] {
    Subaccount::from(principal).0
}

#[update]
pub async fn get_allowance(principal: Principal) -> Result<Allowance, String> {
    let allowance_args = icrc2::allowance::AllowanceArgs {
        account: Account {
            owner: principal,
            subaccount: None,
        },
        spender: Account {
            owner: id(),
            subaccount: Some(get_subaccount(principal)),
        },
    };

    let transfer: Result<(Allowance,), (RejectionCode, String)> = ic_cdk::call(
        Principal::from_text(NEW_LEDGER_ID).unwrap(),
        "icrc2_allowance",
        (allowance_args,),
    )
    .await;

    match transfer {
        Ok(ok) => Ok(ok.0),
        Err((code, msg)) => Err(format!("Error: {:?} {:?}", code, msg)),
    }
}

#[update]
pub async fn handle_transaction(amount: Nat) -> Result<Nat, String> {
    let transfer_from_args = icrc2::transfer_from::TransferFromArgs {
        spender_subaccount: Some(get_subaccount(caller())),
        from: {
            Account {
                owner: caller(),
                subaccount: None,
            }
        },
        to: Account {
            owner: id(),
            subaccount: Some(get_subaccount(caller())),
        },
        amount,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    let transfer: Result<(Result<Nat, TransferFromError>,), (RejectionCode, String)> =
        ic_cdk::call(
            Principal::from_text(NEW_LEDGER_ID).unwrap(),
            "icrc2_transfer_from",
            (transfer_from_args,),
        )
        .await;

    match transfer {
        Ok((Ok(height),)) => Ok(height),
        Ok((Err(err),)) => Err(format!("Error: {:?}", err)),
        Err((code, msg)) => Err(format!("Error: {:?} {:?}", code, msg)),
    }
}

#[update]
pub async fn check_subaccount_balance(principal: Principal) -> Result<Nat, String> {
    let account = icrc1::account::Account {
        owner: id(),
        subaccount: Some(get_subaccount(principal)),
    };

    let balance: Result<(Nat,), (RejectionCode, String)> = ic_cdk::call(
        Principal::from_text(NEW_LEDGER_ID).unwrap(),
        "icrc1_balance_of",
        (account,),
    )
    .await;

    match balance {
        Ok((balance,)) => Ok(balance),
        Err((code, msg)) => Err(format!("Error: {:?} {:?}", code, msg)),
    }
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
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
