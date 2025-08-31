# Anchor Escrow Program

This repository contains an Anchor-based program for creating and managing escrow transactions on the Solana blockchain. The program allows a "maker" to create an escrow, locking tokens in a vault, and a "taker" to fulfill the escrow conditions by exchanging tokens.

## Key Features:

*   **Secure Token Exchange:** Facilitates trustless token swaps between two parties.
*   **Program-Controlled Vault:** Tokens are held in a vault controlled by the escrow program, ensuring secure and transparent management.
*   **Refund Mechanism:** Allows the maker to reclaim their tokens if the escrow conditions are not met.
*   **Associated Token Accounts (ATAs):** Utilizes ATAs for seamless token management.
*   **CPIs for Token Transfers:** Leverages Cross-Program Invocations (CPIs) to interact with the SPL Token program for token transfers.

## Account Ownership Hierarchy

*   **Maker:** The initiator of the escrow.
    *   Owns `maker_ata_a` (ATA for token A - the token being offered).
    *   Owns `maker_ata_b` (ATA for token B - the desired token).
    *   Pays for the creation of the `escrow` account.
    *   Receives the rent lamports when the `escrow` account is closed (in the `take` or `refund` instructions).
*   **Taker:** The party who fulfills the escrow conditions.
    *   Owns `taker_ata_a` (ATA for token A - the token being received).
    *   Owns `taker_ata_b` (ATA for token B - the token being offered).
    *   Pays for the creation of `taker_ata_a` and `maker_ata_b` if they don't already exist.
*   **Escrow:** The program-derived account (PDA) that manages the escrow.
    *   The PDA owns the `vault` (ATA for token A). The `vault`'s authority is set to the `escrow` account. The program controls the tokens in the vault through the `escrow` account's signing ability.

In essence, the `escrow` account acts as a secure intermediary, holding the tokens in the `vault` under its authority until the conditions of the escrow are met (either the taker takes the offer, or the maker refunds the tokens). The maker and taker own their respective token accounts, and the program uses CPIs to transfer tokens between these accounts under specific conditions.

## Instructions

The program includes the following instructions:

*   **make:** Creates a new escrow.
*   **take:** Executes the escrow, transferring tokens between the maker and taker.
*   **refund:** Cancels the escrow, returning tokens to the maker.

## Errors

The program defines several custom errors to handle specific scenarios, such as invalid amounts, unauthorized signers, and incorrect mint addresses.