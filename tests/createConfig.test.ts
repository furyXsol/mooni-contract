import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { Coinkick } from "../target/types/coinkick";
import { assert } from "chai";

import { createConfig } from "./utils/create_config";

describe("CreateConfig", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Coinkick as Program<Coinkick>;
  const payer = anchor.Wallet.local().payer;

  it("Create Config", async () => {
    const { feeReceiptPk, configPk } = await createConfig(
      program,
      payer,
    )

    // Fetch the created account
    const configAccount = await program.account.config.fetch(
      configPk
    );
    assert( configAccount.admin.toBase58() === feeReceiptPk.toBase58() )
    assert( configAccount.feeRecipient.toBase58() === feeReceiptPk.toBase58() )
    assert( configAccount.buyFee === 1000 )
    assert( configAccount.mintFee === 1000 )

  });
});
