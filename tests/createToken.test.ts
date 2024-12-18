import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { Coinkick } from "../target/types/coinkick";
import { assert } from "chai";
import { createConfig } from "./utils/create_config";
import { createToken } from "./utils/create_token";
import { fetchMetadata, mplTokenMetadata, findMetadataPda, fetchDigitalAsset  } from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { PublicKey, publicKey } from "@metaplex-foundation/umi";
import { getMint, getAccount } from "@solana/spl-token"

describe("CreateToken", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Coinkick as Program<Coinkick>;
  const payer = anchor.Wallet.local().payer;
  const connection = anchor.getProvider().connection;
  const umi = createUmi(connection).use(mplTokenMetadata())

  it("Create Token", async () => {
    // create Config Account
    const { feeReceiptPk, configPk } = await createConfig(
      program,
      payer,
    )

    // const[ configPk ] = web3.PublicKey.findProgramAddressSync(
    //   [
    //     Buffer.from("pumpfun_config"),
    //   ],
    //   program.programId
    // );
    const {
      tokenMint,
      bondingCurve,
      associtedBondingCurve,
      metadataPDA,
      associtedFeeTokenAccount
    } = await createToken(
      program,
      payer,
      configPk,
      feeReceiptPk,
      "AAA",
      "A11",
      "URI123456789"
    )

    console.log('payer:', payer.publicKey.toBase58());
    console.log('tokenMint:', tokenMint.toBase58());
    console.log('bondingCurve:', bondingCurve.toBase58());
    console.log('associtedBondingCurve:', associtedBondingCurve.toBase58());
    console.log('metadataAccount:', metadataPDA.toBase58());

    const mint  = await getMint(connection, tokenMint)
    assert( mint.decimals === 9)
    assert( mint.mintAuthority.toBase58() === bondingCurve.toBase58())
    assert( mint.freezeAuthority == null)

    const metadataInfo = await fetchMetadata(umi, publicKey(metadataPDA.toBase58()))
    assert( metadataInfo.name === "AAA")
    assert( metadataInfo.symbol === "A11")
    assert( metadataInfo.uri === "URI123456789")

    const feeTokenAccount = await getAccount(connection, associtedFeeTokenAccount)
    assert( feeTokenAccount.mint.toBase58() === tokenMint.toBase58())
    assert( feeTokenAccount.owner.toBase58() === feeReceiptPk.toBase58())
    assert( feeTokenAccount.amount === BigInt(5_000_000_000_000_000)) // MAX_SUPPLY(500_000_000_000_000_000)'s 1%

    const associtedBondingCurveInfo = await getAccount(connection, associtedBondingCurve)
    assert( associtedBondingCurveInfo.mint.toBase58() === tokenMint.toBase58())
    assert( associtedBondingCurveInfo.owner.toBase58() === bondingCurve.toBase58())
    assert( associtedBondingCurveInfo.amount === BigInt(495_000_000_000_000_000)) // MAX_SUPPLY(500_000_000_000_000_000)'s 99%
  });
});
