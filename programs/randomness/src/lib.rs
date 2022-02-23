use anchor_lang::{prelude::*, solana_program};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod randomness {
    use anchor_lang::solana_program::log::sol_log_compute_units;

    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let ixns = ctx.accounts.instructions.to_account_info();
        msg!("ixns len: {:?}", ixns.data_len());
        let current_index =
            solana_program::sysvar::instructions::load_current_index_checked(&ixns)? as usize;
        msg!("current index: {:?}", current_index);
        let current_ixn = solana_program::sysvar::instructions::load_instruction_at_checked(
            current_index,
            &ixns,
        )?;
        msg!("current ixn: {:?}", current_ixn);

        let mut i = current_index + 1;
        let mut total_indices = current_index;
        loop {
            if let Ok(ixn) =
                solana_program::sysvar::instructions::load_instruction_at_checked(i, &ixns)
            {
                total_indices += 1;
                i += 1;
            } else {
                break;
            }
        }
        msg!("total indices: {}", total_indices);
        // if total_indices > current_index {
        //     return Err(error!(ErrorCode::NothingAfter));
        // }
        panic!();
        //sol_log_compute_units();
        Ok(())
    }

    pub fn do_nothing(ctx: Context<DoNothing>) -> Result<()> {
        Ok(())
    }
}

//83
//grows to 116 with extra account. interesting
#[derive(Accounts)]
pub struct Initialize<'info> {
    ///CHECK: address
    #[account(address = solana_program::sysvar::instructions::id())]
    instructions: UncheckedAccount<'info>,
    ///CHECK: shut up
    extra: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct DoNothing {}

#[error_code]
pub enum ErrorCode {
    #[msg("one ixn my guy")]
    OneIxn,
    #[msg("can't pack instructions after this one")]
    NothingAfter,
}
