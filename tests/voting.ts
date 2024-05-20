import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Voting } from "../target/types/voting";

describe("voting", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Voting as Program<Voting>;

  const candidateName = "John Smith";

  it("Initializes a candidate", async () => {
    const [candidatePda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(candidateName)],
      program.programId
    );

    await program.methods.initCandidate(candidateName).rpc();

    const candidateAccount = await program.account.candidate.fetch(candidatePda);
    expect(candidateAccount.votesReceived).equal(0);
  });

  it("Votes for a candidate", async () => {
    const [candidatePda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(candidateName)],
      program.programId
    );

    await program.methods.voteForCandidate(candidateName).rpc();

    const candidateAccount = await program.account.candidate.fetch(candidatePda);
    expect(candidateAccount.votesReceived).equal(1);
  });

});
