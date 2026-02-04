use anchor_lang::prelude::*;

declare_id!("GbTC2a7rohHvGejH8dtvrgEV6usdqrk8eJs6Du97Pzh");

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

#[program]
pub mod trust_protocol {
    use super::*;

    /// Register a new agent in the reputation system
    pub fn register_agent(ctx: Context<RegisterAgent>) -> Result<()> {
        instructions::register_agent::handler(ctx)
    }

    /// Record a successful job completion
    pub fn record_job_success(ctx: Context<RecordJob>, job_id: String) -> Result<()> {
        instructions::record_job::success_handler(ctx, job_id)
    }

    /// Record a failed job
    pub fn record_job_failure(ctx: Context<RecordJob>, job_id: String) -> Result<()> {
        instructions::record_job::failure_handler(ctx, job_id)
    }

    /// Endorse another agent's skill
    pub fn endorse_skill(
        ctx: Context<EndorseSkill>,
        skill_id: u8,
        comment: String,
    ) -> Result<()> {
        instructions::endorse::handler(ctx, skill_id, comment)
    }

    /// Add a skill to agent profile
    pub fn add_skill(
        ctx: Context<AddSkill>,
        skill_id: u8,
        skill_name: String,
        category: String,
    ) -> Result<()> {
        instructions::add_skill::handler(ctx, skill_id, skill_name, category)
    }
}
