import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TestDapp } from '../target/types/test_dapp';
const { SystemProgram } = anchor.web3;

describe('test-dapp', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const provider = anchor.Provider.env();
  const program = anchor.workspace.TestDapp as Program<TestDapp>;

  it('Is initialized!', async () => {
    // Add your test here
    const userAccount = anchor.web3.Keypair.generate();
    let authority = anchor.web3.Keypair.generate();
    console.log(userAccount.publicKey, "==> userAccount");
    console.log(provider.wallet.publicKey, "==> userAccount");
    console.log(SystemProgram.programId, "==> userAccount");

    const tx = await program.rpc.initializeUser(
      new anchor.BN(3), 
      new anchor.BN(3),
      new anchor.BN(3),
      new anchor.BN(3),
      new anchor.BN(3),
      new anchor.BN(0),
      {
        accounts:{
        userAccount:userAccount.publicKey,
        authority:provider.wallet.publicKey,
        systemProgram:SystemProgram.programId,
      },
      signers:[authority]
    });
    console.log("Your transaction signature", tx);
  });
});
