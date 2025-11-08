use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken; // ATA creation
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer, SetAuthority};

declare_id!("332rrVwdQRSz6VKin6UjZb7KtvG9WHzJuJXABszzrdgi");

#[program]
pub mod day34 {
    use super::*;

    pub fn create_and_mint_token(ctx: Context<CreateMint>) -> Result<()>{

        let mint_amount = 100_000_000_000; // 100 token with 9 decimals
        let mint = ctx.accounts.new_mint.clone();
        let destination_ata = &ctx.accounts.new_ata;
        let authority = ctx.accounts.signer.clone();
        let token_program = ctx.accounts.token_program.clone();

        let mint_intruction = MintTo {
            mint: mint.to_account_info(),
            to: destination_ata.to_account_info(),
            authority: authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(token_program.to_account_info(), mint_intruction);
        token::mint_to(cpi_ctx, mint_amount)?;

        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferSpl>, amount: u64) -> Result<()> {
        let source_ata = &ctx.accounts.from_ata;
        let destination_ata = &ctx.accounts.to_ata;
        let authority = &ctx.accounts.from;
        let token_program = &ctx.accounts.token_program;

        let cpi_accounts = Transfer {
            from: source_ata.to_account_info(),
            to: destination_ata.to_account_info(),
            authority: authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn get_balance(ctx: Context<GetBalance>) -> Result<()> {
        let ata_pubkey = ctx.accounts.token_account.key();
        let owner = ctx.accounts.token_account.owner;
        let balance = ctx.accounts.token_account.amount;

        msg!("Token Account Address: {}", ata_pubkey);
        msg!("Token Account Owner: {}", owner);
        msg!("Token Account Balance: {}", balance);

        Ok(())
    }

    pub fn disable_mint_authority(ctx: Context<DisableAuthority>) -> Result<()>{
        let cpi_accounts = SetAuthority {
            current_authority: ctx.accounts.signer.to_account_info(),
            account_or_mint: ctx.accounts.mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::set_authority(cpi_ctx, token::spl_token::instruction::AuthorityType::MintTokens, None)?;
        Ok(())
    }

    #[derive(Accounts)]
    pub struct DisableAuthority<'info> {
        #[account(mut)]
        pub signer: Signer<'info>,

        #[account(
            mut,
            seeds = [ b"my_mint", signer.key().as_ref()],
            bump
        )]
        pub mint: Account<'info, Mint>,

        pub token_program: Program<'info, Token>,
    }

    #[derive(Accounts)]
    pub struct GetBalance<'info> {
        pub token_account: Account<'info, TokenAccount>,
    }

    #[derive(Accounts)]
    pub struct CreateMint<'info> {
        #[account(mut)]
        pub signer: Signer<'info>,

        #[account(
            init,
            payer = signer,
            mint::decimals = 9,
            mint::authority = signer,
            // Comment this line to disable freeze authority, you won't. be able to update the token later , more decentralized
            mint::freeze_authority = signer,
            seeds = [ b"my_mint", signer.key().as_ref() ],
            bump
        )]
        pub new_mint: Account<'info, Mint>,

        #[account(
            init,
            payer = signer,
            associated_token::mint = new_mint,
            associated_token::authority = signer,
        )]
        pub new_ata: Account<'info, TokenAccount>,

        // SPL token program
        pub token_program: Program<'info, Token>,

        // Ata program
        pub associated_token_program: Program<'info, AssociatedToken>,

        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct TransferSpl<'info>{
        pub from: Signer<'info>,
        #[account(mut)]
        pub from_ata: Account<'info, TokenAccount>,
        #[account(mut)]
        pub to_ata: Account<'info,  TokenAccount>,

        pub token_program: Program<'info, Token>,
    }
}
