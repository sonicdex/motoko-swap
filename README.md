# Internet Computer Token Swap

This repository contains a token swapping system for the Internet Computer (IC), specifically designed to work with tokens adhering to the ICRC2 standard. It allows users to swap an "old" token for a "new" token using a dedicated swap canister.

## Prerequisites

Before starting the swap process, update the `constants.ts` file with the correct canister addresses:

- `fromLedger`: The Principal ID of the token you want to swap from.
- `toLedger`: The Principal ID of the token you want to swap to.
- `swapCanister`: The Principal ID of the canister that handles the swap, included in this repository.

## Configuration

```typescript
// Token you want to swap from (give allowance to the swap canister)
export const fromLedger = "aaaaa-aa";

// Token you want to swap to (transfer these tokens to the swap canister before initiating the swap)
export const toLedger = "bbbbb-bb";

// Swap canister (backend canister that facilitates the swap, included in this repository)
export const swapCanister = "ccccc-cc";

export const debugMode = true;
export const host = "https://ic0.app";
```

### Swap Process Flow

Here's how the token swap process works step-by-step:

1. **Approval**: The user makes an approve call for the swap canister, setting the allowance for the amount of tokens to be swapped.
2. **Validation**: The swap canister checks if it has enough of the new token available for the swap.
3. **Transfer**:
   - If enough tokens are available, the canister:
   - Transfers the approved amount of old tokens to itself under the user's subaccount.
   - Transfers the equivalent amount of new tokens to the user.
   - Moves the old tokens from the user's subaccount to the default subaccount (used for tracking total amounts swapped).
   - If insufficient tokens are available, the transaction is cancelled, and the user is notified to retry.

### Transaction Fees

The token swap process involves three distinct transactions, each incurring a network fee:

1. **Approval Transaction**: This is the first transaction where the user approves the swap canister to handle the specified amount of old tokens. This transaction is necessary to authorize the swap canister to transfer tokens on behalf of the user.

2. **Swap Execution Transaction**: After approval, the swap canister executes the swap if it has sufficient new tokens. This includes transferring the old tokens to the swap canister's subaccount and the new tokens to the user's account.

3. **Reconciliation Transaction**: The final transaction involves the swap canister moving the swapped old tokens from the user's subaccount to a default subaccount for tracking purposes.

Each of these transactions requires a fee, which is dependent on the network's current transaction cost. Users must ensure that they have enough balance to cover these fees in addition to the tokens being swapped. It is advisable to check the current fee rates on the Internet Computer network to estimate the total cost of the swap process.

### Notes

- Ensure that both tokens involved in the swap support the ICRC2 standard.
- The repository includes debug mode for additional logging and tracing during development.
- This is 1:1 swap (excluding fees)

### Getting Started

To use this swap, clone the repository, configure your tokens as described, and deploy it to the IC.
Detailed deployment instructions will depend on your specific environment and setup requirements.
