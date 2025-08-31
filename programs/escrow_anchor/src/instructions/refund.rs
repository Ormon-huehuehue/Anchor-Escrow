use anchor_lang::prelude::*;
use crate::errors::EscrowError;
use crate::state::Escrow;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};


#[derive(Accounts)]
pub struct Refund<'info>{
    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(
        mut, 
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker,
        has_one = mint_a @ EscrowError::InvalidMintA
    )]
    pub escrow : Box<Account<'info, Escrow>>,

    pub mint_a : Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account( 
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a : Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,  
}


impl <'info> Refund<'info>{
    fn withdraw_funds_and_close_vault(&mut self)-> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from : self.vault.to_account_info(),
            to : self.maker_ata_a.to_account_info(),
            mint : self.mint_a.to_account_info(),
            authority : self.escrow.to_account_info()
        };

        let close_cpi_accounts = CloseAccount{
            account : self.vault.to_account_info(),
            authority : self.escrow.to_account_info(),
            destination : self.maker.to_account_info()
        };

        let signer_key = self.maker.key();
        let escrow_seeds = self.escrow.seed.to_le_bytes();

        let seeds = &[b"escrow", signer_key.as_ref(), &escrow_seeds.as_ref(), &[self.escrow.bump] ];

        let signer_seeds = &[&seeds[..]];


        let cpi_context = CpiContext::new_with_signer(
            cpi_program.clone(), 
            cpi_accounts, 
            signer_seeds
        );

        transfer_checked(
            cpi_context, 
            self.vault.amount, 
            self.mint_a.decimals
        )?;

        let close_cpi_context = CpiContext::new_with_signer(
            cpi_program, 
            close_cpi_accounts, 
            signer_seeds
        );

        close_account(
            close_cpi_context
        )?;


        Ok(())

    }
}

pub fn handler(ctx: Context<Refund>)->Result<()>{
    ctx.accounts.withdraw_funds_and_close_vault()?;

    Ok(())
}