package main

import (
	"context"
	"crypto/rand"
	"crypto/sha256"
	"fmt"
	"log"
	"os"

	"github.com/consensys/gnark-crypto/ecc/bls12-381/fr"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/crypto/kzg4844"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/holiman/uint256"
	"github.com/joho/godotenv"
)

// Leave the most significant 2 bits empty to make sure each chunk fits
// into a bls12-381 scalar field element, which doesn't fully fill 255 bits.
func GenerateRandomBlob() (blob *kzg4844.Blob) {
	blob = new(kzg4844.Blob)
	for i := 0; i < len(blob); i += 32 {
		if _, err := rand.Read(blob[i : i+32]); err != nil {
			log.Fatalf("failed to generate random blob: %v", err)
		}
		blob[i] &= 0x3F
	}
	return
}

func GenerateRandomScalar() (scalar kzg4844.Point) {
	retVal := new(kzg4844.Point)
	if _, err := rand.Read(retVal[:]); err != nil {
		log.Fatalf("failed to generate random scalar: %v", err)
	}
	retVal[0] &= 0x3F
	return *retVal
}

func SendBlobCarryingTx(blob *kzg4844.Blob) string {
	sidecar := GetBlobSidecar(blob)

	client, err := ethclient.Dial(rpcURL)
	if err != nil {
		panic(fmt.Errorf("failed to connect to the Ethereum client: %s", err))
	}
	defer client.Close()

	pKeyBytes, _ := hexutil.Decode("0x" + PrivKey)
	ecdsaPrivateKey, err := crypto.ToECDSA(pKeyBytes)
	if err != nil {
		panic(fmt.Errorf("failed to convert private key to ECDSA: %s", err))
	}

	fromAddress := crypto.PubkeyToAddress(ecdsaPrivateKey.PublicKey)
	nonce, err := client.PendingNonceAt(context.Background(), fromAddress)
	if err != nil {
		panic(fmt.Errorf("failed to get nonce: %s", err))
	}

	chainID, err := client.ChainID(context.Background())
	if err != nil {
		panic(fmt.Errorf("failed to get chain ID: %s", err))
	}

	// Create the transaction with the blob data and cryptographic proofs
	tx := types.NewTx(&types.BlobTx{
		ChainID:    uint256.MustFromBig(chainID),
		Nonce:      nonce,
		GasTipCap:  uint256.NewInt(1e10),  // max priority fee per gas
		GasFeeCap:  uint256.NewInt(50e10), // max fee per gas
		Gas:        250000,                // gas limit for the transaction
		To:         To,                    // recipient's address
		Value:      uint256.NewInt(0),     // value transferred in the transaction
		Data:       nil,                   // No additional data is sent in this transaction
		BlobFeeCap: uint256.NewInt(3e10),  // fee cap for the blob data
		BlobHashes: sidecar.BlobHashes(),  // blob hashes in the transaction
		Sidecar:    &sidecar,              // sidecar data in the transaction
	})

	signedTx, err := types.SignTx(tx, types.LatestSignerForChainID(chainID), ecdsaPrivateKey)
	if err != nil {
		panic(fmt.Errorf("failed to sign transaction: %s", err))
	}

	if err = client.SendTransaction(context.Background(), signedTx); err != nil {
		panic(fmt.Errorf("failed to send transaction: %s", err))
	}

	return signedTx.Hash().Hex()
}

func GetBlobSidecar(blob *kzg4844.Blob) types.BlobTxSidecar {
	blobCommitment, err := kzg4844.BlobToCommitment(blob)
	if err != nil {
		panic(fmt.Errorf("failed to compute blob commitment: %s", err))
	}

	blobProof, err := kzg4844.ComputeBlobProof(blob, blobCommitment)
	if err != nil {
		panic(fmt.Errorf("failed to compute blob proof: %s", err))
	}

	if err = kzg4844.VerifyBlobProof(blob, blobCommitment, blobProof); err != nil {
		panic(fmt.Errorf("failed to verify blob proof: %s", err))
	}

	return types.BlobTxSidecar{
		Blobs:       []kzg4844.Blob{*blob},
		Commitments: []kzg4844.Commitment{blobCommitment},
		Proofs:      []kzg4844.Proof{blobProof},
	}
}

func ValidateAndGetOpeningProofForSolidity(
	blob *kzg4844.Blob,
	openingPoint kzg4844.Point, // [32]byte,
) (
	versionedBlobHash [32]byte,
	evaluationResult kzg4844.Claim, // [32]byte
	blobCommitment kzg4844.Commitment, // [48]byte
	openingProof kzg4844.Proof, // [48]byte,
	err error,
) {
	if blobCommitment, err = kzg4844.BlobToCommitment(blob); err != nil {
		return versionedBlobHash, evaluationResult, blobCommitment, openingProof, fmt.Errorf("failed to compute blob commitment: %w", err)
	}

	versionedBlobHash = kzg4844.CalcBlobHashV1(sha256.New(), &blobCommitment)

	if openingProof, evaluationResult, err = kzg4844.ComputeProof(blob, openingPoint); err != nil {
		return versionedBlobHash, evaluationResult, blobCommitment, openingProof, fmt.Errorf("failed to compute proof: %w", err)
	}

	if err = kzg4844.VerifyProof(blobCommitment, openingPoint, evaluationResult, openingProof); err != nil {
		return versionedBlobHash, evaluationResult, blobCommitment, openingProof, fmt.Errorf("failed to verify blob proof: %w", err)
	}

	return
}

func init() {
	if err := godotenv.Load("../.env"); err != nil {
		log.Fatalf("Error loading .env file")
	}
	PrivKey = os.Getenv("PRIV_KEY")
	rpcURL = os.Getenv("SEPOLIA_RPC_URL")

	baseDir := "../files"
	if _, err := os.Stat(baseDir); os.IsNotExist(err) {
		if err := os.MkdirAll(baseDir, os.ModePerm); err != nil {
			log.Fatalf("failed to create base directory: %v", err)
		}
	}
}

func WriteToFile(filename, data string) {
	file, err := os.Create(filename)
	if err != nil {
		log.Fatalf("failed to create file: %v", err)
	}
	defer file.Close()

	_, err = file.WriteString(data)
	if err != nil {
		log.Fatalf("failed to write to file: %v", err)
	}
}

func DeserializeBlob(blob *kzg4844.Blob) (Polynomial, error) {
	poly := make(Polynomial, 4096)
	for i := 0; i < 4096; i++ {
		if err := poly[i].SetBytesCanonical(blob[i*32 : (i+1)*32]); err != nil {
			return nil, fmt.Errorf("deserialize blob: %w", err)
		}
	}
	return poly, nil
}

func DeserializeScalar(serScalar [32]byte) (fr.Element, error) {
	scalar, err := ReduceCanonicalBigEndian(serScalar[:])
	if err != nil {
		return fr.Element{}, fmt.Errorf("deserialize scalar: %w", err)
	}
	return scalar, nil
}

func ReduceCanonicalBigEndian(serScalar []byte) (fr.Element, error) {
	var scalar fr.Element
	err := scalar.SetBytesCanonical(serScalar)

	return scalar, err
}
