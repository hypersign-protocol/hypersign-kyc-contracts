import * as jsonld from 'jsonld';
import CONTEXTS from './contexts.json'

const nodeDocumentLoader = jsonld.documentLoaders.node();

const customLoader = async (url, options) => {
    if(url in CONTEXTS) {
      return {
        contextUrl: null, // this is for a context via a link header
        document: CONTEXTS[url], // this is the actual document that was loaded
        documentUrl: url // this is the actual context URL after redirects
      };
    }
    // call the default documentLoader
    return nodeDocumentLoader(url);
  };
  jsonld.documentLoader = customLoader;

async function generateDIDProof(didDoc: any){
    return {
        "@context": [
          "https://www.w3.org/ns/did/v1",
          "https://w3id.org/security/suites/ed25519-2020/v1"
        ],
        "id": "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans",
        "controller": [
          "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans"
        ],
        "alsoKnownAs": [
          "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans"
        ],
        "verificationMethod": [
          {
            "id": "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1",
            "type": "Ed25519VerificationKey2020",
            "controller": "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans",
            "publicKeyMultibase": "z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans"
          }
        ],
        "authentication": [
          "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1"
        ],
        "assertionMethod": [
          "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1"
        ],
        "keyAgreement": [],
        "capabilityInvocation": [
          "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1"
        ],
        "capabilityDelegation": [
          "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1"
        ],
        "service": [],
        "proof": {
          "type": "Ed25519Signature2020",
          "created": "2024-11-07T02:59:29Z",
          "verificationMethod": "did:hid:testnet:z6MkoWeh2x4WbPEPwJMDNVXQKcXy8foAZtWLp8Ex8h57Jans#key-1",
          "proofPurpose": "authentication",
          "challenge": "AgPGfTzahb8Q8zGyr92C6rrsZDTcG13lfzwN1HXlmEc7",
          "domain": "hypersign.id",
          "proofValue": "z2K94UHYbcQouKp5U4uDJv9XGu5pvXi3m5jZQE6MByYJrotSdnkGJ6FBTgR21wnBjc2uf82eeXNTAsJMsdfCYE5KB"
        }
      }
}

export async function  dummyGetDidDocAndProofs(did: string, didDoc: any) {
    
    // const didDocument = didDoc
    // const verificationMethodId = didDocument?.verificationMethod[0]?.id

    // delete didDocument["verificationMethod"][0]["blockchainAccountId"]
    const proofdoc = await generateDIDProof(didDoc)

    const tempproofdoc: any = proofdoc;

    // signature
    const signature = tempproofdoc['proof']['proofValue'];

    // expanded diddocproof
    delete tempproofdoc['proof']['proofValue'];
    
    const did_doc_proof_normal = {
        '@context': tempproofdoc['@context'],
        ...tempproofdoc['proof']
    }
    const did_doc_proof_expanded = await jsonld.expand(did_doc_proof_normal, { documentLoader: customLoader })

    // expanded diddoc
    // eslint-disable-next-line
    delete tempproofdoc['proof']
    const did_doc_normal = tempproofdoc;
    const did_doc_expanded = await jsonld.expand(did_doc_normal, { documentLoader: customLoader })
    // console.log({did_doc_expanded})
    return {
        "counter": 0, did_doc: JSON.stringify(did_doc_expanded), did_doc_proof: JSON.stringify(did_doc_proof_expanded), signature, "kyc_contract_code_id": 156
    }

    // return {
    //     "counter": 0, did_doc: did_doc_expanded, did_doc_proof: did_doc_proof_expanded, signature, "kyc_contract_code_id": 156
    // }
}