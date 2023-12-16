# Swap 

This is a TypeScript implementation of the private swap. This example is built on the Solana blockchain and is bootstrapped using the [Light CLI](https://www.npmjs.com/package/@lightprotocol/cli) (which also leverages the Anchor framework).  

It uses [Light Protocol v3](https://github.com/Lightprotocol/light-protocol) for private state and state transitions. This allows the swaps to be executed fully on-chain.

## Support

If you face any issues running this example or have questions related to PSPs, feel free to join the community Discord [here](https://discord.gg/Q3wvnSrKgm).

## Prerequisites

Before running the code, ensure that you have the following installed on your machine:


- node.js, npm
- circom
- rust
- rust fmt (```rustup component add rustfmt```)
- cargo-expand (```cargo install cargo-expand```)
- solana-cli >= 1.16.4

## Setup

1. Install the required dependencies using npm:
```bash
npm install
```

2. Build circuits:
```
npm run build
```

3. Execute the test suite using the following command:
```bash
npm run test
```

## Tutorial

In this tutorial you will build a private swap which can be used to negotiate and settle an over the counter (OTC) transaction.


### Primer on Light Protocol:
- Shielded Balance:
    - Shield(deposit): You can deposit(shield) value to Light Protocol. You transfer to the Light liquidity pool and receive a utxo in return.
    Example: Alice shields one sol thus receives one utxo worth 1 sol in return.
    - Utxos (Unspent transaction output) are used to store state and value.
  You can imagine a utxo similar to a bank note, which is single use and can be split up.
  Example: Alice has one utxo A which holds 1 sol.
  Alice sends Bob 0.5 sol. Utxo A is Alice's transaction input utxo. Bob receives utxo B worth 0.5. Alice receives a change utxo C worth the remaining 0.5 sol. In this transaction Utxo A 
  [See wikipedia](https://en.wikipedia.org/wiki/Unspent_transaction_output) for a more detailed explanation.
- Zero-knowledge proofs are used to prove validity of a transaction.


### Application flow

Alic wants to sell Sol and knows that Bob is a potential Bob.
**Alice:**
    1. Creates offer escrow utxo
**Bob:**
    2. Fetches offer escrow utxo
    3. Creates out utxos
    4. Generates system and PSP proofs
    5. Creates solana instructions
    6. Settles trade by invoking Swap PSP in 3 transactions

### Repo Structure:

1. circuits/swaps/swaps.light
    Defines the PSP logic in a zero-knowledege proof circuit
2. programs/swaps
    Defines the PSP onchain logic, including the verification of the PSP zero-knowledge proof.
    In this case there are no changes to the template code.
3. tests/swaps.ts
    Example execution of the PSP.

## Take Offer Instruction

1. Setup: create and fund Alice and Bob users.
2. Alice: creates offer utxo, to swap 10 sol for 300 USDC.
3. Bob: fetches and decrypt offer.
4. Bob: creates utxos and transaction parameters to accept and settle the swap.
5. Bob: generates the system proof (proves valid state transitions).
6. Bob: generates the PSP proof.
5. Bob: creates solana transactions to execute the PSP transaction.

## Cancel Instruction 

1. In ``circuits/swaps/swaps.light `` uncomment lines: 64-70, 111-114, 120-121
2. run `` npm run build`` to rebuild circuit and program
3. uncomment Swap Cancel functional test
