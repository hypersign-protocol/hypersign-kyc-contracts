## ------ insecure doc
```
{
  "@context":[
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/ed25519-2020/v1"
  ],
  "id": "did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM",
  "controller": [
    "did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM"
  ],
  "alsoKnownAs": [
    "did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM"
  ],
  "verificationMethod": [
    {
      "id": "did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1",
      "type": "Ed25519VerificationKey2020",
      "controller": "did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM",
      "publicKeyMultibase": "z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM"
    }
  ],
  "authentication": [
    "did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1"
  ],
  "assertionMethod": [
    "did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1"
  ],
  "keyAgreement": [],
  "capabilityInvocation": [],
  "capabilityDelegation": [],
  "service": []
}
```

## ------ insecure canonized doc 


```

<did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519VerificationKey2020> .
<did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1> <https://w3id.org/security#controller> <did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM> .
<did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1> <https://w3id.org/security#publicKeyMultibase> "z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM"^^<https://w3id.org/security#multibase> .
<did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM> <https://w3id.org/security#assertionMethod> <did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1> .
<did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM> <https://w3id.org/security#authenticationMethod> <did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1> .
<did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM> <https://w3id.org/security#controller> <did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM> .
<did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM> <https://w3id.org/security#verificationMethod> <did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1> .
<did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM> <https://www.w3.org/ns/activitystreams#alsoKnownAs> <did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM> .


```
## ---- insecure doc hash 


```
1e4c6152326424ee226927ce572264fb05958df55156f8241cf2db3bc113bfb7
```

## ---- inscure proof doc 
```
{
    "@context":[
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/ed25519-2020/v1"
    ],
    "type": "Ed25519Signature2020",
    "created": "2024-05-09T08:01:46Z",
    "verificationMethod": "did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1",
    "proofPurpose": "authentication",
    "challenge": "1231231231",
    "domain": "www.adbv.com"
}
```
## --- insude canonized proof doc 
```
_:c14n0 <http://purl.org/dc/terms/created> "2024-05-09T08:01:46Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
_:c14n0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2020> .
_:c14n0 <https://w3id.org/security#challenge> "1231231231" .
_:c14n0 <https://w3id.org/security#domain> "www.adbv.com" .
_:c14n0 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#authenticationMethod> .
_:c14n0 <https://w3id.org/security#verificationMethod> <did:hid:testnet:z6MkmKhhHKKAXrMcfLDZZkd5fhx1jUa1sz87QP6j9LtvHBwM#key-1> .
```
## --- proof doc hash 
```
300ca1bc6cda0ef58ce58f638afc759be35c39fb41ae8879687d9180e581b720
```
## --- Message 
```
300ca1bc6cda0ef58ce58f638afc759be35c39fb41ae8879687d9180e581b7201e4c6152326424ee226927ce572264fb05958df55156f8241cf2db3bc113bfb7
```