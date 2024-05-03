import { useState } from "react";
import { TokenDisplay } from "./components";
import { debugMode, fromLedger, host, swapCanister, toLedger } from "./misc/constants";
import { Agent } from "@dfinity/agent";
import { ledgerActor, swapActor } from "./actors";
import { Principal } from "@dfinity/principal";
import { principalToSubAccount } from "@dfinity/utils";

const _window = window as any;

function App() {
  const [isLoggingIn, setIsLoggingIn] = useState(false);
  const [isSettingAllowance, setIsSettingAllowance] = useState(false);
  const [isSwapping, setIsSwapping] = useState(false);
  const [userPrincipal, setUserPrincipal] = useState<Principal | undefined>(undefined);
  const [agent, setAgent] = useState<Agent | undefined>(undefined);
  const [update, setUpdate] = useState(false);

  async function handleLogin() {
    try {
      setIsLoggingIn(true);
      await _window.ic.plug.requestConnect({
        whitelist: [swapCanister, fromLedger, toLedger],
        host,
      });
      const principalString: string = _window.ic.plug.principalId;
      setUserPrincipal(Principal.fromText(principalString));
      setAgent(_window.ic.plug.agent);
    } catch (e) {
      console.log(e);
    } finally {
      setIsLoggingIn(false);
    }
  }

  async function setAllowance() {
    if (!userPrincipal) {
      return;
    }
    try {
      setIsSettingAllowance(true);

      // These calls are public and can be done anonymously which improves speed
      const anomymousLedger = ledgerActor(fromLedger);
      const callerBalance = await anomymousLedger.icrc1_balance_of({ owner: userPrincipal, subaccount: [] });
      const fee = await anomymousLedger.icrc1_fee();

      // This call requires authentication from the caller
      await ledgerActor(fromLedger, agent).icrc2_approve({
        spender: {
          owner: Principal.fromText(swapCanister),
          subaccount: [principalToSubAccount(userPrincipal)],
        },
        amount: callerBalance - fee,
        from_subaccount: [],
        created_at_time: [],
        expected_allowance: [],
        expires_at: [],
        fee: [fee],
        memo: [],
      });

      setUpdate((prevstate) => !prevstate);
    } catch (error) {
      console.log(error);
    } finally {
      setIsSettingAllowance(false);
    }
  }

  async function handleSwap() {
    try {
      setIsSwapping(true);
      const result = await swapActor(agent).swap(Principal.fromText(fromLedger), Principal.fromText(toLedger));
      setUpdate((prevstate) => !prevstate);
      console.log(result);
    } catch (error) {
      console.log(error);
    } finally {
      setIsSwapping(false);
    }
  }

  async function claimBack() {
    try {
      const result = await swapActor(agent)._dev_transfer_to_caller(Principal.fromText(fromLedger));
      console.log(result);
      setUpdate((prevstate) => !prevstate);
    } catch (error) {
      console.log(error);
    }
  }

  async function claimBackSub() {
    try {
      const result = await swapActor(agent)._dev_transfer_to_subaccount_to_caller(Principal.fromText(fromLedger));
      console.log(result);
      setUpdate((prevstate) => !prevstate);
    } catch (error) {
      console.log(error);
    }
  }

  async function automatedSwap() {
    try {
      await handleLogin();
      await setAllowance();
      await handleSwap();
    } catch (error) {
      console.log(error);
    }
  }

  function renderActions() {
    if (debugMode) {
      return (
        <div className="actions">
          <button disabled={!!userPrincipal} onClick={handleLogin}>
            {isLoggingIn ? "..." : "Login"}
          </button>
          <button disabled={!userPrincipal} onClick={setAllowance}>
            {isSettingAllowance ? "..." : "Set allowance"}
          </button>
          <button disabled={!userPrincipal} onClick={handleSwap}>
            {isSwapping ? "..." : "Swap"}
          </button>
          <br />

          <button disabled={!userPrincipal} onClick={claimBack}>
            {isLoggingIn ? "..." : "transfer FROM swap balance back to caller"}
          </button>
          <button disabled={!userPrincipal} onClick={claimBackSub}>
            {isLoggingIn ? "..." : "transfer FROM swap subaccount balance back to caller"}
          </button>
        </div>
      );
    }
    return (
      <div className="actions">
        <button onClick={automatedSwap}>{isLoggingIn ? "..." : "Swap"}</button>
      </div>
    );
  }

  return (
    <div>
      <div className="container">
        <TokenDisplay title="From" canisterId={fromLedger} userPrincipal={userPrincipal} update={update} />
        {renderActions()}
        <TokenDisplay title="To" canisterId={toLedger} userPrincipal={userPrincipal} update={update} />
      </div>
    </div>
  );
}

export default App;
