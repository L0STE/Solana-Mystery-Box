import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaMysteryBox } from "../target/types/solana_mystery_box";
import { Connection, clusterApiUrl, ConfirmOptions, PublicKey, SystemProgram, LAMPORTS_PER_SOL} from "@solana/web3.js";

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
    box,
    odd1,
    new anchor.BN(amount1),
    odd2,
    new anchor.BN(amount2),
    odd3,
    new anchor.BN(amount3),
    odd4,
    new anchor.BN(amount4),
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

});
