https://notes.ethereum.org/@vbuterin/proto_danksharding_faq#How-exactly-do-ZK-rollups-work-with-the-KZG-commitment-efficiently



1. Use `btx` package to send the blob to Ethereum consensus layer. This will give you back a `KZG commitment to your blob` and an `opening proof`. The code has lines to save the blob, its versioned hash, and the KZG commitment to a file. 
2. Feed the blob and the commitment to `poe` circuit, which will compute another commitment to the same blob polynomial (simply by hashing), and apply fiat-shamir heuristic to get a challenge point from these two commitments. It will then evaluate the polynomial at this challenge point, and output the resulting value. 
3. Use the `btx` package again to compute an opening proof for the blob polynomial at the challenge point outputted by the circuit. The resulting value must match the one outputted by the circuit. 
4. Use `verifier` package to send this new opening proof to EVM, which will verify the opening on this new point. 

Don't skip to fill-in the values in `.env.example` and save it as `.env`. 
