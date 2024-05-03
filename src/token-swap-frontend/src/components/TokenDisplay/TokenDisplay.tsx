import React, { useEffect, useState } from "react";
import { ledgerActor } from "../../actors";
import { debugMode, swapCanister } from "../../misc/constants";
import { Principal } from "@dfinity/principal";
import { principalToSubAccount } from "@dfinity/utils";
import { getPrettyDecimals } from "../../misc/tokenHelper";

interface IProps {
  title: string;
  canisterId: string;
  userPrincipal?: Principal;
  // ugly way to force update the component
  update: boolean;
}

interface IBalances {
  swap: number;
  user: number;
  subaccount: number;
  allowance: number;
}

const TokenDisplay = ({ title, canisterId, userPrincipal, update }: IProps) => {
  const [name, setName] = useState("");
  const [symbol, setSymbol] = useState("");
  const [decimals, setDecimals] = useState(0);
  const [fee, setFee] = useState(0);
  const [balances, setBalances] = useState<IBalances>({
    allowance: 0,
    subaccount: 0,
    swap: 0,
    user: 0,
  });

  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    getDetails();
  }, [canisterId, update]);

  useEffect(() => {
    if (userPrincipal) {
      getUserBalance(userPrincipal);
    }
  }, [canisterId, userPrincipal, update]);

  async function getDetails() {
    try {
      setIsLoading(true);
      const actor = ledgerActor(canisterId);
      setName(await actor.icrc1_name());
      setSymbol(await actor.icrc1_symbol());
      setDecimals(await actor.icrc1_decimals());
      setFee(Number(await actor.icrc1_fee()));
      const swapBalance = await actor.icrc1_balance_of({ owner: Principal.fromText(swapCanister), subaccount: [] });
      setBalances((prev) => ({ ...prev, swap: Number(swapBalance) }));
    } catch (error) {
      console.log(error);
    } finally {
      setIsLoading(false);
    }
  }

  async function getUserBalance(principal: Principal) {
    try {
      const balance = await ledgerActor(canisterId).icrc1_balance_of({
        owner: principal,
        subaccount: [],
      });

      const subaccountBalance = await ledgerActor(canisterId).icrc1_balance_of({
        owner: Principal.fromText(swapCanister),
        subaccount: [principalToSubAccount(principal)],
      });

      const { allowance } = await ledgerActor(canisterId).icrc2_allowance({
        account: {
          owner: principal,
          subaccount: [],
        },
        spender: {
          owner: Principal.fromText(swapCanister),
          subaccount: [principalToSubAccount(principal)],
        },
      });
      setBalances((prev) => ({
        ...prev,
        user: Number(balance),
        subaccount: Number(subaccountBalance),
        allowance: Number(allowance),
      }));
    } catch (error) {
      console.log(error);
    }
  }

  if (isLoading) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <div className="token-display" style={debugMode ? {} : { display: "flex", justifyContent: "center" }}>
        <h4>
          {title} {symbol}
        </h4>
        {debugMode && (
          <>
            <p>
              Name:
              <br />
              <strong>{name}</strong>
            </p>
            <p>
              Decimals:
              <br />
              <strong>{decimals}</strong>
            </p>
            <p>
              Fee (e8s):
              <br />
              <strong>{fee}</strong>
            </p>
            <p>
              Canister ID:
              <br />
              <strong>{canisterId}</strong>
            </p>
          </>
        )}
      </div>
      {debugMode ? (
        <div className="token-display">
          <p>
            Swap balance:
            <br />
            <strong>{getPrettyDecimals(balances.swap, decimals)}</strong>
          </p>
          <p>
            User balance:
            <br />
            <strong>{getPrettyDecimals(balances.user, decimals)}</strong>
          </p>
          <p>
            Swap subaccount balance:
            <br />
            <strong>{getPrettyDecimals(balances.subaccount, decimals)}</strong>
          </p>
          <p>
            Allowance:
            <br />
            <strong>{getPrettyDecimals(balances.allowance, decimals)}</strong>
          </p>
        </div>
      ) : (
        <div className="token-display" style={debugMode ? {} : { display: "flex", textAlign: "center" }}>
          <p>
            User balance:
            <br />
            <strong>{getPrettyDecimals(balances.user, decimals)}</strong>
          </p>
        </div>
      )}
    </div>
  );
};

export default TokenDisplay;
