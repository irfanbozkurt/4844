https://notes.ethereum.org/@vbuterin/proto_danksharding_faq#How-exactly-do-ZK-rollups-work-with-the-KZG-commitment-efficiently


`btx` package demonstrates sending a blob carrying transaction and getting the KZG commitment for the blob data, and an opening proof at a point determined by EVM. 
- Modify and run `main.go`. You can send a blob-carrying transaction to the consensus layer, or, you can generate an opening proof for your blob data. 
- If you do the latter, it will save some artifacts in `files` folder to be read by `verifier` package. 


`poe` package implements proof-of-commitment-equivalence in plonky2

`verifier` package implements the smart contract to verify the KZG proof returned by EVM, the zk proof-of-commitment-equivalence, and another KZG proof for the same polynomial at a different point. 
- After creating a blob commitment via `btx` package, you can run `npm run verify-sepolia` to call corresponding smart contract and check on-chain using the new point evaluation precompile. 

Don't skip to fill-in the values in `.env.example` and save it as `.env`. 
