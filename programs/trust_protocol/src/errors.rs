use anchor_lang::prelude::*;

#[error_code]
pub enum TrustProtocolError {
    #[msg("Agent is already registered")]
    AlreadyRegistered,

    #[msg("Agent is not registered")]
    NotRegistered,

    #[msg("Cannot endorse yourself")]
    SelfEndorsement,

    #[msg("Already endorsed this skill")]
    AlreadyEndorsed,

    #[msg("Skill not found on agent profile")]
    SkillNotFound,

    #[msg("Skill already exists on agent profile")]
    SkillAlreadyExists,

    #[msg("Skill name too long")]
    SkillNameTooLong,

    #[msg("Category name too long")]
    CategoryTooLong,

    #[msg("Comment too long")]
    CommentTooLong,

    #[msg("Job ID too long")]
    JobIdTooLong,

    #[msg("Invalid rating (must be 1-5)")]
    InvalidRating,
}
