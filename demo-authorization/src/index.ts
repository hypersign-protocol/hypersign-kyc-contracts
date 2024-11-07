// import { generateDidAndDoc }  from './did'
// import * as jsonld from 'jsonld';
// import CONTEXTS from './contexts.json'
import { dummyGetDidDocAndProofs } from './expanded'
// generateDidAndDoc()
  
  // // grab the built-in Node.js doc loader
  // const nodeDocumentLoader = jsonld.documentLoaders.node();
  // // or grab the XHR one: jsonld.documentLoaders.xhr()
  
  // // change the default document loader
  // const customLoader = async (url, options) => {
  //   if(url in CONTEXTS) {
  //     return {
  //       contextUrl: null, // this is for a context via a link header
  //       document: CONTEXTS[url], // this is the actual document that was loaded
  //       documentUrl: url // this is the actual context URL after redirects
  //     };
  //   }
  //   // call the default documentLoader
  //   return nodeDocumentLoader(url);
  // };
  // jsonld.documentLoader = customLoader;
  
  
  // // alternatively, pass the custom loader for just a specific call:
  // const doc = {
  //   "@context": [
  //     "https://www.w3.org/ns/did/v1",
  //     "https://w3id.org/security/suites/ed25519-2020/v1"
  //   ],
  //     "type": "Ed25519Signature2020",
  //     "created": "2024-09-01T17:44:11Z",
  //     "verificationMethod": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1",
  //     "proofPurpose": "authentication",
  //     "challenge": "123123",
  //     "domain": "http:adsasd"
  // }

  // const expanded = (async () => {
  //   return jsonld.expand(
  //       doc, {documentLoader: customLoader});
  // })()

  //  expanded.then((e) => {
  //   console.log(e)
  //   console.log(JSON.stringify(e, null, 2))
  // });


  dummyGetDidDocAndProofs("1231", "something").then(x => {
    console.log(JSON.stringify(x));
    // console.log(x);
  })

