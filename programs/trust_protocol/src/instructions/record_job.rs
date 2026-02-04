use anchor_lang::prelude::*;
use crate::state::{
    AgentProfile, JobCompletion, 
    MAX_TRUST_SCORE, JOB_SUCCESS_BONUS, JOB_FAILURE_PENALTY, MAX_JOB_ID_LEN
};
use crate::errors::TrustProtocolError;

#[derive(Accounts)]
#[instruction(job_id: String)]
pub struct RecordJob<'info> {
    #[account(
        mut,
        seeds = [b"agent", agent.key().as_ref()],
        bump = agent_profile.bump,
    )]
    pub agent_profile: Account<'info, AgentProfile>,

    #[account(
        init,
        payer = client,
        space = JobCompletion::LEN,
        seeds = [b"job", job_id.as_bytes()],
        bump
    )]
    pub job_completion: Account<'info, JobCompletion>,

    /// The agent who completed/failed the job
    /// CHECK: We just need the pubkey, validation done via PDA seeds
    pub agent: UncheckedAccount<'info>,

    /// The client recording the job outcome
    #[account(mut)]
    pub client: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn success_handler(ctx: Context<RecordJob>, job_id: String) -> Result<()> {
    require!(job_id.len() <= MAX_JOB_ID_LEN, TrustProtocolError::JobIdTooLong);

    let agent_profile = &mut ctx.accounts.agent_profile;
    let job = &mut ctx.accounts.job_completion;
    let clock = Clock::get()?;

    let old_score = agent_profile.trust_score;
    agent_profile.jobs_completed += 1;
    agent_profile.trust_score = std::cmp::min(
        agent_profile.trust_score + JOB_SUCCESS_BONUS,
        MAX_TRUST_SCORE
    );

    job.agent = ctx.accounts.agent.key();
    job.client = ctx.accounts.client.key();
    job.outcome = 0; // Success
    job.rating = 5;  // Default 5-star for success
    job.timestamp = clock.unix_timestamp;
    job.job_id = job_id.clone();
    job.bump = ctx.bumps.job_completion;

    msg!("Job {} completed successfully", job_id);
    msg!("Trust score: {} -> {}", old_score, agent_profile.trust_score);

    Ok(())
}

pub fn failure_handler(ctx: Context<RecordJob>, job_id: String) -> Result<()> {
    require!(job_id.len() <= MAX_JOB_ID_LEN, TrustProtocolError::JobIdTooLong);

    let agent_profile = &mut ctx.accounts.agent_profile;
    let job = &mut ctx.accounts.job_completion;
    let clock = Clock::get()?;

    let old_score = agent_profile.trust_score;
    agent_profile.jobs_failed += 1;
    agent_profile.trust_score = agent_profile.trust_score
        .saturating_sub(JOB_FAILURE_PENALTY);

    job.agent = ctx.accounts.agent.key();
    job.client = ctx.accounts.client.key();
    job.outcome = 1; // Failure
    job.rating = 1;  // 1-star for failure
    job.timestamp = clock.unix_timestamp;
    job.job_id = job_id.clone();
    job.bump = ctx.bumps.job_completion;

    msg!("Job {} failed", job_id);
    msg!("Trust score: {} -> {}", old_score, agent_profile.trust_score);

    Ok(())
}
