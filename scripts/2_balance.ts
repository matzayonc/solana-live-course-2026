import { address, createClient } from '@solana/kit';
import { solanaMainnetRpc } from '@solana/kit-plugin-rpc';
import { signerFromFile } from '@solana/kit-plugin-signer';

const USDC = address('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');

const client = await createClient()
  .use(signerFromFile('./keypair.json'))
  .use(solanaMainnetRpc({ rpcUrl: 'https://api.mainnet-beta.solana.com' }));

const owner = client.identity.address;
const { value } = await client.rpc
  .getTokenAccountsByOwner(owner, { mint: USDC }, { encoding: 'jsonParsed' })
  .send();

const balance = value[0]?.account.data.parsed.info.tokenAmount.uiAmountString ?? '0';
console.log(`wallet  ${owner}`);
console.log(`USDC    ${balance}`);
