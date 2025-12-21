use anchor_lang::prelude::*;

declare_id!("E7Ha7cJQCddaNtqMA2okwJd12rHU2uUWfJMGEtNjswuk");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(_ctx: Context<IntializePoll>, _poll_id: u64) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(poll_id: u64)] //passing poll_id as instruction argument
pub struct IntializePoll<'info> {
    //signer account
    #[account(mut)]
    pub signer: Signer<'info>,

    //poll account
    #[account(̥init,
        payer= payer,
        space= 8  + Poll::InitSpace, //8 bytes for account discriminator + space needed for Poll struct
        seeds= [poll_id.to_le_bytes().as_ref()]
        bump, //program derived address bump
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)] //auto calculate space needed
pub struct Poll{
    pub poll_id: u64,
    #[max_len(280)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}