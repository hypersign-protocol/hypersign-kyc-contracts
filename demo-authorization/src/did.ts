import { HypersignDID } from "hs-ssi-sdk";

import { createWallet, mnemonic } from './wallet'


let did; 
let didDoc; 

export async function generateDidAndDoc(){

    const wallet = await createWallet(mnemonic)

    const hypersignDID = new HypersignDID({
        offlineSigner: wallet,
        namespace: 'testnet'
    });

    // ed25519 in multibase
    const keys = await hypersignDID.generateKeys({})

    console.log(keys)

    const user = await hypersignDID.generate({ publicKeyMultibase: keys.publicKeyMultibase})
    did = user.id
    didDoc = user;


    


    if(user && user.verificationMethod  && user.verificationMethod[0].id){
        const verificationMethodId =  user.verificationMethod[0].id

        console.log(JSON.stringify(didDoc, null, 2))

        // const resutl1 = await hypersignDID.register({didDocument: didDoc, 
        //     privateKeyMultibase: keys.privateKeyMultibase, 
        //     verificationMethodId})
        // console.log(resutl1)
    
    
        const challenge = "12123123123"
        const domain = "https://hypersign.id"

        const signature = await hypersignDID.sign({
            didDocument: didDoc, 
            privateKeyMultibase: keys.privateKeyMultibase,
            verificationMethodId: verificationMethodId,
            challenge, 
            domain
        })
        console.log(signature)

        /// write in rust code...
        const result = await hypersignDID.verify({didDocument: signature, verificationMethodId, challenge: challenge, domain })
        console.log(result)
    }
    


}







