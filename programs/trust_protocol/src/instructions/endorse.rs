use anchor_lang::prelude::*;
use crate::state::{
    AgentProfile, AgentSkill, SkillEndorsement,
    MAX_TRUST_SCORE, ENDORSEMENT_BONUS, MAX_COMMENT_LEN
};
use crate::errors::TrustProtocolError;

#[derive(Accounts)]
#[instruction(skill_id: u8, comment: String)]
pub struct EndorseSkill<'info> {
    /// Endorser's profile (must be registered)
    #[account(
        seeds = [b"agent", endorser.key().as_ref()],
        bump = endorser_profile.bump,
    )]
    pub endorser_profile: Account<'info, AgentProfile>,

    /// Endorsed agent's profile
    #[account(
        mut,
        seeds = [b"agent", endorsed_agent.key().as_ref()],
        bump = endorsed_profile.bump,
    )]
    pub endorsed_profile: Account<'info, AgentProfile>,

    /// The skill being endorsed
    #[account(
        mut,
        seeds = [b"skill", endorsed_agent.key().as_ref(), &[skill_id]],
        bump = agent_skill.bump,
    )]
    pub agent_skill: Account<'info, AgentSkill>,

    /// The endorsement record (ensures uniqueness)
    #[account(
        init,
        payer = endorser,
        space = SkillEndorsement::LEN,
        seeds = [b"endorsement", endorser.key().as_ref(), endorsed_agent.key().as_ref(), &[skill_id]],
        bump
    )]
    pub endorsement: Account<'info, SkillEndorsement>,

    #[account(mut)]
    pub endorser: Signer<'info>,

    /// CHECK: Just need pubkey for PDA seeds
    pub endorsed_agent: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EndorseSkill>, skill_id: u8, comment: String) -> Result<()> {
    require!(comment.len() <= MAX_COMMENT_LEN, TrustProtocolError::CommentTooLong);
    require!(
        ctx.accounts.endorser.key() != ctx.accounts.endorsed_agent.key(),
        TrustProtocolError::SelfEndorsement
    );

    let endorser_trust = ctx.accounts.endorser_profile.trust_score;
    let clock = Clock::get()?;

    // Create endorsement record
    let endorsement = &mut ctx.accounts.endorsement;
    endorsement.endorser = ctx.accounts.endorser.key();
    endorsement.endorsed = ctx.accounts.endorsed_agent.key();
    endorsement.skill_id = skill_id;
    endorsement.weight = endorser_trust;
    endorsement.timestamp = clock.unix_timestamp;
    endorsement.comment = comment;
    endorsement.bump = ctx.bumps.endorsement;

    // Update skill stats
    let skill = &mut ctx.accounts.agent_skill;
    skill.endorsement_count += 1;
    skill.weighted_score += endorser_trust;

    // Update endorsed agent's profile
    let endorsed = &mut ctx.accounts.endorsed_profile;
    let old_score = endorsed.trust_score;
    endorsed.total_endorsements += 1;
    endorsed.endorsement_weight += endorser_trust;

    // Bonus based on endorser's reputation (same formula as EVM)
    let bonus = (endorser_trust * ENDORSEMENT_BONUS) / MAX_TRUST_SCORE;
    endorsed.trust_score = std::cmp::min(endorsed.trust_score + bonus, MAX_TRUST_SCORE);

    msg!(
        "Endorsement: {} endorsed {}'s skill {} with weight {}",
        ctx.accounts.endorser.key(),
        ctx.accounts.endorsed_agent.key(),
        skill_id,
        endorser_trust
    );
    msg!("Trust score: {} -> {}", old_score, endorsed.trust_score);

    Ok(())
}
