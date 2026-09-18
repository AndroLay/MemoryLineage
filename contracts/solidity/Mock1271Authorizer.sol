// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract Mock1271Authorizer {
    bytes4 public constant MAGICVALUE = 0x1626ba7e;
    address public immutable owner;
    bool public accept;

    constructor(address initialOwner) {
        owner = initialOwner;
        accept = true;
    }

    function setAccept(bool next) external {
        require(msg.sender == owner, "ONLY_OWNER");
        accept = next;
    }

    function isValidSignature(bytes32 digest, bytes calldata signature) external view returns (bytes4) {
        if (!accept || signature.length != 65) return bytes4(0xffffffff);
        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := calldataload(signature.offset)
            s := calldataload(add(signature.offset, 32))
            v := byte(0, calldataload(add(signature.offset, 64)))
        }
        if (v < 27) v += 27;
        if (v != 27 && v != 28) return bytes4(0xffffffff);
        return ecrecover(digest, v, r, s) == owner ? MAGICVALUE : bytes4(0xffffffff);
    }
}
