use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount}};

//initializes the escrow record and stores all the terms

//creates the vault (an ATA for mint_a owned by the escrow)

//Moves the maker's token A into the vault with a CPI to the SPL-Token Program


#[derive(Accounts)]
#[instruction(seeds : u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(
        init, 
        payer = maker,
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [b"escrow", maker.key().as_ref(), seeds.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow : Account<'info, Escrow>,

    //Token accounts
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'info, Mint>, //InterfaceAccount works for both Token and Token2022 accounts

    #[account(
        mint::token_program = token_program
    )]
    pub mint_b : InterfaceAccount<'info, Mint>,


    #[account(
        mut, 
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a : InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>,

    //Programs

    pub associated_token_program : Program<'info, AssociatedToken>,

    //why TokenInterface ? 
    // Interface<'info, Token> would only work with classic SPL-Token program, but with TokenInterface -> it can work with both classic spl-token and token-2022 programs both
    pub token_program : Interface<'info, TokenInterface>,
    pub system_program : Program<'info, System>
}


impl <'info> Make<'info> {
    //create the escrow
}