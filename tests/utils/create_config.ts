
import { Program, BN } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
export const createConfig = async (
    program: Program,
    payer: Keypair,
): Promise<{
  feeReceiptPk: PublicKey,
  configPk: PublicKey,
}> => {
    // const feeReceiptKp = Keypair.generate();
    const authorityPk = payer.publicKey; //admin
    const feeRecipient = new PublicKey("4cASLHHXtomyWCD6muAGeEF17dran74LnXx78LhVfwz3");
    const liquidityPk = new PublicKey("5KjtLUmuFenkLK8ZRsqcC2b7zoLaG1T4rGUeFgkqTGcL");
    const[ configPk ] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("pumpfun_config"),
      ],
      program.programId
    );
    const tx = await program.methods.createConfig({
      admin: authorityPk,
      feeRecipient,
      liquidity: liquidityPk,
      buyFee: 100, //1%
      mintFee: 100, // 1%
    }).accounts({
      payer: authorityPk,
      config: configPk,
      systemProgram: SystemProgram.programId,
    }).signers([])
    .rpc();

    console.log("Your transaction signature:", tx);
    console.log("configPk:", configPk.toBase58())
    return {
      feeReceiptPk: authorityPk,
      configPk,
    }
}