# 🛡️ Trust Protocol — On-Chain Reputation for AI Agents

> **Colosseum Hackathon Submission** | [Live Demo](https://phronesis-owl-labs.github.io/trust-protocol-solana/) | [Trust Visualizer](https://phronesis-owl-labs.github.io/trust-visualizer/) | Solana Devnet

## The Problem

As autonomous AI agents proliferate, there's no standard way to evaluate whether an agent is trustworthy before delegating work to it. How do you know if an agent will complete a task reliably? How do multi-agent systems coordinate without a reputation layer?

## The Solution

**Trust Protocol** is an on-chain reputation system built on Solana that enables:

- **Verifiable track records** — Every job outcome (success/failure) is recorded immutably
- **Skill endorsements** — Agents endorse each other's skills, weighted by the endorser's own reputation
- **Trust tiers** — Automated classification (Untrusted → Newcomer → Reliable → Trusted → Elite) based on on-chain history
- **Composable reputation** — Any dApp or agent framework can query trust scores before delegating tasks

## Architecture

```
┌─────────────────────────────────────────────────┐
│                Trust Protocol                    │
│                 (Solana Program)                  │
├────────────┬──────────────┬─────────────────────┤
│ AgentProfile│ JobCompletion │ SkillEndorsement   │
│    PDA      │     PDA       │      PDA           │
├────────────┴──────────────┴─────────────────────┤
│            TypeScript SDK                        │
├─────────────────────────────────────────────────┤
│         Agent Frameworks / dApps                 │
│    (OpenClaw, SAID, AgentDEX, AEGIS...)         │
└─────────────────────────────────────────────────┘
```

## Program Details

| | |
|---|---|
| **Program ID** | `GbTC2a7rohHvGejH8dtvrgEV6usdqrk8eJs6Du97Pzh` |
| **Chain** | Solana Devnet |
| **Framework** | Anchor |
| **Language** | Rust |

### Instructions

| Instruction | Description | Trust Impact |
|-------------|-------------|-------------|
| `register_agent` | Register a new agent (initial score: 500/1000) | — |
| `add_skill` | Declare a skill with name and category | — |
| `record_job` (success) | Record a successful job completion | +10 trust |
| `record_job` (failure) | Record a failed job | -25 trust |
| `endorse_skill` | Endorse another agent's skill | Weighted by endorser reputation |

### Trust Tiers

| Tier | Score Range | Meaning |
|------|-----------|---------|
| 🔴 Untrusted | 0–199 | Too many failures, high risk |
| 🟡 Newcomer | 200–399 | New agent, limited history |
| 🟢 Reliable | 400–599 | Consistent performer |
| 🔵 Trusted | 600–799 | Strong track record |
| ⭐ Elite | 800–1000 | Top-tier, community-endorsed |

### PDA Seeds

| Account | Seeds |
|---------|-------|
| AgentProfile | `["agent", agent_pubkey]` |
| JobCompletion | `["job", job_id_bytes]` |
| AgentSkill | `["skill", agent_pubkey, skill_id]` |
| SkillEndorsement | `["endorsement", endorser, endorsed, skill_id]` |

## SDK

Full TypeScript SDK for easy integration:

```bash
npm install @phronesis/trust-protocol-sdk
```

```typescript
import { TrustProtocolClient } from "@phronesis/trust-protocol-sdk";

// Initialize
const client = new TrustProtocolClient(provider, idl);

// Register as an agent
await client.registerAgent("https://example.com/metadata.json");

// Record job outcomes
await client.recordJob("job-123", true);   // success: +10 trust
await client.recordJob("job-456", false);  // failure: -25 trust

// Query reputation
const profile = await client.getAgentProfile(publicKey);
console.log(`Trust: ${profile.trustScore} (${TrustProtocolClient.getTrustTier(profile.trustScore)})`);
```

See [SDK documentation](./sdk/README.md) for full API reference.

## Integration Opportunities

Trust Protocol is designed to be a composable primitive. Potential integrations:

- **SAID Protocol** — Identity + reputation combo for agent verification
- **AgentDEX** — Reputation-gated trading and task delegation
- **AEGIS** — Swarm task assignment based on trust scores
- **Agent Yield Router** — Treasury access control weighted by reputation
- **ZNAP** — Social reputation layer for agent communities

## Also Deployed on Base (EVM)

Trust Protocol also runs on Base mainnet as part of the [Clawathon hackathon](https://team-phronesis-labs.vercel.app):

| Contract | Address |
|----------|---------|
| ReputationRegistry | `0x96BF408C918355a4AE3EE5eedf962F647c962e0d` |
| SkillEndorsement | `0x4d2Db474D472dCF7aACD694120adD70ED02f9Ec9` |

## Building from Source

```bash
# Prerequisites: Rust, Solana CLI, Anchor

# Build
anchor build

# Test
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## Team

Built by **Phronesis Labs** 🦉

## License

MIT
