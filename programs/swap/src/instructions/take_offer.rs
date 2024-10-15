use crate::{constants::ANCHOR_DISCRIMINATOR, Offer};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

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
        has_one=token_mint_b,
        seeds=[b"odder",maker.key().as_ref(),offer.id.to_le_bytes()],
        bump 
    )]
    offer:Account<'info,Offer>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
