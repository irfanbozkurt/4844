const fs = require('fs');
const path = require('path');
const { expect } = require('chai');
const { ethers } = require('hardhat');
require("@nomicfoundation/hardhat-chai-matchers");

describe("Run", function () {
    let verifier;

    before(async function () {
        verifier = (await ethers.getContractFactory("Verifier")).attach(
            "0xdceA5C391F6dCfA2a1796fd1a19B6E30569508EF",
        );
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

    it("Failing verify with modified x", async function () {
        const dirPath = "../files";

        const versionedBlobHash = fs.readFileSync(path.join(dirPath, "hash"), 'utf8');
        let openingPoint = fs.readFileSync(path.join(dirPath, "x"), 'utf8');
        const evaluationResult = fs.readFileSync(path.join(dirPath, "y"), 'utf8');
        const blobCommitment = fs.readFileSync(path.join(dirPath, "commitment"), 'utf8');
        const openingProof = fs.readFileSync(path.join(dirPath, "proof"), 'utf8');

        if (blobCommitment.length !== 96 || openingProof.length !== 96) {
            throw new Error("Invalid commitment or proof length");
        }
        if (versionedBlobHash.length !== 64 || openingPoint.length !== 64 || evaluationResult.length !== 64) {
            throw new Error("Invalid hash, x, or y length");
        }

        // Modify one character of the openingPoint
        const modifiedChar = openingPoint[0] === '3' ? '4' : '3';
        openingPoint = modifiedChar + openingPoint.substring(1);

        const _blobHash = "0x" + versionedBlobHash;
        const _x = "0x" + openingPoint;
        const _y = "0x" + evaluationResult;
        const _commitment = "0x" + blobCommitment;
        const _pointProof = "0x" + openingProof;

        await expect(verifier.evaluatePoint(_blobHash, _x, _y, _commitment, _pointProof)).to.be.reverted;
    });
});

