// Import necessary libraries and components
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
const { SystemProgram, PublicKey, Keypair, LAMPORTS_PER_SOL } = anchor.web3;
import { expect } from "chai";
import { Lottery } from "../target/types/lottery";

// Define the sleep function to add a delay
const sleep = async (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// Define the airdropSOL function to airdrop SOL to the specified account
async function airdropSOL(program, accountPublicKey, solAmount) {
  const lamports = LAMPORTS_PER_SOL * solAmount;
  await program.provider.connection.requestAirdrop(accountPublicKey, lamports);
}

// Main test suite for the Lottery program
describe("Lottery", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Lottery as Program<Lottery>;
  const provider = anchor.AnchorProvider.env();

  // Define variables to be used throughout the tests
  let authorityAccount, vaultAccount, authority, vault, user, userAccount;

  let lotteryId, lotteryStateAddress;

  // Set up the test environment before running the tests
  before(async () => {
    // Generate keypairs for the authority, vault, and user accounts
    authority = Keypair.generate();
    authorityAccount = authority.publicKey;

    vault = Keypair.generate();
    vaultAccount = vault.publicKey;

    user = Keypair.generate();
    userAccount = user.publicKey;

    // Airdrop 10 SOL to the authority, vault, user, and program accounts
    await airdropSOL(program, authorityAccount, 10);
    await airdropSOL(program, vaultAccount, 10);
    await airdropSOL(program, user.publicKey, 10);
    await airdropSOL(program, program.programId, 10);

    console.log(
      `Authority account: ${authorityAccount.toString()} (https://explorer.solana.com/address/${authorityAccount.toString()}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899)`
    );

    console.log(
      `Vault account: ${vaultAccount.toString()} (https://explorer.solana.com/address/${vaultAccount.toString()}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899)`
    );

    console.log(
      `User account: ${user.publicKey.toString()} (https://explorer.solana.com/address/${user.publicKey.toString()}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899)`
    );

    // Wait for 3 seconds for the airdrops to complete
    await sleep(3000);
  });

  // Test suite for initializing the lottery state
  describe("Initialize Lottery State", () => {
    it("should initialize a new lottery state", async () => {
      // Generate a random lottery ID and define other parameters for the lottery state
      lotteryId = new anchor.BN(
        Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
      );
      const ticketPrice = new anchor.BN(Math.round(0.01 * LAMPORTS_PER_SOL));
      const startTime = new anchor.BN(
        Math.floor(new Date().getTime() / 1000 + 60)
      );
      const endTime = new anchor.BN(startTime.toNumber() + 60 * 60);
      const maxTickets = new anchor.BN(1000);

      // Create the lottery state account with PDA
      const lotteryStateSeeds = [
        Buffer.from("lottery_state"),
        new anchor.BN(lotteryId).toArrayLike(Buffer, "le", 8),
      ];

      let [lotteryStateAddress, lotteryStateBump] =
        await PublicKey.findProgramAddressSync(
          lotteryStateSeeds,
          program.programId
        );

      // Create the lottery vault account with PDA
      const lotteryVaultSeeds = [
        Buffer.from("lottery_vault"),
        new anchor.BN(lotteryId).toArrayLike(Buffer, "le", 8),
      ];

      let [lotteryVaultAddress, lotteryVaultBump] =
        await PublicKey.findProgramAddressSync(
          lotteryVaultSeeds,
          program.programId
        );

      // Initialize the lottery state using the `initializeLotteryState` method
      try {
        await program.methods
          .initializeLotteryState(
            lotteryId,
            startTime,
            endTime,
            maxTickets,
            ticketPrice
          )
          .accounts({
            authority: authority.publicKey,
            lotteryState: lotteryStateAddress,
            lotteryVault: lotteryVaultAddress,
            systemProgram: SystemProgram.programId,
          })
          .signers([authority])
          .rpc();
      } catch (error) {
        console.error("Error in initializeLotteryState:", error);
      }

      // Fetch the created lottery state account
      const lotteryState = await program.account.lotteryState.fetch(
        lotteryStateAddress
      );

      // Assert that the lottery state account was initialized with the correct data
      expect(lotteryState.ticketPrice.eq(ticketPrice)).to.be.true;
      expect(lotteryState.startTime.eq(startTime)).to.be.true;
      expect(lotteryState.endTime.eq(endTime)).to.be.true;
      expect(lotteryState.maxTickets.eq(maxTickets)).to.be.true;

      console.log(`Lottery ID: ${lotteryId.toString()}`);

      console.log(
        `Lottery state address: ${lotteryStateAddress.toString()} (https://explorer.solana.com/address/${lotteryStateAddress.toString()}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899)`
      );

      lotteryStateAddress = lotteryStateAddress;
    });
  });

  // Test suite for buying lottery tickets
  describe("Buy Tickets", () => {
    it("should allow user to buy 1 SOL worth of lottery tickets", async () => {
      // Set the desired amount of SOL to buy in tickets
      const solAmount = 1;
      const ticketPrice = new anchor.BN(Math.round(0.05 * LAMPORTS_PER_SOL));
      const ticketCount = new anchor.BN(solAmount * 1); // 1 SOL = 80 tickets at 0.05 SOL per ticket

      console.log("ticketPrice: ", ticketPrice.toString());
      console.log("Desired ticket count:", ticketCount.toString());

      // Calculate the deposit amount
      const depositAmount = ticketCount.div(ticketPrice);

      console.log("depositAmount:", depositAmount.toString());

      // Create the lottery state account with PDA
      const lotteryStateSeeds = [
        Buffer.from("lottery_state"),
        new anchor.BN(lotteryId).toArrayLike(Buffer, "le", 8),
      ];

      let [lotteryStateAddress, lotteryStateBump] =
        await PublicKey.findProgramAddressSync(
          lotteryStateSeeds,
          program.programId
        );

      // Calculate the PDA for the user's ticket data
      const ticketDataSeeds = [
        Buffer.from("user_ticket_data"),
        new anchor.BN(lotteryId).toArrayLike(Buffer, "le", 8),
        userAccount.toBuffer(),
      ];
      const [ticketDataAddress, ticketDataBump] =
        await PublicKey.findProgramAddressSync(
          ticketDataSeeds,
          program.programId
        );

      // Create the lottery vault account with PDA
      const lotteryVaultSeeds = [
        Buffer.from("lottery_vault"),
        new anchor.BN(lotteryId).toArrayLike(Buffer, "le", 8),
      ];

      let [lotteryVaultAddress, lotteryVaultBump] =
        await PublicKey.findProgramAddressSync(
          lotteryVaultSeeds,
          program.programId
        );

      // Buy tickets using the `buyTickets` method
      try {
        await program.methods
          .buyTickets(ticketCount)
          .accounts({
            lotteryState: lotteryStateAddress,
            lotteryVault: lotteryVaultAddress,
            ticketData: ticketDataAddress,
            user: userAccount,
            systemProgram: SystemProgram.programId,
          })
          .signers([user])
          .rpc();
      } catch (error) {
        console.error("Error in buyTickets:", error);
      }

      // Fetch the updated user ticket data account
      const userTicketData = await program.account.ticketData.fetch(
        ticketDataAddress
      );

      console.log("User ticket data:", userTicketData);

      // Fetch the updated lottery state account
      const updatedLotteryState = await program.account.lotteryState.fetch(
        lotteryStateAddress
      );

      console.log("Updated lottery state:", updatedLotteryState);

      // Add assertions to verify if the user has successfully bought the tickets
      expect(userTicketData.ticketCount.eq(ticketCount)).to.be.true;

      // Add assertions to verify if the lottery state has been updated correctly
      expect(
        updatedLotteryState.ticketCount.eq(
          updatedLotteryState.ticketCount.add(ticketCount)
        )
      ).to.be.true;
    });
  });
});
