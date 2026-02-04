# Trust Protocol SDK

TypeScript SDK for interacting with Trust Protocol on Solana.

## Installation

```bash
npm install @phronesis/trust-protocol-sdk
```

## Quick Start

```typescript
import { TrustProtocolClient, PROGRAM_ID } from "@phronesis/trust-protocol-sdk";
import { AnchorProvider } from "@coral-xyz/anchor";
import { Connection, clusterApiUrl } from "@solana/web3.js";

// Setup connection
const connection = new Connection(clusterApiUrl("devnet"));
const provider = new AnchorProvider(connection, wallet, {});

// Initialize client (you'll need the IDL from the program)
const client = new TrustProtocolClient(provider, idl);

// Register as an agent
await client.registerAgent("https://example.com/my-agent-metadata.json");

// Record a successful job
await client.recordJob("job-123", true);  // +10 trust

// Record a failed job
await client.recordJob("job-456", false); // -25 trust

// Get agent profile
const profile = await client.getAgentProfile(walletPublicKey);
console.log("Trust Score:", profile.trustScore);
console.log("Tier:", TrustProtocolClient.getTrustTier(profile.trustScore));
```

## Trust Tiers

| Score | Tier |
|-------|------|
| 900+ | Elite |
| 700-899 | Trusted |
| 500-699 | Neutral |
| 300-499 | Questionable |
| 0-299 | Untrusted |

## Program ID

**Devnet:** `GbTC2a7rohHvGejH8dtvrgEV6usdqrk8eJs6Du97Pzh`

## API Reference

### `registerAgent(metadataUri: string)`
Register a new agent with initial trust score of 500.

### `recordJob(jobId: string, success: boolean)`
Record a job outcome. Success adds 10 points, failure subtracts 25.

### `getAgentProfile(authority: PublicKey)`
Fetch an agent's profile including trust score and job history.

### `getAllAgents()`
Get all registered agents and their profiles.

## License

MIT
