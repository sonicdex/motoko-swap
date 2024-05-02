import { Actor, HttpAgent } from "@dfinity/agent";
import { _SERVICE as _ICRC1_LEDGER_SERVICE, idlFactory as icrc1LedgerIdlFactory } from "./declarations/icrc1_ledger.js";
import { Principal } from "@dfinity/principal";
import {
  idlFactory,
  _SERVICE as _BACKEND_SERVICE,
} from "../../declarations/motoko-token-swap-backend/motoko-token-swap-backend.did.js";

export const host = "https://ic0.app";

export const swap_canister = "skfly-zaaaa-aaaap-ahc5q-cai";

export const fromLedger = "zfcdd-tqaaa-aaaaq-aaaga-cai"; // dragginz
export const toLedger = "uf2wh-taaaa-aaaaq-aabna-cai"; // catalyze

export function anoymousActor(canisterId: string) {
  return Actor.createActor<_ICRC1_LEDGER_SERVICE>(icrc1LedgerIdlFactory, {
    agent: new HttpAgent({ host }),
    canisterId: canisterId,
  });
}

export async function getBalance(from: string, canisterId: string) {
  const actor = anoymousActor(canisterId);
  const balance = await actor.icrc1_balance_of({
    owner: Principal.fromText(from),
    subaccount: [],
  });
  return Number(balance);
}

export function backendActor() {
  let actor = Actor.createActor<_BACKEND_SERVICE>(idlFactory, {
    agent: (window as any).ic.plug.agent,
    canisterId: "skfly-zaaaa-aaaap-ahc5q-cai",
  });
  return actor;
}

export async function ledgerActor(canisterId: string) {
  const agent = (window as any).ic.plug.agent;

  const actor = Actor.createActor<_ICRC1_LEDGER_SERVICE>(icrc1LedgerIdlFactory, {
    agent: agent,
    canisterId,
  });

  return actor;
}
