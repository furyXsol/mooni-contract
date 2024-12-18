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
import { withdraw } from "./utils/withdraw"

describe("WithdrawToken", () => {
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.Coinkick as Program<Coinkick>
  const payer = anchor.Wallet.local().payer
  const connection: Connection = anchor.getProvider().connection

  it("withdraw", async () => {
    const maxSupply = new BN(1_000_000_000_000); //with_decimal
    const initSupply = new BN(200_000_000_000);  //with_decimal
    const defaultDecimals = 6

    const { feeReceiptPk, configPk } = await createConfig(
      program,
      payer,
    )
    // create Token Account
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

    const buyAmount = new BN("1000000000000")
    await buy(
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
    await withdraw(
      program,
      payer,
      tokenMint,
      bondingCurve,
      associtedBondingCurve,
      associtedUserTokenAccount.address
    )

    const bondingCurveInfo = await connection.getAccountInfo(bondingCurve)
    assert(bondingCurveInfo == null) // empty lamports.
    const associtedBondingInfo = await getAccount(connection, associtedBondingCurve)
    assert(associtedBondingInfo.amount === BigInt(0))

  })
})