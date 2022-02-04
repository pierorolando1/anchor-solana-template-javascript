const anchor = require('@project-serum/anchor');

describe('anchor-template', () => {


  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorTemplate;

  const counterAccount = anchor.web3.Keypair.generate()

  it('Is initialized!', async () => {
    // Add your test here.

    console.log("HOLAAAAAA", program.programId)
    const tx = await program.rpc.initialize({
      accounts: {
        counterAccount: counterAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [counterAccount]
    });
    console.log("Your transaction signature", tx);
  });

  it("Add +1", async () => {

    let account = await program.account.counterAccount.fetch(counterAccount.publicKey);
    console.log('👀Count', account.counter.toString())

    // Call add_gif!
    await program.rpc.add({
      accounts: {
        counterAccount: counterAccount.publicKey,
      },
    });

    account = await program.account.counterAccount.fetch(counterAccount.publicKey);
    console.log('👀Count', account.counter.toString())

  })
});

