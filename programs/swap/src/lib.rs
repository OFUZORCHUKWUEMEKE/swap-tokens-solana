pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4R7mUYNDGrW6Z1smH7G3UP6GYStuuYx2fL52MqBa96ZL");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>,id:u64,token_a_offered_amount:u64,token_b_wanted_amount:u64) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&ctx, token_a_offered_amount)?;
        instructions::make_offer::saveOffer(ctx, id, token_b_wanted_amount)?;
        Ok(())
    }
}
