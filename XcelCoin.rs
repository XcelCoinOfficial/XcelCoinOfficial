use anchor_lang::prelude::*;

declare_id!("XcelCoin1111111111111111111111111111111");

#[program]
pub mod xcel_coin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        let token_account = &mut ctx.accounts.token_account;
        token_account.name = "XcelCoin".to_string();
        token_account.symbol = "XCEL".to_string();
        token_account.total_supply = total_supply;
        token_account.decimals = 9;
        token_account.owner = *ctx.accounts.initializer.key;
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let from = &mut ctx.accounts.from;
        let to = &mut ctx.accounts.to;
        require!(from.balance >= amount, ErrorCode::InsufficientBalance);
        from.balance -= amount;
        to.balance += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, space = 8 + 40)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
}

#[account]
pub struct TokenAccount {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub decimals: u8,
    pub owner: Pubkey,
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient Balance.")]
    InsufficientBalance,
}
