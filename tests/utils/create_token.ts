import { Program, web3 } from "@coral-xyz/anchor";
import { createMint, getAssociatedTokenAddressSync, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID,
  createAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction
 } from "@solana/spl-token";
import { MPL_TOKEN_METADATA_PROGRAM_ID  } from "@metaplex-foundation/mpl-token-metadata";
import { PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import { Coinkick } from "../../target/types/coinkick";

export const createToken = async (
  program: Program<Coinkick>,
  payer: web3.Keypair,
  config: PublicKey,
  feeReceiptPk: PublicKey,
  tokenName: string,
  tokenSymbol: string,
  tokenUri: string,
):Promise<{
  tokenMint: PublicKey,
  bondingCurve: PublicKey,
  associtedBondingCurve: PublicKey,
  metadataPDA: PublicKey,
  associtedFeeTokenAccount: PublicKey,
}> => {
  const tokenMintKP = web3.Keypair.generate();

  const [ bondingCurve ] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("pumpfun_bonding_curve"),
      tokenMintKP.publicKey.toBuffer()
    ],
    program.programId
  )

  const associtedBondingCurve = getAssociatedTokenAddressSync(
    tokenMintKP.publicKey,
    bondingCurve,
    true
  )

  const metadata_program_id = new web3.PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID)
  const [ metadataPDA ] = web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from('metadata'),
        metadata_program_id.toBuffer(),
        tokenMintKP.publicKey.toBuffer(),
    ],
    metadata_program_id
  )

  
  const associtedFeeTokenAccount = await getAssociatedTokenAddressSync(tokenMintKP.publicKey, feeReceiptPk)
  console.log('--------associted_user_token_account:', associtedFeeTokenAccount.toBase58())
  // const associtedFeeTokenAccount = await createAssociatedTokenAccount(
  //   program.provider.connection,
  //   payer,
  //   tokenMintKP.publicKey,
  //   payer.publicKey,  //fee_receipt
  //   {
  //     commitment:"confirmed"
  //   },
  //   TOKEN_PROGRAM_ID,
  //   ASSOCIATED_TOKEN_PROGRAM_ID
  // )


  console.log('-------createMemeToken')
  const tx = await program.methods.createToken(
    {
      name: Buffer.from(tokenName),
      symbol: Buffer.from(tokenSymbol),
      uri: Buffer.from(tokenUri),
    }
  ).accounts({
    payer: payer.publicKey,
    tokenMint: tokenMintKP.publicKey,
    bondingCurve,
    associtedBondingCurve,
    feeRecipient: feeReceiptPk,
    associtedFeeTokenAccount,
    config,
    metadata: metadataPDA,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    tokenProgram: TOKEN_PROGRAM_ID,
    tokenMetadataProgram: metadata_program_id,
    rent: web3.SYSVAR_RENT_PUBKEY,
    systemProgram: web3.SystemProgram.programId,
  }).signers([payer, tokenMintKP]).rpc()

  return {
    tokenMint: tokenMintKP.publicKey,
    bondingCurve,
    associtedBondingCurve,
    metadataPDA,
    associtedFeeTokenAccount
  }
}