use anchor_lang::prelude::*;

declare_id!("HavvbGE7ws5XSKMNpvTaqBPVbH5JGk7UUoKNW2NST6qb");

#[program]
pub mod likearray {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let array_account = &mut ctx.accounts.array_account;
        array_account.count = 0;
        Ok(())
    }

    pub fn add(ctx: Context<Add>, name: String) -> ProgramResult {
        let items_account = &mut ctx.accounts.items_account;
        let user = &mut ctx.accounts.user;

        let item = Item {
            name: name.to_string(),
            owner: *user.to_account_info().key,
        };

        items_account.items.push(item);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub array_account: Account<'info, ItemsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub items_account: Account<'info, ItemsAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Item {
    pub name: String,
    pub owner: Pubkey,
}

#[account]
pub struct ItemsAccount {
    pub items: Vec<Item>,
    pub count: u64,
}
