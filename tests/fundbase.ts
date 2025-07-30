import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fundbase } from "../target/types/fundbase";
import { expect } from "chai";

describe("fundbase", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Fundbase as Program<Fundbase>;
  const user = provider.wallet;

  let campaignPda: anchor.web3.PublicKey;
  let campaignBump: number;

  const title = "Test Campaign";
  const description = "This is a test campaign to raise funds.";
  const target = new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL);
  const deadline = new anchor.BN(Math.floor(Date.now() / 1000) + 3600); // 1 hour
  const image = "https://example.com/image.jpg";

  before(async () => {
    [campaignPda, campaignBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("campaign"), user.publicKey.toBuffer()],
      program.programId
    );

    // const sig = await provider.connection.requestAirdrop(
    //   user.publicKey,
    //   5 * anchor.web3.LAMPORTS_PER_SOL
    // );
    // await provider.connection.confirmTransaction(sig);

    // console.log("\x1b[36m%s\x1b[0m", `✅ Airdropped 5 SOL to: ${user.publicKey.toBase58()}`);
  });

  it("Creates a campaign", async () => {
    await program.methods
      .createCampaign(title, description, target, deadline, image)
      .accounts({
        campaign: campaignPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const campaign = await program.account.campaign.fetch(campaignPda);

    console.log("\x1b[32m%s\x1b[0m", `✅ Created campaign: ${campaignPda.toBase58()}`);
    console.log("📦 Campaign Details:", {
      title: campaign.title,
      owner: campaign.owner.toBase58(),
      target: campaign.target.toNumber() / anchor.web3.LAMPORTS_PER_SOL + " SOL",
    });

    expect(campaign.owner.toBase58()).to.equal(user.publicKey.toBase58());
    expect(campaign.title).to.equal(title);
    expect(campaign.description).to.equal(description);
  });

  it("Donates to the campaign", async () => {
    const amount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL / 2);

    await program.methods
      .donate(amount)
      .accounts({
        campaign: campaignPda,
        donator: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const campaign = await program.account.campaign.fetch(campaignPda);

    console.log("\x1b[33m%s\x1b[0m", `💰 Donated 0.5 SOL to campaign: ${campaignPda.toBase58()}`);
    console.log(`🧍 Total Donors: ${campaign.donators.length}`);
    console.log(`📈 Amount Collected: ${campaign.amountCollected.toNumber() / anchor.web3.LAMPORTS_PER_SOL} SOL`);

    expect(campaign.amountCollected.toNumber()).to.be.gte(amount.toNumber());
    expect(campaign.donators.length).to.equal(1);
  });

  it("Withdraws from the campaign", async () => {
    const withdrawAmount = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL / 4);

    await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        campaign: campaignPda,
        owner: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const campaign = await program.account.campaign.fetch(campaignPda);

    console.log("\x1b[35m%s\x1b[0m", `💸 Withdrawn 0.25 SOL from campaign to owner: ${user.publicKey.toBase58()}`);
    console.log(`💼 Remaining balance in campaign account: ~ may vary`);

    expect(campaign.amountCollected).to.not.equal(null);
  });
});
