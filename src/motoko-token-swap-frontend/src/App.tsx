import { useEffect, useState } from "react";
import { principalToSubAccount } from "@dfinity/utils";
import { Principal } from "@dfinity/principal";
import { backendActor, toLedger, fromLedger, getBalance, host, ledgerActor, swap_canister } from "./actors";

function App() {
  const [isLoading, setIsLoading] = useState(false);
  const [fromBalanceSwap, setFromBalanceSwap] = useState(0);
  const [toBalanceSwap, setToBalanceSwap] = useState(0);
  const [fromBalanceUser, setfromBalanceUser] = useState(0);
  const [toBalanceUser, setToBalanceUser] = useState(0);

  useEffect(() => {
    getSwapBalances();
  }, []);

  async function getSwapBalances() {
    try {
      setFromBalanceSwap(await getBalance(swap_canister, fromLedger));
      setToBalanceSwap(await getBalance(swap_canister, toLedger));
    } catch (error) {
      console.log(error);
    }
  }

  async function getUserBalances(principal: string) {
    try {
      setfromBalanceUser(await getBalance(principal, fromLedger));
      setToBalanceUser(await getBalance(principal, toLedger));
    } catch (error) {
      console.log(error);
    }
  }

  async function handleSwapFlow() {
    try {
      const principal = await login();
      if (principal) {
        getUserBalances(principal);
        await handleApprove(Principal.fromText(principal));
      }
    } catch (error) {
      console.log(error);
    }
  }

  async function login() {
    try {
      setIsLoading(true);
      await (window as any).ic.plug.requestConnect({
        whitelist: [swap_canister, fromLedger, toLedger],
        host,
      });
      const principal: string = (window as any).ic.plug.principalId;
      return principal;
    } catch (e) {
      console.log(e);
    } finally {
      setIsLoading(false);
    }
  }

  async function handleApprove(principal: Principal) {
    try {
      const ledger = await ledgerActor(fromLedger);
      const balanceOfCaller = await ledger.icrc1_balance_of({ owner: principal, subaccount: [] });
      await ledger.icrc2_approve({
        spender: {
          owner: Principal.fromText(swap_canister),
          subaccount: [principalToSubAccount(principal)],
        },
        amount: balanceOfCaller,
        from_subaccount: [],
        created_at_time: [],
        expected_allowance: [],
        expires_at: [],
        fee: [],
        memo: [],
      });

      const allowance = await ledger.icrc2_allowance({
        account: {
          owner: principal,
          subaccount: [],
        },
        spender: {
          owner: Principal.fromText(swap_canister),
          subaccount: [principalToSubAccount(principal)],
        },
      });

      console.log({ balanceOfCaller, allowance });
      const result = await backendActor().swap(Principal.fromText(fromLedger), Principal.fromText(toLedger));
      await getSwapBalances();
      await getUserBalances(principal.toString());
      console.log(result);
    } catch (error) {
      console.log(error);
    }
  }

  function renderSwapBalances() {
    return (
      <div>
        <h6>Swap</h6>
        <p>From Balance: {fromBalanceSwap}</p>
        <p>To Balance: {toBalanceSwap}</p>
      </div>
    );
  }

  function renderUserBalances() {
    return (
      <div>
        <h6>User</h6>
        <p>From Balance: {fromBalanceUser}</p>
        <p>To Balance: {toBalanceUser}</p>
      </div>
    );
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      {renderSwapBalances()}
      {renderUserBalances()}
      <br />
      <button onClick={handleSwapFlow}>{isLoading ? "..." : "Swap"}</button>
      {/* <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
      </form>
      <section id="greeting">{greeting}</section> */}
    </main>
  );
}

export default App;
