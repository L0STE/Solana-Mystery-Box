import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaMysteryBox } from "../target/types/solana_mystery_box";
import { Connection, clusterApiUrl, ConfirmOptions, PublicKey, SystemProgram, LAMPORTS_PER_SOL, Transaction} from "@solana/web3.js";
import { NATIVE_MINT, TOKEN_PROGRAM_ID, createSyncNativeInstruction, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

describe("solana-mystery-box", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider(); 
  anchor.AnchorProvider.env().opts.commitment = "finalized";
  const program = anchor.workspace.SolanaMysteryBox as Program<SolanaMysteryBox>;
  const connection = new Connection("https://api.devnet.solana.com");

  const keypair = anchor.web3.Keypair.generate();
  const roller = anchor.web3.Keypair.generate();

  ///////////////////////////////////////////////////////////////////////////////////////////

  //INPUT
  const box = "Free Sol";
  let odd1 = 0.4;
  let amount1 = 0.1*LAMPORTS_PER_SOL; 
  let odd2 = 0.3;
  let amount2 = 0.2*LAMPORTS_PER_SOL;
  let odd3 = 0.2;
  let amount3 = 0.3*LAMPORTS_PER_SOL;
  let odd4 = 0.1;
  let amount4 = 0.4*LAMPORTS_PER_SOL;

  ///////////////////////////////////////////////////////////////////////////////////////////
  
  //Funding the wallet
  it("Starts an airdrop and confirms it", async () => {
    const signature = await provider.connection.requestAirdrop(keypair.publicKey, 100 * LAMPORTS_PER_SOL);
    const latestBlockhash = await connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
    {
        signature,
        ...latestBlockhash,
    },
  "finalized"
    );  
  })

  it("Starts an airdrop and confirms it", async () => {
    const signature = await provider.connection.requestAirdrop(roller.publicKey, 100 * LAMPORTS_PER_SOL);
    const latestBlockhash = await connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
    {
        signature,
        ...latestBlockhash,
    },
  "finalized"
    );  
  })

///////////////////////////////////////////////////////////////////////////////////////////

const boxState = anchor.web3.Keypair.generate();
// Deriving the PDAs
const boxSeeds = [Buffer.from("box"), boxState.publicKey.toBuffer()];
const [boxKey, _bump] = PublicKey.findProgramAddressSync(boxSeeds, program.programId);

it("Initialising the BOX", async () => {
  try {
    const tx = await program.methods
    .initizializeBox(
      odd1,
      odd2,
      odd3,
      odd4,
      new anchor.BN(amount1),
      new anchor.BN(amount2),
      new anchor.BN(amount3),
      new anchor.BN(amount4),
      box,
      _bump
    )
    .accounts({
      boxState: boxState.publicKey,
      boxVault: boxKey,
      owner: keypair.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([
      boxState,
      keypair
    ]).rpc();
  } catch (err) {
    console.log(err);
  }
})

///////////////////////////////////////////////////////////////////////////////////////////

let amount = 10*LAMPORTS_PER_SOL;

it("Initialising both the ATA and fund Sol transfer", async () => {
  try {
    const ownerAta = await getOrCreateAssociatedTokenAccount(provider.connection, keypair, NATIVE_MINT, keypair.publicKey);
    console.log("\nOwnerATA created Succesfully!");    
    const boxAta = await getOrCreateAssociatedTokenAccount(provider.connection, keypair, NATIVE_MINT, boxKey, true);
    console.log("\nBoxAta created Succesfully!");  

    let transferTx = new Transaction().add(
      // trasnfer SOL
      SystemProgram.transfer({
        fromPubkey: keypair.publicKey,
        toPubkey: ownerAta.address,
        lamports: amount,
      }),
      // sync wrapped SOL balance
      createSyncNativeInstruction(ownerAta.address)
    );

    let depositTx = await program.methods
      .boxDeposit(
        new anchor.BN(amount),
      )
      .accounts({
        boxState: boxState.publicKey,
        boxVault: boxKey,
        boxAta: boxAta.address,
        owner: keypair.publicKey,
        ownerAta: ownerAta.address,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([
        keypair
      ]).rpc();

    console.log("\nDeposit Succesfull!");

    let withdrawTx = await program.methods
      .boxWithdraw()
      .accounts({
        boxState: boxState.publicKey,
        boxVault: boxKey,
        boxAta: boxAta.address,
        owner: keypair.publicKey,
        ownerAta: ownerAta.address,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([
        keypair
      ]).rpc();

    console.log("\nWithdraw Succesfull!");

    depositTx = await program.methods
      .boxDeposit(
        new anchor.BN(amount),
      )
      .accounts({
        boxState: boxState.publicKey,
        boxVault: boxKey,
        boxAta: boxAta.address,
        owner: keypair.publicKey,
        ownerAta: ownerAta.address,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([
        keypair
      ]).rpc();

    let accountBalance = await provider.connection.getBalance(boxAta.address);
    console.log("\nDeposit Succesfull! New Vault Balance: ", accountBalance/LAMPORTS_PER_SOL);
    let boxAccount = await program.account.boxState.fetch(boxState.publicKey);
    console.log(`Your Balance: ${boxAccount.bank}`);

  } catch (err) {
    console.log(err);
  }
})

///////////////////////////////////////////////////////////////////////////////////////////



});
