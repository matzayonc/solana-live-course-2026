import { generateKeyPairSigner, writeKeyPairSigner } from '@solana/kit';

for (const name of ['keypair', 'another-keypair']) {
  // `true` makes the key extractable — otherwise its bytes can never leave memory.
  const signer = await generateKeyPairSigner(true);
  await writeKeyPairSigner(signer, `./${name}.json`);

  console.log(`${name.padEnd(16)} ${signer.address}`);
}
