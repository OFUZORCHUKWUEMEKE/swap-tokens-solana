use crate::{Offer};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked,transfer_checked,CloseAccount};

use super::transfer_tokens;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=token_mint_a,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_token_account_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint=token_mint_b,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_token_account_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=token_mint_b,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_token_account_b:Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        close=maker,
        has_one=token_mint_a,
        seeds=[b"offer",maker.key().as_ref(),offer.id.to_le_bytes().as_ref()],
        bump 
    )]
    offer:Account<'info,Offer>,
    #[account(
    mut,
    associated_token::mint=token_mint_a,
    associated_token::authority=maker,
    associated_token::token_program=token_program
    )]
    vault:InterfaceAccount<'info,TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_wanted_token_offer(ctx:&Context<TakeOffer>)->Result<()>{
    transfer_tokens(&ctx.accounts.taker_token_account_b, &ctx.accounts.maker_token_account_b, &ctx.accounts.offer.token_b_wanted_amount, &ctx.accounts.token_mint_b, &ctx.accounts.taker,&ctx.accounts.token_program)?;
    Ok(())
}

pub fn withdraw_amcd_close_vault(context:Context<TakeOffer>)->Result<()>{
    let seeds = &[b"offer",context.accounts.maker.to_account_info().key.as_ref(),&context.accounts.offer.id.to_le_bytes(),&[context.accounts.offer.bump]];

    let signer_seeds=[&seeds[..]];
    let accounts= TransferChecked{
        from:context.accounts.vault.to_account_info(),
        to:context.accounts.taker_token_account_a.to_account_info(),
        mint:context.accounts.token_mint_a.to_account_info(),
        authority:context.accounts.offer.to_account_info()
    };
    let cpi_context = CpiContext::new_with_signer(context.accounts.token_program.to_account_info(), accounts, &signer_seeds);
    transfer_checked(
        cpi_context,
        context.accounts.vault.amount,
        context.accounts.token_mint_a.decimals
    )?;
    let _account = CloseAccount{
        account:context.accounts.vault.to_account_info(),
        destination:context.accounts.taker.to_account_info(),
        authority:context.accounts.offer.to_account_info()
    };
    Ok(())
}