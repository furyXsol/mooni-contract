import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor"
import { Connection } from "@solana/web3.js";
import { Coinkick } from "../target/types/coinkick"
import { createConfig } from "./utils/create_config"
import { createToken } from "./utils/create_token"
import { buy } from "./utils/buy"
import { getAccount, getOrCreateAssociatedTokenAccount } from "@solana/spl-token"
import { assert } from "chai"
import { getTransactionLogs } from "./utils/utils"

describe("BuyToken", () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.Coinkick as Program<Coinkick>
  const payer = anchor.Wallet.local().payer
  const connection: Connection = anchor.getProvider().connection

  it("buy", async () => {

    const { feeReceiptPk, configPk } = await createConfig(
      program,
      payer,
    )
    // create Token
    const {
      tokenMint,
      bondingCurve,
      associtedBondingCurve,
    } = await createToken(
      program,
      payer,
      configPk,
      feeReceiptPk,
      "TOKEN_NAME",
      "TSYM",
      "ipfs://TOKEN_URI"
    )

    const associtedUserTokenAccount = await getOrCreateAssociatedTokenAccount(connection, payer, tokenMint, payer.publicKey)
    const oldAmount = associtedUserTokenAccount.amount
    const oldBondingCurveInfo = await connection.getAccountInfo(bondingCurve)
    const oldSolAmount = oldBondingCurveInfo.lamports

    const buyAmount = new BN("1000000000000") //1000 token
    const sig = await buy(
      program,
      payer,
      buyAmount, //BUY AMOUNT 1000 token (decimal is 9)
      new BN("10000000"), //0.01SOL
      tokenMint,
      configPk,
      bondingCurve,
      associtedBondingCurve,
      associtedUserTokenAccount.address,
      feeReceiptPk,
    )
    await connection.confirmTransaction(sig, "finalized");

    await getTransactionLogs(connection, sig)
    const userTokenAccountInfo = await getAccount(connection, associtedUserTokenAccount.address)
    const newAmount = userTokenAccountInfo.amount
    // check to increased buy amount
    assert( BigInt(newAmount - oldAmount) === BigInt(buyAmount.toNumber()))
    // check to increased sol amount
    const newBondingCurveInfo = await connection.getAccountInfo(bondingCurve)
    const newSolAmount = newBondingCurveInfo.lamports
    //0.008020033 SOL for 1000 token
    console.log("oldSolAmount:", oldSolAmount)
    console.log("newSolAmount:", newSolAmount)
    assert(newSolAmount - oldSolAmount === 5256381)
  })
})