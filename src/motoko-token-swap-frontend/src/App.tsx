import { useState } from "react";
import { Actor } from "@dfinity/agent";
import { _SERVICE as _ICRC1_LEDGER_SERVICE, idlFactory as icrc1LedgerIdlFactory } from "./declarations/icrc1_ledger.js";
import {
  idlFactory,
  _SERVICE as _BACKEND_SERVICE,
} from "../../declarations/motoko-token-swap-backend/motoko-token-swap-backend.did.js";
import { principalToSubAccount } from "@dfinity/utils";
import { Principal } from "@dfinity/principal";

interface TransactionObject {
  to: string;
  strAmount: string;
  token: string;
  opts?: {
    fee?: number;
    memo?: string;
    from_subaccount?: number[] | Uint8Array;
    created_at_time?: {
      timestamp_nanos: number;
    };
  };
}

const backend_canister = "skfly-zaaaa-aaaap-ahc5q-cai";
const new_ledger_canister = "zfcdd-tqaaa-aaaaq-aaaga-cai";

function App() {
  // const [greeting, setGreeting] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  async function login() {
    try {
      setIsLoading(true);
      const publicKey = await (window as any).ic.plug.requestConnect({
        whitelist: [backend_canister, new_ledger_canister],
        host: "https://icp0.io",
      });
      console.log(`The connected user's public key is:`, publicKey);
      console.log((window as any).ic.plug.agent);
    } catch (e) {
      console.log(e);
    } finally {
      setIsLoading(false);
    }
  }

  function backendActor() {
    let actor = Actor.createActor<_BACKEND_SERVICE>(idlFactory, {
      agent: (window as any).ic.plug.agent,
      canisterId: "skfly-zaaaa-aaaap-ahc5q-cai",
    });
    return actor;
  }

  async function ledgerActor() {
    const agent = (window as any).ic.plug.agent;

    const actor = Actor.createActor<_ICRC1_LEDGER_SERVICE>(icrc1LedgerIdlFactory, {
      agent: agent,
      canisterId: new_ledger_canister,
    });

    return actor;
  }

  async function handleApprove() {
    try {
      const principal = Principal.fromText((window as any).ic.plug.principalId);
      const ledger = await ledgerActor();

      const balanceOfCaller = await ledger.icrc1_balance_of({ owner: principal, subaccount: [] });
      const approveResponse = await ledger.icrc2_approve({
        spender: {
          owner: Principal.fromText(backend_canister),
          subaccount: [principalToSubAccount(principal)],
        },
        amount: 100000000n,
        from_subaccount: [],
        created_at_time: [],
        expected_allowance: [],
        expires_at: [],
        fee: [],
        memo: [],
      });

      console.log({ balanceOfCaller, transactionResponse: approveResponse });
    } catch (error) {
      console.log(error);
    }
  }

  async function handleTransfer() {
    try {
      const result = await backendActor().handle_transaction(10000000n);
      console.log(result);
    } catch (error) {
      console.log(error);
    }
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <button onClick={login}>{isLoading ? "..." : "login"}</button>
      <button onClick={handleApprove}>{isLoading ? "..." : "allowance"}</button>
      <button onClick={handleTransfer}>{isLoading ? "..." : "transfer"}</button>
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
