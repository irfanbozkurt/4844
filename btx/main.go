package main

import (
	"encoding/hex"
	"path/filepath"

	"github.com/ethereum/go-ethereum/common"
)

var (
	PrivKey string
	rpcURL  string
	dirPath = "../files"
	To      = common.HexToAddress("0x0000000000000000000000000000000000000000") // Irrelevant
)

func main() {
	blob := GenerateRandomBlob()

	/////////////////////
	///////////////////// Generate opening proof for blob
	/////////////////////

	openingPoint := GenerateRandomScalar()
	versionedBlobHash, evaluationResult, blobCommitment, openingProof, err := ValidateAndGetOpeningProofForSolidity(blob, openingPoint)
	if err != nil {
		panic(err)
	}

	WriteToFile(filepath.Join(dirPath, "blob"), hex.EncodeToString(blob[:]))
	WriteToFile(filepath.Join(dirPath, "hash"), hex.EncodeToString(versionedBlobHash[:]))
	WriteToFile(filepath.Join(dirPath, "x"), hex.EncodeToString(openingPoint[:]))
	WriteToFile(filepath.Join(dirPath, "y"), hex.EncodeToString(evaluationResult[:]))
	WriteToFile(filepath.Join(dirPath, "commitment"), hex.EncodeToString(blobCommitment[:]))
	WriteToFile(filepath.Join(dirPath, "proof"), hex.EncodeToString(openingProof[:]))

	/////////////////////
	///////////////////// Send blob to Ethereum
	/////////////////////

	// // Send the blob to consensus layer, for those who want to download and check the blob KZG proof
	// blobTxHash := SendBlobCarryingTx(blob)
	// fmt.Printf("\nVisit the following url for tx details\nhttps://sepolia.etherscan.io/tx/%s\n\n", blobTxHash)
	// fmt.Printf("\nVisit the following url to see the blob transaction details\nhttps://sepolia.blobscan.com/blob/%s\n\n", sidecar.BlobHashes()[0])

	// Run the poe circuit, get the evaluation point, and the evaluation result. Here we
	// get the point randomly for demonstration purposes. Also the verification would happen
	// in Solidity.
}