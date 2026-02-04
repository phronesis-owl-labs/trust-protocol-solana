use anchor_lang::prelude::*;
use crate::state::{AgentProfile, INITIAL_TRUST_SCORE};

#[derive(Accounts)]
pub struct RegisterAgent<'info> {
    #[account(
        init,
        payer = agent,
        space = AgentProfile::LEN,
        seeds = [b"agent", agent.key().as_ref()],
        bump
    )]
    pub agent_profile: Account<'info, AgentProfile>,

    #[account(mut)]
    pub agent: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterAgent>) -> Result<()> {
    let agent_profile = &mut ctx.accounts.agent_profile;
    let clock = Clock::get()?;

    agent_profile.agent = ctx.accounts.agent.key();
    agent_profile.trust_score = INITIAL_TRUST_SCORE;
    agent_profile.jobs_completed = 0;
    agent_profile.jobs_failed = 0;
    agent_profile.total_endorsements = 0;
    agent_profile.endorsement_weight = 0;
    agent_profile.registered_at = clock.unix_timestamp;
    agent_profile.bump = ctx.bumps.agent_profile;

    msg!("Agent registered: {}", ctx.accounts.agent.key());
    msg!("Initial trust score: {}", INITIAL_TRUST_SCORE);

    Ok(())
}
