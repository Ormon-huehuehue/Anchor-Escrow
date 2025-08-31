# Anchor-Escrow
In essence, the `escrow` account acts as a secure intermediary, holding the tokens in the `vault` under its authority until the conditions of the escrow are met (either the taker takes the offer, or the maker refunds the tokens). The maker and taker own their respective token accounts, and the program uses CPIs to transfer tokens between these accounts under specific conditions.

## Account Ownership Hierarchy

*   **Maker:**
    *   Owns `maker_ata_a` (ATA for token A)
    *   Owns `maker_ata_b` (ATA for token B)
    *   Pays for the creation of the `escrow` account.
    *   Receives the rent lamports when the `escrow` account is closed
*   **Taker:**
    *   Owns `taker_ata_a` (ATA for token A)
    *   Owns `taker_ata_b` (ATA for token B)
    *   Pays for the creation of `taker_ata_a` and `maker_ata_b` if they don't already exist.
*   **Escrow:**
    *   The PDA owns the `vault` (ATA for token A). The `vault`'s authority is set to the `escrow` account. The program controls the tokens in the vault through the `escrow` account's signing ability.

In essence, the `escrow` account acts as a secure intermediary, holding the tokens in the `vault` under its authority until the conditions of the escrow are met (either the taker takes the offer, or the maker refunds the tokens). The maker and taker own their respective token accounts, and the program uses CPIs to transfer tokens between these accounts under specific conditions.