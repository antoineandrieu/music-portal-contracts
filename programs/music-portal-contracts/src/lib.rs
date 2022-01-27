use anchor_lang::prelude::*;

declare_id!("9wqcUtiFxUnuqYbAbBCp9XJjAvJWYzdZsPLiup4r56AJ");

#[program]
pub mod music_portal_contracts {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Get a reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs
        base_account.total_songs = 0;
        Ok(())
    }

    pub fn add_song(ctx: Context<AddSong>, song_link: String) -> ProgramResult {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            song_link: song_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        base_account.song_list.push(item);
        base_account.total_songs += 1;
        Ok(())
    }
}

// Attach variables to the context
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddSong<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub song_link: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account
#[account]
pub struct BaseAccount {
    pub total_songs: u64,
    pub song_list: Vec<ItemStruct>,
}
