use anchor_lang::prelude::*;
use crate::state::{AgentProfile, AgentSkill, MAX_SKILL_NAME_LEN, MAX_CATEGORY_LEN};
use crate::errors::TrustProtocolError;

#[derive(Accounts)]
#[instruction(skill_id: u8, skill_name: String, category: String)]
pub struct AddSkill<'info> {
    /// Agent's profile (must be registered)
    #[account(
        seeds = [b"agent", agent.key().as_ref()],
        bump = agent_profile.bump,
    )]
    pub agent_profile: Account<'info, AgentProfile>,

    /// The skill being added
    #[account(
        init,
        payer = agent,
        space = AgentSkill::LEN,
        seeds = [b"skill", agent.key().as_ref(), &[skill_id]],
        bump
    )]
    pub agent_skill: Account<'info, AgentSkill>,

    #[account(mut)]
    pub agent: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<AddSkill>,
    skill_id: u8,
    skill_name: String,
    category: String,
) -> Result<()> {
    require!(skill_name.len() <= MAX_SKILL_NAME_LEN, TrustProtocolError::SkillNameTooLong);
    require!(category.len() <= MAX_CATEGORY_LEN, TrustProtocolError::CategoryTooLong);

    let skill = &mut ctx.accounts.agent_skill;
    
    skill.agent = ctx.accounts.agent.key();
    skill.skill_id = skill_id;
    skill.name = skill_name.clone();
    skill.category = category.clone();
    skill.endorsement_count = 0;
    skill.weighted_score = 0;
    skill.bump = ctx.bumps.agent_skill;

    msg!(
        "Skill added: {} ({}) for agent {}",
        skill_name,
        category,
        ctx.accounts.agent.key()
    );

    Ok(())
}
