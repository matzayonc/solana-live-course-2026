import { address, createClient, createKeyPairFromBytes, createSignerFromKeyPair, estimateAndSetResourceLimitsFactory, estimateResourceLimitsFactory, generateKeyPair, getAddressFromPublicKey, getBase58Encoder, getBase64EncodedWireTransaction, lamports, setTransactionMessageLifetimeUsingBlockhash, signTransactionMessageWithSigners, writeKeyPair } from "@solana/kit";
import phantomKey from './key.json' with {type: 'json'}
import { solanaMainnetRpc } from "@solana/kit-plugin-rpc";
import { signerFromFile } from "@solana/kit-plugin-signer";
import { getTransferSolInstruction } from "@solana-program/system";
import { transferInstructionData } from "@solana/spl-token";
import { program } from "@anchor-lang/core/dist/cjs/native/system.js";


const bytes = getBase58Encoder()
.encode(phantomKey[0])
const keypair = await createKeyPairFromBytes(bytes, true)
// await writeKeyPair(keypair, 'signer.json')

const client = await createClient()
.use(await signerFromFile('signer.json'))
.use(solanaMainnetRpc({rpcUrl: 'https://api.mainnet-beta.solana.com'}))


const balance = await client.rpc
.getBalance(client.identity.address)
.send()

console.log(`${client.identity.address}: ${balance.value}`)

// const ix = getTransferSolInstruction({
//     source: client.identity,
//     destination: address('FAGdMLB8mRmpKgmkLYfdWHrbaNDM2449jkStELHh6YYx'),
//     amount: lamports(1000n)
// })



console.log(`${client.identity.address}: ${balance.value}`)


// console.log(simulation.value)

const viewers = [
`FAGdMLB8mRmpKgmkLYfdWHrbaNDM2449jkStELHh6YYx`,
`4eyiqa3QrUSGDnz3kC35VNfGMTVsD4rC9EBzuyXaN7kA`,
`3q7yUYKY2fsmooreAYYQEKHpkeMeGec4nnWDDP28aS6v`,
`4F7a2psqbL5Uod72SePZpvpyFo9vTCJsPAEvipKkAWbx`,
`6sM6AgBhPXHwhACVKvRU9QKUHnHteovvKxhb3iLZtdrG`,
`48w2YGcPy3Y9vUoL3Tjcq2CZ6svdUfDeBkUyDut8duJH`,
`ELZNtDSijZkGPFuYiiDNmokkgENFrjnyRyUajXQKJoZy`,
`5AAWyae4Xqqs5ShofYdPuonhtZFBzLycNRLMgVSrBoQ3`,
`HgATKPyRJXqfqitFeaxK9JLkbjQqtdrRMvN1JyqyrheC`,
`3bksD1YN7DZ2aq8pcgrUC7KBFWJR3ULQSViy2XnQAvT8`,
`DgefiQmbmGkRPUT498BAfv6htUQVNSRA5nkKAzBzFj8v`,
`3uD7udthcqJVfAgZxAMwtyrf4yVmtbvnHANHWVNPMeGd`,
'7MMLqVx3gmBgT9Du3AxS2zXbCsrptAtntUB2cNbZhT7F',
'7v7sTHaj2mjoU61nDJH22YiHpKbR74J1VhxgPeJW7Nrz',
'Fya53mMXWXFd72xKEB4CeXCPHnXwgmcgDP7asVxr88R5'
]

const instructions = viewers.map(v => 
 getTransferSolInstruction({
    source: client.identity,
    destination: address(v),
    amount: lamports(20000000n)
}))

// transferInstructionData()
// `program.instruction.transfer`


const estimateAndSetResourceLimits = estimateAndSetResourceLimitsFactory(
    estimateResourceLimitsFactory({ rpc: client.rpc })
)

const { value: latestBlockhash } = await client.rpc.getLatestBlockhash().send()

const message = await estimateAndSetResourceLimits(
    setTransactionMessageLifetimeUsingBlockhash(
        latestBlockhash,
        await client.planTransaction(instructions)
    )
)
const transaction = await signTransactionMessageWithSigners(message)

const simulation = await client.rpc.simulateTransaction(
    getBase64EncodedWireTransaction(transaction), {
    encoding: 'base64',
    replaceRecentBlockhash: true
}).send()


const sig = await client.rpc.sendTransaction(
    getBase64EncodedWireTransaction(transaction), {
    encoding: 'base64'
}).send()

console.log(`Signature: ${sig}`)
