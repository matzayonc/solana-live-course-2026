import { createKeyPairSignerFromBytes, createClient, lamports } from '@solana/kit';
import { solanaDevnetRpc } from '@solana/kit-plugin-rpc';
import { signerFromFile } from '@solana/kit-plugin-signer';
import { getTransferSolInstruction } from '@solana-program/system';
import { readFile } from 'node:fs/promises';

// Devnet, because the sender needs SOL and only devnet gives it away for free.
const client = await createClient()
  .use(signerFromFile('./keypair.json'))
  .use(solanaDevnetRpc());

const recipient = await createKeyPairSignerFromBytes(
  new Uint8Array(JSON.parse(await readFile('./another-keypair.json', 'utf8'))),
);

await client.airdrop(client.identity.address, lamports(1_000_000_000n));

const signature = await client.sendTransaction([
  getTransferSolInstruction({
    source: client.identity,
    destination: recipient.address,
    amount: lamports(100_000_000n),
  }),
]);

console.log(`from   ${client.identity.address}`);
console.log(`to     ${recipient.address}`);
console.log(`sent   0.1 SOL`);
console.log(`tx     ${signature}`);
