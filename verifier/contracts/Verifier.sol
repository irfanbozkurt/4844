// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

// import "hardhat/console.sol";

contract Verifier {
    address public constant POINT_EVALUATION_PRECOMPILE_ADDRESS = address(0x0A);
    uint256 public constant BLS_MODULUS =
        52_435_875_175_126_190_479_447_740_508_185_965_837_690_552_500_527_637_822_603_658_699_938_581_184_513;
    uint32 public constant FIELD_ELEMENTS_PER_BLOB = 4096;

    constructor() payable {}

    error EVAL_FAILED_1();
    error EVAL_FAILED_2();
    error POINT_X_TOO_LARGE();
    error POINT_Y_TOO_LARGE();

    function getDummyValue() external pure returns (uint256) {
        return 41;
    }

    function getBlobHash(bytes32) external view returns (bytes32) {
        return blobhash(0);
    }

    /// @notice Evaluates the 4844 point using the precompile.
    /// @param _blobHash The versioned hash
    /// @param _x The evaluation point
    /// @param _y The expected output
    /// @param _commitment The input kzg point
    /// @param _pointProof The quotient kzg
    function evaluatePoint(
        bytes32 _blobHash,
        uint256 _x,
        uint256 _y,
        bytes memory _commitment,
        bytes memory _pointProof
    ) external view {
        require(_commitment.length == 48, "Invalid commitment length");
        require(_pointProof.length == 48, "Invalid point proof length");
        
        if (_x >= BLS_MODULUS) revert POINT_X_TOO_LARGE();
        if (_y >= BLS_MODULUS) revert POINT_Y_TOO_LARGE();

        (bool ok, bytes memory ret) = POINT_EVALUATION_PRECOMPILE_ADDRESS
            .staticcall(
                abi.encodePacked(_blobHash, _x, _y, _commitment, _pointProof)
            );

        if (!ok) revert EVAL_FAILED_1();

        if (ret.length != 64) revert EVAL_FAILED_2();

        bytes32 first;
        bytes32 second;
        assembly {
            first := mload(add(ret, 32))
            second := mload(add(ret, 64))
        }
        if (
            uint256(first) != FIELD_ELEMENTS_PER_BLOB ||
            uint256(second) != BLS_MODULUS
        ) {
            revert EVAL_FAILED_2();
        }
    }
}
