import { Program, AnchorProvider, Idl, BN } from "@coral-xyz/anchor";
import { Connection, PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

export const PROGRAM_ID = new PublicKey("GbTC2a7rohHvGejH8dtvrgEV6usdqrk8eJs6Du97Pzh");

export interface AgentProfile {
  authority: PublicKey;
  trustScore: number;
  jobsCompleted: number;
  jobsFailed: number;
  metadataUri: string;
  registeredAt: BN;
}

export interface JobCompletion {
  agent: PublicKey;
  jobId: string;
  success: boolean;
  timestamp: BN;
}

export class TrustProtocolClient {
  private program: Program;
  private provider: AnchorProvider;

  constructor(provider: AnchorProvider, idl: Idl) {
    this.provider = provider;
    this.program = new Program(idl, provider);
  }

  /**
   * Get the PDA for an agent profile
   */
  getAgentPDA(authority: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("agent"), authority.toBuffer()],
      PROGRAM_ID
    );
  }

  /**
   * Get the PDA for a job completion record
   */
  getJobPDA(agentProfile: PublicKey, jobId: string): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("job"), agentProfile.toBuffer(), Buffer.from(jobId)],
      PROGRAM_ID
    );
  }

  /**
   * Register a new agent with initial trust score of 500
   */
  async registerAgent(metadataUri: string): Promise<string> {
    const authority = this.provider.wallet.publicKey;
    const [agentPda] = this.getAgentPDA(authority);

    const tx = await this.program.methods
      .registerAgent(metadataUri)
      .accounts({
        agentProfile: agentPda,
        authority,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  /**
   * Record a job completion (success or failure)
   * Success: +10 trust score
   * Failure: -25 trust score
   */
  async recordJob(jobId: string, success: boolean): Promise<string> {
    const authority = this.provider.wallet.publicKey;
    const [agentPda] = this.getAgentPDA(authority);
    const [jobPda] = this.getJobPDA(agentPda, jobId);

    const tx = await this.program.methods
      .recordJob(success, jobId)
      .accounts({
        agentProfile: agentPda,
        jobCompletion: jobPda,
        authority,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    return tx;
  }

  /**
   * Fetch an agent's profile
   */
  async getAgentProfile(authority: PublicKey): Promise<AgentProfile | null> {
    const [agentPda] = this.getAgentPDA(authority);
    try {
      const account = await this.program.account.agentProfile.fetch(agentPda);
      return account as unknown as AgentProfile;
    } catch {
      return null;
    }
  }

  /**
   * Fetch a job completion record
   */
  async getJobCompletion(agentProfile: PublicKey, jobId: string): Promise<JobCompletion | null> {
    const [jobPda] = this.getJobPDA(agentProfile, jobId);
    try {
      const account = await this.program.account.jobCompletion.fetch(jobPda);
      return account as unknown as JobCompletion;
    } catch {
      return null;
    }
  }

  /**
   * Get all registered agents
   */
  async getAllAgents(): Promise<{ publicKey: PublicKey; account: AgentProfile }[]> {
    const accounts = await this.program.account.agentProfile.all();
    return accounts.map((a) => ({
      publicKey: a.publicKey,
      account: a.account as unknown as AgentProfile,
    }));
  }

  /**
   * Calculate trust tier based on score
   */
  static getTrustTier(score: number): string {
    if (score >= 900) return "Elite";
    if (score >= 700) return "Trusted";
    if (score >= 500) return "Neutral";
    if (score >= 300) return "Questionable";
    return "Untrusted";
  }
}

export default TrustProtocolClient;
