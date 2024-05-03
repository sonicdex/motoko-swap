import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { _SERVICE as _ICRC1_LEDGER_SERVICE, idlFactory as icrc1LedgerIdlFactory } from "./declarations/icrc1_ledger";
import {
  idlFactory,
  _SERVICE as _BACKEND_SERVICE,
} from "../../declarations/motoko-token-swap-backend/motoko-token-swap-backend.did.js";
import { host, swapCanister } from "./misc/constants";

export function swapActor(agent: Agent | undefined) {
  let actor = Actor.createActor<_BACKEND_SERVICE>(idlFactory, {
    agent,
    canisterId: swapCanister,
  });
  return actor;
}

export function ledgerActor(canisterId: string, agent?: Agent | undefined) {
  return Actor.createActor<_ICRC1_LEDGER_SERVICE>(icrc1LedgerIdlFactory, {
    agent: agent ?? new HttpAgent({ host }),
    canisterId,
  });
}
