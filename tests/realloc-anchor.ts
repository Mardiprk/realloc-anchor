import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Note } from "../target/types/note";

describe("realloc-anchor", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const user = anchor.web3.Keypair.generate(); // new test user
  const program = anchor.workspace.Note as Program<Note>;

  let notePda: anchor.web3.PublicKey;
  let bump: number;

  before(async () => {
    const airdropSig = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);

    // Derive PDA
    [notePda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("note"), user.publicKey.toBuffer()],
      program.programId
    );

    console.log("User:", user.publicKey.toBase58());
    console.log("Note PDA:", notePda.toBase58());
  });

  it("☑️ Initialized!", async () => {
    const tx = await program.methods
      .initialize("Hello")
      .accounts({
        noteAccount: notePda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Initialize TX:", tx);

    const account = await program.account.noteAccount.fetch(notePda);
    console.log("Stored content:", account.content);
  });

  it("☑️ Append!", async () => {
    const tx = await program.methods
      .append(" Solana is very Fast!")
      .accounts({
        noteAccount: notePda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Append TX:", tx);

    const account = await program.account.noteAccount.fetch(notePda);
    console.log("Updated content:", account.content);
  });
});
