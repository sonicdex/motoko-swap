use candid::{Nat, Principal};
use ic_cdk::{api::call::RejectionCode, id};
use ic_ledger_types::Subaccount;
use icrc_ledger_types::{
    icrc1::account::Account,
    icrc2::{
        allowance::{Allowance, AllowanceArgs},
        transfer_from::{TransferFromArgs, TransferFromError},
    },
};

pub struct Ledger(Principal);

impl Ledger {
    pub fn new(canister: Principal) -> Self {
        Self(canister)
    }

    pub async fn get_allowance(&self, principal: Principal) -> Result<Allowance, String> {
        let allowance_args = AllowanceArgs {
            account: Account {
                owner: principal,
                subaccount: None,
            },
            spender: Account {
                owner: id(),
                subaccount: Some(Subaccount::from(principal).0),
            },
        };

        let result: Result<(Allowance,), (RejectionCode, String)> =
            ic_cdk::call(self.0, "icrc2_allowance", (allowance_args,)).await;

        match result {
            Ok(ok) => Ok(ok.0),
            Err((code, msg)) => Err(format!("Error: {:?} {:?}", code, msg)),
        }
    }

    pub async fn from_ledger_to_canister_transaction(
        &self,
        from: Principal,
        amount: Nat,
    ) -> Result<Nat, String> {
        let subaccount = Subaccount::from(from).0;
        let fee: Nat = self.get_fee().await?;

        let transfer_amount: Nat = amount - (fee.clone() + fee.clone()); // amount - approve - transfer
        let transfer_from_args = TransferFromArgs {
            spender_subaccount: Some(subaccount),
            from: {
                Account {
                    owner: from,
                    subaccount: None,
                }
            },
            to: Account {
                owner: id(),
                subaccount: Some(subaccount),
            },
            amount: transfer_amount.clone(),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let result: Result<(Result<Nat, TransferFromError>,), (RejectionCode, String)> =
            ic_cdk::call(self.0, "icrc2_transfer_from", (transfer_from_args,)).await;

        match result {
            Ok((Ok(_),)) => Ok(transfer_amount),
            Ok((Err(err),)) => Err(format!("Error: {:?}", err)),
            Err((code, msg)) => Err(format!("Error: {:?} {:?}", code, msg)),
        }
    }

    pub async fn get_balance(&self, principal: Option<Principal>) -> Result<Nat, String> {
        let subaccount = match principal {
            Some(principal) => Some(Subaccount::from(principal).0),
            None => None,
        };

        let account = Account {
            owner: id(),
            subaccount,
        };

        let result: Result<(Nat,), (RejectionCode, String)> =
            ic_cdk::call(self.0, "icrc1_balance_of", (account,)).await;

        match result {
            Ok((balance,)) => Ok(balance),
            Err((code, msg)) => Err(format!("Error: {:?} {:?}", code, msg)),
        }
    }

    pub async fn from_canister_to_caller_transaction(
        &self,
        to: Principal,
        amount: Nat,
    ) -> Result<Nat, String> {
        let fee: Nat = self.get_fee().await?;
        let transfer_from_args = TransferFromArgs {
            spender_subaccount: None,
            from: {
                Account {
                    owner: id(),
                    subaccount: None,
                }
            },
            to: Account {
                owner: to,
                subaccount: None,
            },
            amount: amount - fee.clone(),
            fee: Some(fee),
            memo: None,
            created_at_time: None,
        };

        let result: Result<(Result<Nat, TransferFromError>,), (RejectionCode, String)> =
            ic_cdk::call(self.0, "icrc2_transfer_from", (transfer_from_args,)).await;

        match result {
            Ok((Ok(height),)) => Ok(height),
            Ok((Err(err),)) => Err(format!("Error: {:?}", err)),
            Err((code, msg)) => Err(format!("Error: {:?} {:?}", code, msg)),
        }
    }

    async fn get_fee(&self) -> Result<Nat, String> {
        let result: Result<(Nat,), (RejectionCode, String)> =
            ic_cdk::call(self.0, "icrc1_fee", ()).await;

        match result {
            Ok((fee,)) => Ok(fee),
            Err((code, msg)) => Err(format!("Error: {:?} {:?}", code, msg)),
        }
    }
}
