const fs = require('fs');
const path = require('path');
const { expect } = require('chai');
const { ethers } = require('hardhat');
require("@nomicfoundation/hardhat-chai-matchers");

describe("Run", function () {
    let verifier;

    before(async function () {
        verifier = (await ethers.getContractFactory("Verifier")).attach(
            "0xb4B46bdAA835F8E4b4d8e208B6559cD267851051",
        );
    });

    it("Query balance", async function () {
        const address = "0xbec46bA213691c0d64733D92E609b42eb6Be5AeB";
        const balance = await ethers.provider.getBalance(address);
        console.log(`Balance of ${address}: ${ethers.formatEther(balance)} ETH`);

        // const address = "0xAC2a8969083771900578198E1faa846F36223210";
        // const code = await ethers.provider.getCode(address);
        // if (code === '0x') {
        //     console.log(`No contract deployed at ${address}`);
        // } else {
        //     console.log(`Contract is deployed at ${address}`);
        // }
    });

    it("Check ERC20 balance of 0x70997970C51812dc3A010C7d01b50e0d17dc79C8", async function () {
        const erc20Address = "0xC6bA8C3233eCF65B761049ef63466945c362EdD2";
        const accountAddress = "0x2270F962e7a0363295e2424112f48191f5862780";

        const erc20Abi = [
            "function balanceOf(address owner) view returns (uint256)"
        ];

        const erc20Contract = new ethers.Contract(erc20Address, erc20Abi, ethers.provider);
        const balance = await erc20Contract.balanceOf(accountAddress);
        console.log(`ERC20 balance of ${accountAddress}: ${balance} tokens`);

        // 199999999999999999393959960000000
        // 199999999999999999313959960000000
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

        await verifier.EvaluatePoint(_blobHash, _x, _y, _commitment, _pointProof);
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

        await expect(verifier.EvaluatePoint(_blobHash, _x, _y, _commitment, _pointProof)).to.be.reverted;
    });
});

