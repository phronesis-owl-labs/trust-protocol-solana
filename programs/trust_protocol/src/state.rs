use anchor_lang::prelude::*;

/// Constants matching the EVM contract
pub const INITIAL_TRUST_SCORE: u64 = 500;  // Start at 50%
pub const MAX_TRUST_SCORE: u64 = 1000;
pub const JOB_SUCCESS_BONUS: u64 = 10;
pub const JOB_FAILURE_PENALTY: u64 = 25;
pub const ENDORSEMENT_BONUS: u64 = 5;

pub const MAX_SKILL_NAME_LEN: usize = 32;
pub const MAX_CATEGORY_LEN: usize = 32;
pub const MAX_COMMENT_LEN: usize = 256;
pub const MAX_JOB_ID_LEN: usize = 64;

/// Agent profile PDA - stores reputation data
/// Seeds: ["agent", agent_pubkey]
#[account]
#[derive(Default)]
pub struct AgentProfile {
    /// Agent's public key
    pub agent: Pubkey,
    /// Trust score (0-1000, divide by 10 for percentage)
    pub trust_score: u64,
    /// Number of successfully completed jobs
    pub jobs_completed: u32,
    /// Number of failed jobs
    pub jobs_failed: u32,
    /// Total endorsements received
    pub total_endorsements: u32,
    /// Sum of endorser trust scores
    pub endorsement_weight: u64,
    /// Registration timestamp
    pub registered_at: i64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl AgentProfile {
    pub const LEN: usize = 8  // discriminator
        + 32  // agent pubkey
        + 8   // trust_score
        + 4   // jobs_completed
        + 4   // jobs_failed
        + 4   // total_endorsements
        + 8   // endorsement_weight
        + 8   // registered_at
        + 1;  // bump
}

/// Job completion record PDA
/// Seeds: ["job", job_id]
#[account]
pub struct JobCompletion {
    /// The agent who completed the job
    pub agent: Pubkey,
    /// The client who posted the job
    pub client: Pubkey,
    /// Job outcome: 0=Success, 1=Failure, 2=Disputed
    pub outcome: u8,
    /// Rating (1-5)
    pub rating: u8,
    /// Completion timestamp
    pub timestamp: i64,
    /// Job ID string
    pub job_id: String,
    /// Bump seed
    pub bump: u8,
}

impl JobCompletion {
    pub const LEN: usize = 8  // discriminator
        + 32  // agent
        + 32  // client
        + 1   // outcome
        + 1   // rating
        + 8   // timestamp
        + 4 + MAX_JOB_ID_LEN  // job_id string
        + 1;  // bump
}

/// Skill record for an agent
/// Seeds: ["skill", agent_pubkey, skill_id]
#[account]
pub struct AgentSkill {
    /// Agent who has this skill
    pub agent: Pubkey,
    /// Skill identifier
    pub skill_id: u8,
    /// Skill name
    pub name: String,
    /// Skill category
    pub category: String,
    /// Number of endorsements for this skill
    pub endorsement_count: u32,
    /// Weighted score (sum of endorser trust scores)
    pub weighted_score: u64,
    /// Bump seed
    pub bump: u8,
}

impl AgentSkill {
    pub const LEN: usize = 8  // discriminator
        + 32  // agent
        + 1   // skill_id
        + 4 + MAX_SKILL_NAME_LEN  // name
        + 4 + MAX_CATEGORY_LEN    // category
        + 4   // endorsement_count
        + 8   // weighted_score
        + 1;  // bump
}

/// Skill endorsement record
/// Seeds: ["endorsement", endorser, endorsed_agent, skill_id]
#[account]
pub struct SkillEndorsement {
    /// Who gave the endorsement
    pub endorser: Pubkey,
    /// Who received the endorsement
    pub endorsed: Pubkey,
    /// Which skill was endorsed
    pub skill_id: u8,
    /// Endorser's trust score at time of endorsement
    pub weight: u64,
    /// Endorsement timestamp
    pub timestamp: i64,
    /// Optional comment
    pub comment: String,
    /// Bump seed
    pub bump: u8,
}

impl SkillEndorsement {
    pub const LEN: usize = 8  // discriminator
        + 32  // endorser
        + 32  // endorsed
        + 1   // skill_id
        + 8   // weight
        + 8   // timestamp
        + 4 + MAX_COMMENT_LEN  // comment
        + 1;  // bump
}

/// Job outcome enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum JobOutcome {
    Success = 0,
    Failure = 1,
    Disputed = 2,
}
