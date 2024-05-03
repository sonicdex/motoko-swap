use candid::{Nat, Principal};
use ic_cdk::{api::call::RejectionCode, id};
use ic_ledger_types::Subaccount;
use icrc_ledger_types::{
    icrc1::{
        account::Account,
        transfer::{TransferArg, TransferError},
    },
    icrc2::{
        allowance::{Allowance, AllowanceArgs},
        transfer_from::{TransferFromArgs, TransferFromError},
    },
};

pub struct Ledger(Principal, Nat);

impl Ledger {
    pub fn new(canister: Principal) -> Self {
        Self(canister, Nat::from(0u32))
    }

    pub fn set_fee(&mut self, fee: Nat) {
        self.1 = fee;
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
            Err((code, msg)) => Err(format!("get_allowance, Error: {:?} {:?}", code, msg)),
        }
    }

    pub async fn transfer_from_to_swap_subaccount(
        &self,
        from: Principal,
        amount: Nat,
    ) -> Result<Nat, String> {
        let subaccount = Subaccount::from(from).0;

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
            amount: amount.clone() - self.1.clone(),
            fee: Some(self.1.clone()),
            memo: None,
            created_at_time: None,
        };

        self.transfer_from(transfer_from_args).await
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
            Err((code, msg)) => Err(format!("get_balance, Error: {:?} {:?}", code, msg)),
        }
    }

    pub async fn from_canister_to_caller_transaction(
        &self,
        to: Principal,
        amount: Nat,
    ) -> Result<Nat, String> {
        let transfer_arg = TransferArg {
            memo: None,
            amount: amount - self.1.clone(),
            fee: Some(self.1.clone()),
            from_subaccount: None,
            to: Account {
                owner: to,
                subaccount: None,
            },
            created_at_time: None,
        };

        self.transfer(transfer_arg).await
    }

    pub async fn from_canister_to_subaccount_to_caller_transaction(
        &self,
        to: Principal,
        amount: Nat,
    ) -> Result<Nat, String> {
        let transfer_arg = TransferArg {
            memo: None,
            amount,
            fee: Some(self.1.clone()),
            from_subaccount: Some(Subaccount::from(to).0),
            to: Account {
                owner: to,
                subaccount: None,
            },
            created_at_time: None,
        };

        self.transfer(transfer_arg).await
    }

    pub async fn internal_transaction(
        &self,
        principal: Principal,
        amount: Nat,
    ) -> Result<Nat, String> {
        let transfer_arg = TransferArg {
            from_subaccount: Some(Subaccount::from(principal).0),
            to: Account {
                owner: id(),
                subaccount: None,
            },
            amount: amount - self.1.clone(),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        self.transfer(transfer_arg).await
    }

    pub async fn get_fee(&self) -> Result<Nat, String> {
        let result: Result<(Nat,), (RejectionCode, String)> =
            ic_cdk::call(self.0, "icrc1_fee", ()).await;

        match result {
            Ok((fee,)) => Ok(fee),
            Err((code, msg)) => Err(format!("get_fee, Error: {:?} {:?}", code, msg)),
        }
    }

    async fn transfer(&self, transfer_args: TransferArg) -> Result<Nat, String> {
        let result: Result<(Result<Nat, TransferError>,), (RejectionCode, String)> =
            ic_cdk::call(self.0, "icrc1_transfer", (transfer_args,)).await;

        match result {
            Ok((Ok(height),)) => Ok(height),
            Ok((Err(err),)) => Err(format!("transfer, Error: {:?}", err)),
            Err((code, msg)) => Err(format!("transfer, Error: {:?} {:?}", code, msg)),
        }
    }

    async fn transfer_from(&self, transfer_from_args: TransferFromArgs) -> Result<Nat, String> {
        let result: Result<(Result<Nat, TransferFromError>,), (RejectionCode, String)> =
            ic_cdk::call(self.0, "icrc2_transfer_from", (transfer_from_args,)).await;

        match result {
            Ok((Ok(height),)) => Ok(height),
            Ok((Err(err),)) => Err(format!("transfer_from, Error: {:?}", err)),
            Err((code, msg)) => Err(format!("transfer_from, Error: {:?} {:?}", code, msg)),
        }
    }
}
