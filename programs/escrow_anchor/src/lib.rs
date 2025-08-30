use anchor_lang::prelude::*;

declare_id!("9S5m1NYWwtc7XhMP7Nb7UFTwhRvxRTupX2KMZsYWhYLF");

#[program]
pub mod escrow_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
