import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tiamo } from "../target/types/tiamo";
import assert from "chai";



describe("tiamo", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
 

  const program = anchor.workspace.Tiamo as Program<Tiamo>;

  it("Is created team!", async () => {
    // Creating team test here.
    
    let createTeam;
    const [team] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("team_account"),
        anchor.utils.bytes.utf8.encode("Blues Team"),
      ],
      program.programId
    )
    console.log(`User1 key: ${provider.wallet.publicKey.toBase58()}`);
    console.log(`Team acc: ${team.toBase58()}`);
    const ix = program.methods.createTeam("Blues Team", 10).accounts({
      teamAccount: team,
    });
    const tx = await ix.rpc().catch(console.error);
    console.log(`Transaction id: ${tx}`);
    const acc = (await ix.pubkeys()).teamAccount;
    const data = await program.account.team.fetch(acc);
    console.log(data.teamAutority.toBase58());
  });
  it("Create a proposal", async () => {
    // Creating proposal test here.
    const [proposal] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("prpopsal_account"),
        anchor.utils.bytes.utf8.encode("Blues Team"),
      ],
      program.programId
    )
    console.log(`User1 key: ${provider.wallet.publicKey.toBase58()}`);
    console.log(`Team acc: ${proposal.toBase58()}`);
    const ix = program.methods.createProposal("Blues Team", 10).accounts({
      teamAccount: proposal,
    });
    const tx = await ix.rpc().catch(console.error);
    console.log(`Transaction id: ${tx}`);
    const acc = (await ix.pubkeys()).teamAccount;
    const data = await program.account.proposal.fetch(acc.publicKey);

    console.log(data.teamAutority.toBase58());
  });
  it("Join a team", async () => {
    // Join team test here.
    const [team] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("team_account"),
        anchor.utils.bytes.utf8.encode("Blues Team"),
      ],
      program.programId
    )
    console.log(`User1 key: ${provider.wallet.publicKey.toBase58()}`);
    console.log(`Team acc: ${team.toBase58()}`);
    const ix = program.methods.joinTeam("Blues Team", 10).accounts({
      teamAccount: team,
    });
    const tx = await ix.rpc().catch(console.error);
    console.log(`Transaction id: ${tx}`);
    const acc = (await ix.pubkeys()).teamAccount;
    const data = await program.account.team.fetch(acc.publicKey);

    console.log(data.teamAutority.toBase58());
  });
  it("Leave from  team", async () => {
    // Leave from a teamtest here.
    const [team] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("team_account"),
        anchor.utils.bytes.utf8.encode("Blues Team"),
      ],
      program.programId
    )
    console.log(`User1 key: ${provider.wallet.publicKey.toBase58()}`);
    console.log(`Team acc: ${team.toBase58()}`);
    const ix = program.methods.leaveTeam("Blues Team", 10).accounts({
      teamAccount: team,
    });
    const tx = await ix.rpc().catch(console.error);
    console.log(`Transaction id: ${tx}`);
    const acc = (await ix.pubkeys()).teamAccount;
    const data = await program.account.team.fetch(acc.publicKey);

    console.log(data.teamAutority.toBase58());
  });
  it("Transfer the autority", async () => {
    // Transfer autority test here.
    const [new_captain] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("team_account"),
        anchor.utils.bytes.utf8.encode("Blues Team"),
      ],
      program.programId
    )
    console.log(`User1 key: ${provider.wallet.publicKey.toBase58()}`);
    console.log(`Team acc: ${new_captain.toBase58()}`);
    const ix = program.methods.transferAutority("Blues Team", 10).accounts({
      teamAccount: team,
    });
    const tx = await ix.rpc().catch(console.error);
    console.log(`Transaction id: ${tx}`);
    const acc = (await ix.pubkeys()).teamAccount;
    const data = await program.account.new_captain.fetch(acc.publicKey);

    console.log(data.teamAutority.toBase58());
  });
  it("Transfer the autority", async () => {
    // Transfer autority test here.
    const [new_captain] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("team_account"),
        anchor.utils.bytes.utf8.encode("Blues Team"),
      ],
      program.programId
    )
    console.log(`User1 key: ${provider.wallet.publicKey.toBase58()}`);
    console.log(`Team acc: ${new_captain.toBase58()}`);
    const ix = program.methods.transferAutority("Blues Team", 10).accounts({
      teamAccount: team,
    });
    const tx = await ix.rpc().catch(console.error);
    console.log(`Transaction id: ${tx}`);
    const acc = (await ix.pubkeys()).teamAccount;
    const data = await program.account.new_captain.fetch(acc.publicKey);

    console.log(data.teamAutority.toBase58());
  });
  it("Share the all prizes", async () => {
    // Share prize test here.
    const [new_captain] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("team_account"),
        anchor.utils.bytes.utf8.encode("Blues Team"),
      ],
      program.programId
    )
    console.log(`User1 key: ${provider.wallet.publicKey.toBase58()}`);
    console.log(`Team acc: ${new_captain.toBase58()}`);
    const ix = program.methods.sharePrize("Blues Team", 10).accounts({
      teamAccount: team,
    });
    const tx = await ix.rpc().catch(console.error);
    console.log(`Transaction id: ${tx}`);
    const acc = (await ix.pubkeys()).teamAccount;
    const data = await program.account.team.fetch(acc.publicKey);

    console.log(data.teamAutority.toBase58());
  });
});
