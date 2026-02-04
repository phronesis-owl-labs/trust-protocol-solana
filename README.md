# Trust Protocol - Solana/Anchor

Ported from EVM to Solana for the Colosseum Hackathon.

## Overview

On-chain reputation system for AI agents:
- **AgentProfile PDA** — Stores trust score, job history, endorsement weight
- **JobCompletion PDA** — Records individual job outcomes
- **AgentSkill PDA** — Skills an agent claims to have
- **SkillEndorsement PDA** — Endorsements from other agents

## Building

```bash
# Install Anchor (if not installed)
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm install latest
avm use latest

# Build
anchor build

# Test
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## Program Instructions

### `register_agent`
Register a new agent with initial trust score of 500 (50%).

### `add_skill(skill_id, name, category)`
Add a skill to your agent profile.

### `record_job_success(job_id)` / `record_job_failure(job_id)`
Record job outcomes. Success adds +10 trust, failure subtracts -25.

### `endorse_skill(skill_id, comment)`
Endorse another agent's skill. Weight is based on endorser's trust score.

## PDA Seeds

| Account | Seeds |
|---------|-------|
| AgentProfile | `["agent", agent_pubkey]` |
| JobCompletion | `["job", job_id_bytes]` |
| AgentSkill | `["skill", agent_pubkey, skill_id]` |
| SkillEndorsement | `["endorsement", endorser, endorsed, skill_id]` |

## Integration

Query an agent's reputation before interacting:

```typescript
const [agentPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("agent"), agentPubkey.toBuffer()],
  programId
);

const profile = await program.account.agentProfile.fetch(agentPda);
console.log(`Trust score: ${profile.trustScore / 10}%`);
```

## License

MIT
