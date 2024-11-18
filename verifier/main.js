const fs = require('fs');
const path = require('path');
const { expect } = require('chai');
const hre = require('hardhat');

describe("Run", function () {
  let verifier;

  before(async function () {
    verifier = (await hre.ethers.getContractFactory("Verifier")).attach(
      process.env.SEPOLIA_CONTRACT_ADDRESS,
    );
  });

  it("Can read dummy value", async function () {
    expect(await verifier.getDummyValue()).to.equal(41);
  });

  it("Verify", async function () {
    const dirPath = "../files";

    const versionedBlobHash = fs.readFileSync(path.join(dirPath, "hash"), 'utf8');
    const openingPoint = fs.readFileSync(path.join(dirPath, "x"), 'utf8');
    const evaluationResult = fs.readFileSync(path.join(dirPath, "y"), 'utf8');
    const blobCommitment = fs.readFileSync(path.join(dirPath, "commitment"), 'utf8');
    const openingProof = fs.readFileSync(path.join(dirPath, "proof"), 'utf8');

    if (blobCommitment.length !== 96 || openingProof.length !== 96) {
      throw new Error("Invalid commitment or proof length");
    }
    if (versionedBlobHash.length !== 64 || openingPoint.length !== 64 || evaluationResult.length !== 64) {
      throw new Error("Invalid hash, x, or y length");
    }

    const _blobHash = "0x" + versionedBlobHash;
    const _x = "0x" + openingPoint;
    const _y = "0x" + evaluationResult;
    const _commitment = "0x" + blobCommitment;
    const _pointProof = "0x" + openingProof;

    await verifier.evaluatePoint(_blobHash, _x, _y, _commitment, _pointProof);
  });
});
