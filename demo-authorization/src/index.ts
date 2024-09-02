import { generateDidAndDoc }  from './did'
import * as jsonld from 'jsonld';

// generateDidAndDoc()

const CONTEXTS = {
    "https://www.w3.org/ns/did/v1": {
        "@context": {            "@protected": true,
            "id": "@id",
            "type": "@type",
        
            "alsoKnownAs": {
              "@id": "https://www.w3.org/ns/activitystreams#alsoKnownAs",
              "@type": "@id"
            },
            "assertionMethod": {
              "@id": "https://w3id.org/security#assertionMethod",
              "@type": "@id",
              "@container": "@set"
            },
            "authentication": {
              "@id": "https://w3id.org/security#authenticationMethod",
              "@type": "@id",
              "@container": "@set"
            },
            "capabilityDelegation": {
              "@id": "https://w3id.org/security#capabilityDelegationMethod",
              "@type": "@id",
              "@container": "@set"
            },
            "capabilityInvocation": {
              "@id": "https://w3id.org/security#capabilityInvocationMethod",
              "@type": "@id",
              "@container": "@set"
            },
            "controller": {
              "@id": "https://w3id.org/security#controller",
              "@type": "@id"
            },
            "keyAgreement": {
              "@id": "https://w3id.org/security#keyAgreementMethod",
              "@type": "@id",
              "@container": "@set"
            },
            "service": {
              "@id": "https://www.w3.org/ns/did#service",
              "@type": "@id",
              "@context": {
                "@protected": true,
                "id": "@id",
                "type": "@type",
                "serviceEndpoint": {
                  "@id": "https://www.w3.org/ns/did#serviceEndpoint",
                  "@type": "@id"
                }
              }
            },
            "verificationMethod": {
              "@id": "https://w3id.org/security#verificationMethod",
              "@type": "@id"
            }
        }

    },
    "https://w3id.org/security/suites/ed25519-2020/v1": {
        "@context": {
        "id": "@id",
        "type": "@type",
        "@protected": true,
        "proof": {
          "@id": "https://w3id.org/security#proof",
          "@type": "@id",
          "@container": "@graph"
        },
        "Ed25519VerificationKey2020": {
          "@id": "https://w3id.org/security#Ed25519VerificationKey2020",
          "@context": {
            "@protected": true,
            "id": "@id",
            "type": "@type",
            "controller": {
              "@id": "https://w3id.org/security#controller",
              "@type": "@id"
            },
            "revoked": {
              "@id": "https://w3id.org/security#revoked",
              "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
            },
            "publicKeyMultibase": {
              "@id": "https://w3id.org/security#publicKeyMultibase",
              "@type": "https://w3id.org/security#multibase"
            }
          }
        },
        "Ed25519Signature2020": {
          "@id": "https://w3id.org/security#Ed25519Signature2020",
          "@context": {
            "@protected": true,
            "id": "@id",
            "type": "@type",
            "challenge": "https://w3id.org/security#challenge",
            "created": {
              "@id": "http://purl.org/dc/terms/created",
              "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
            },
            "domain": "https://w3id.org/security#domain",
            "expires": {
              "@id": "https://w3id.org/security#expiration",
              "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
            },
            "nonce": "https://w3id.org/security#nonce",
            "proofPurpose": {
              "@id": "https://w3id.org/security#proofPurpose",
              "@type": "@vocab",
              "@context": {
                "@protected": true,
                "id": "@id",
                "type": "@type",
                "assertionMethod": {
                  "@id": "https://w3id.org/security#assertionMethod",
                  "@type": "@id",
                  "@container": "@set"
                },
                "authentication": {
                  "@id": "https://w3id.org/security#authenticationMethod",
                  "@type": "@id",
                  "@container": "@set"
                },
                "capabilityInvocation": {
                  "@id": "https://w3id.org/security#capabilityInvocationMethod",
                  "@type": "@id",
                  "@container": "@set"
                },
                "capabilityDelegation": {
                  "@id": "https://w3id.org/security#capabilityDelegationMethod",
                  "@type": "@id",
                  "@container": "@set"
                },
                "keyAgreement": {
                  "@id": "https://w3id.org/security#keyAgreementMethod",
                  "@type": "@id",
                  "@container": "@set"
                }
              }
            },
            "proofValue": {
              "@id": "https://w3id.org/security#proofValue",
              "@type": "https://w3id.org/security#multibase"
            },
            "verificationMethod": {
              "@id": "https://w3id.org/security#verificationMethod",
              "@type": "@id"
            }
          }
        }
      }
    }
  };
  
  // grab the built-in Node.js doc loader
  const nodeDocumentLoader = jsonld.documentLoaders.node();
  // or grab the XHR one: jsonld.documentLoaders.xhr()
  
  // change the default document loader
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
  
  
  // alternatively, pass the custom loader for just a specific call:
  const doc = {
    "@context": [
      "https://www.w3.org/ns/did/v1",
      "https://w3id.org/security/suites/ed25519-2020/v1"
    ],
      "type": "Ed25519Signature2020",
      "created": "2024-09-01T17:44:11Z",
      "verificationMethod": "did:hid:testnet:z6Mkk8qQLgMmLKDq6ER9BYGycFEdSaPqy9JPWKUaPGWzJeNp#key-1",
      "proofPurpose": "authentication",
      "challenge": "123123",
      "domain": "http:adsasd"
  }

  const expanded = (async () => {
    return jsonld.expand(
        doc, {documentLoader: customLoader});
  })()

   expanded.then((e) => {
    console.log(JSON.stringify(e))
  });