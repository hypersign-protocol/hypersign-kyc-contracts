import { HypersignDID, SupportedPurpose } from "hs-ssi-sdk";

import { createWallet, mnemonic, hidNodeEp } from './wallet'


let did; 
let didDoc; 

export async function generateDidAndDoc(){

    // Create a wallet
    const offlineSigner = await createWallet(mnemonic)

    // Instantiate the SDK
    const hypersignDID = new HypersignDID({
        offlineSigner,
        nodeRestEndpoint: hidNodeEp.rest,
        nodeRpcEndpoint: hidNodeEp.rpc,
        namespace: hidNodeEp.namespace,
    });

    // Initialize the SDK
    await hypersignDID.init()

    // Generate keys: ed25519 in multibase
    const keys = await hypersignDID.generateKeys({})

    console.log(keys)

    // Generate DIDs
    const user = await hypersignDID.generate({ publicKeyMultibase: keys.publicKeyMultibase})
    did = user.id
    didDoc = user;

    console.log("==== DID DOC ==== ")
    console.log(JSON.stringify(didDoc, null, 2))

    if(user && user.verificationMethod  && user.verificationMethod[0].id){
        const verificationMethodId =  user.verificationMethod[0].id

        // console.log(JSON.stringify(didDoc, null, 2))

        // // Register DID on the blockchian [optional]
        // const resutl1 = await hypersignDID.register({didDocument: didDoc, 
        //     privateKeyMultibase: keys.privateKeyMultibase, 
        //     verificationMethodId})
        
        // console.log("=== register ===")
        // console.log(resutl1)
    
    
        const challenge = "123123"
        const domain = "http:adsasd"

        // Sign a DID with authentication proof purpose
        const signature = await hypersignDID.sign({
            purpose: SupportedPurpose.authentication,
            didDocument: didDoc, 
            privateKeyMultibase: keys.privateKeyMultibase,
            verificationMethodId: verificationMethodId,
            challenge, // 
            domain, // 
        })
        console.log("=== signature ===")
        console.log(JSON.stringify(signature, null, 2))

        // Verify a DID with authentication proof purpose
        const result = await hypersignDID.verify({didDocument: signature, verificationMethodId,  challenge, domain})
        console.log(JSON.stringify(result))
    }
    


}







