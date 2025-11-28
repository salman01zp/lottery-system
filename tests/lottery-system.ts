import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LotterySystem } from "../target/types/lottery_system";

describe("lottery-system", () => {
  // Configure the client to use the local cluster.
  const provider  = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet;


  const program = anchor.workspace.lotterySystem as Program<LotterySystem>;

  it("should initialize config!", async () => {
      const initConfigTx = await program.methods.initializeConfig(
        new anchor.BN(0),
        new anchor.BN(1864310488),
        new anchor.BN(10000) // 10000 lamports per ticket
      ).instruction();


      const blockHashWithContext = await provider.connection.getLatestBlockhashAndContext();
      const tx = new anchor.web3.Transaction({
        feePayer : wallet.publicKey,
        blockhash : blockHashWithContext.value.blockhash,
        lastValidBlockHeight: blockHashWithContext.value.lastValidBlockHeight
 
      }).add(initConfigTx)


      const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [wallet.payer]);

      console.log("Transaction signature  ", signature);
  });
}); 
