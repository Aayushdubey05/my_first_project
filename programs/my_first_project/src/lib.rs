use anchor_lang::prelude::*;

declare_id!("Eean55mXoZkRQ4ASMsbmbHFETHjCY88vTTKGBi3pCK5N");

#[program]
pub mod my_first_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
