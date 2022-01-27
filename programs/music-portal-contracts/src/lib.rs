use anchor_lang::prelude::*;

declare_id!("9wqcUtiFxUnuqYbAbBCp9XJjAvJWYzdZsPLiup4r56AJ");

#[program]
pub mod musicportalcontracts {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_songs = 0;
        Ok(())
    }

    // The function now accepts a song_link param from the user. We also reference the user from the Context
    pub fn add_song(ctx: Context<Addsong>, song_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            song_link: song_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        // Add it to the song_list vector.
        base_account.song_list.push(item);
        base_account.total_songs += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Add the signer who calls the Addsong method to the struct so that we can save it
#[derive(Accounts)]
pub struct Addsong<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub song_link: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_songs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub song_list: Vec<ItemStruct>,
}
