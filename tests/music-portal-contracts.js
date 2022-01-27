const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

describe('music-portal-contracts', () => {
  // Create and set the provider
  // We set it before but we needed to update it, so that it can communicate with our frontend
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  it('Is working!', async () => {
    // Add your test here.
    const program = anchor.workspace.MusicPortalContracts;

    // Create an account keypair for our program to use.
    const baseAccount = anchor.web3.Keypair.generate();

    // Call initialize, pass it the params it needs
    let tx = await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });

    console.log('Your transaction signature', tx);

    // // Fetch data from the account.
    // let account = await program.account.baseAccount.fetch(
    //   baseAccount.publicKey
    // );
    // console.log('👀 Song Count', account.totalSongs.toString());

    // await program.rpc.addSong(
    //   '<iframe style="border-radius:12px" src="https://open.spotify.com/embed/track/697HW34FLNURvRAbSieICv?utm_source=generator" width="100%" height="80" frameBorder="0" allowfullscreen="" allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"></iframe>',
    //   {
    //     accounts: {
    //       baseAccount: baseAccount.publicKey,
    //       user: provider.wallet.publicKey,
    //     },
    //   }
    // );
    // account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    // console.log('👀 Song Count', account.totalSongs.toString());
    // console.log('👀 Song List', account.songList);
  });
});
