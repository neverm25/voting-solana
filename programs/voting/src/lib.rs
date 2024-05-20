use anchor_lang::prelude::*;

declare_id!("Cz97tPLkCNRXAWYApAPU8xTjmqCVE8TAoVswF9vb1YjF");

const OWNER: &str = "FXkpCdzzt26WimkmJs3MswkcRpzEBUbRGxACaadUhtHJ";

#[program]

pub mod voting {

    use super::*;

    #[access_control(check(&ctx))]

    pub fn init_candidate(
        ctx: Context<InitializeCandidate>,
        _candidate_name: String,
    ) -> Result<()> {
        Ok(())
    }

    pub fn vote_for_candidate(ctx: Context<VoteCandidate>, _candidate_name: String) -> Result<()> {
        ctx.accounts.candidate.votes_received += 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_candidate_name: String)]

pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 8 + Candidate::INIT_SPACE,
        payer = payer,
        seeds = [_candidate_name.as_bytes().as_ref()],
        bump,
    )]
    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_candidate_name: String)]

pub struct VoteCandidate<'info> {
    #[account(
        mut,
        seeds = [_candidate_name.as_bytes().as_ref()],
        bump,
    )]
    pub candidate: Account<'info, Candidate>,
}

#[account]
#[derive(InitSpace)]

pub struct Candidate {
    pub votes_received: u8,
}

fn check(ctx: &Context<InitializeCandidate>) -> Result<()> {
    // Check if signer === owner

    require_keys_eq!(
        ctx.accounts.payer.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

#[error_code]

pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}
