import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaTwitter } from "../target/types/solana_twitter";
import * as assert from "assert";

describe("solana-twitter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaTwitter as Program<SolanaTwitter>;

    it('can send a new tweet', async () => {
      // Call the "SendTweet" instruction.
      const tweet = anchor.web3.Keypair.generate();
      await program.rpc.sendTweet('BTS', 'ARMY loves BTS', {
          accounts: {
              tweet: tweet.publicKey,
              author: program.provider.wallet.publicKey,
              systemProgram: anchor.web3.SystemProgram.programId,
          },
          signers: [tweet],
      });

      // Fetch the account details of the created tweet.
      const tweetAccount = await program.account.tweet.fetch(tweet.publicKey);

      // Ensure it has the right data.
      assert.equal(tweetAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58());
      assert.equal(tweetAccount.topic, 'BTS');
      assert.equal(tweetAccount.content, 'ARMY loves BTS');
      assert.ok(tweetAccount.timestamp);
  });

});
