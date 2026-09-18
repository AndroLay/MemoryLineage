// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

interface IERC1271 {
    function isValidSignature(bytes32 digest, bytes calldata signature) external view returns (bytes4);
}

/// @notice Local independent implementation of the normative ERC-8350 v1
/// registry boundary. The contract stores fixed-size commitments only.
contract MemoryLineageRegistry {
    string public constant EIP712_NAME = "AgentMemoryState";
    string public constant EIP712_VERSION = "1";

    bytes32 public constant EIP712_DOMAIN_TYPEHASH = keccak256(
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
    );
    bytes32 public constant EXPERIENCE_DELTA_TYPEHASH = keccak256(
        "ExperienceDelta(bytes32 spaceId,uint64 sequence,bytes32 prevStateRoot,bytes32 deltaCommitment,bytes32 provenanceCommitment,bytes32 profileId,bytes32 locatorCommitment)"
    );
    bytes32 public constant MEMORY_STATE_TYPEHASH = keccak256(
        "MemoryState(bytes32 prevStateRoot,bytes32 transitionId)"
    );
    bytes32 public constant SPACE_REGISTRATION_TYPEHASH = keccak256(
        "SpaceRegistration(bytes32 spaceId,address controller,address authorizer)"
    );
    bytes32 public constant MEMORY_SPACE_TYPEHASH = keccak256(
        "MemorySpace(address initialController,bytes32 salt)"
    );
    bytes32 public constant SPACE_AUTHORIZATION_TYPEHASH = keccak256(
        "SpaceAuthorization(bytes32 spaceId,address newController,address newAuthorizer,uint64 nonce)"
    );

    bytes4 private constant ERC1271_MAGIC_VALUE = 0x1626ba7e;
    bytes32 public constant ZERO_ROOT = bytes32(0);
    uint256 private constant SECP256K1N_HALF =
        0x7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0;

    struct ExperienceDelta {
        bytes32 spaceId;
        uint64 sequence;
        bytes32 prevStateRoot;
        bytes32 deltaCommitment;
        bytes32 provenanceCommitment;
        bytes32 profileId;
        bytes32 locatorCommitment;
    }

    struct SpaceRecord {
        address controller;
        address authorizer;
        bytes32 transitionId;
        bytes32 stateRoot;
        uint64 sequence;
        uint64 configNonce;
    }

    struct StoredTransition {
        bytes32 spaceId;
        bytes32 nextStateRoot;
        uint64 sequence;
        uint64 committedAt;
    }

    uint256 private immutable _cachedChainId;
    bytes32 private immutable _cachedDomainSeparator;
    mapping(bytes32 => SpaceRecord) private _spaces;
    mapping(bytes32 => StoredTransition) private _transitions;

    event SpaceRegistered(bytes32 indexed spaceId, address indexed controller, address indexed authorizer);
    event SpaceAuthorizationUpdated(
        bytes32 indexed spaceId,
        address indexed controller,
        address indexed authorizer,
        uint64 configNonce
    );
    event TransitionCommitted(
        bytes32 indexed spaceId,
        bytes32 indexed transitionId,
        uint64 indexed sequence,
        bytes32 prevStateRoot,
        bytes32 nextStateRoot,
        bytes32 deltaCommitment,
        bytes32 provenanceCommitment,
        bytes32 profileId,
        bytes32 locatorCommitment,
        address authorizer
    );

    constructor() {
        _cachedChainId = block.chainid;
        _cachedDomainSeparator = _buildDomainSeparator();
    }

    function registerSpace(
        bytes32 spaceId,
        address controller,
        address authorizer,
        bytes32 salt,
        bytes calldata controllerSignature
    ) external {
        require(spaceId != bytes32(0), "ZERO_SPACE_ID");
        require(controller != address(0) && authorizer != address(0), "ZERO_ADDRESS");
        require(deriveSpaceId(controller, salt) == spaceId, "INVALID_SPACE_ID");
        require(_spaces[spaceId].controller == address(0), "SPACE_EXISTS");

        bytes32 registrationId = keccak256(
            abi.encode(SPACE_REGISTRATION_TYPEHASH, spaceId, controller, authorizer)
        );
        _requireAuthorization(controller, _digest(registrationId), controllerSignature, true);

        _spaces[spaceId] = SpaceRecord({
            controller: controller,
            authorizer: authorizer,
            transitionId: bytes32(0),
            stateRoot: ZERO_ROOT,
            sequence: 0,
            configNonce: 0
        });
        emit SpaceRegistered(spaceId, controller, authorizer);
    }

    function updateSpaceAuthorization(
        bytes32 spaceId,
        address newController,
        address newAuthorizer,
        bytes calldata controllerSignature
    ) external {
        require(newController != address(0) && newAuthorizer != address(0), "ZERO_ADDRESS");
        SpaceRecord storage space = _space(spaceId);
        uint64 nextNonce = space.configNonce + 1;
        bytes32 authorizationId = keccak256(
            abi.encode(SPACE_AUTHORIZATION_TYPEHASH, spaceId, newController, newAuthorizer, nextNonce)
        );
        _requireAuthorization(space.controller, _digest(authorizationId), controllerSignature, true);
        space.controller = newController;
        space.authorizer = newAuthorizer;
        space.configNonce = nextNonce;
        emit SpaceAuthorizationUpdated(spaceId, newController, newAuthorizer, nextNonce);
    }

    function commitTransition(ExperienceDelta calldata delta, bytes calldata authorizerSignature)
        external
        returns (bytes32 transitionId, bytes32 nextStateRoot)
    {
        SpaceRecord storage space = _space(delta.spaceId);
        require(delta.deltaCommitment != bytes32(0), "ZERO_DELTA_COMMITMENT");
        require(delta.profileId != bytes32(0), "ZERO_PROFILE_ID");
        require(delta.sequence == space.sequence + 1, "BAD_SEQUENCE");
        require(delta.prevStateRoot == space.stateRoot, "BAD_PREVIOUS_STATE");

        transitionId = hashExperienceDelta(delta);
        require(_transitions[transitionId].spaceId == bytes32(0), "TRANSITION_EXISTS");
        _requireAuthorization(space.authorizer, _digest(transitionId), authorizerSignature, true);

        nextStateRoot = computeNextStateRoot(delta.prevStateRoot, transitionId);
        _transitions[transitionId] = StoredTransition({
            spaceId: delta.spaceId,
            nextStateRoot: nextStateRoot,
            sequence: delta.sequence,
            committedAt: uint64(block.timestamp)
        });
        space.transitionId = transitionId;
        space.stateRoot = nextStateRoot;
        space.sequence = delta.sequence;
        emit TransitionCommitted(
            delta.spaceId,
            transitionId,
            delta.sequence,
            delta.prevStateRoot,
            nextStateRoot,
            delta.deltaCommitment,
            delta.provenanceCommitment,
            delta.profileId,
            delta.locatorCommitment,
            space.authorizer
        );
    }

    function head(bytes32 spaceId)
        external
        view
        returns (bytes32 transitionId, bytes32 stateRoot, uint64 sequence)
    {
        SpaceRecord storage space = _space(spaceId);
        return (space.transitionId, space.stateRoot, space.sequence);
    }

    function spaceAuthorization(bytes32 spaceId)
        external
        view
        returns (address controller, address authorizer, uint64 configNonce)
    {
        SpaceRecord storage space = _space(spaceId);
        return (space.controller, space.authorizer, space.configNonce);
    }

    function transition(bytes32 transitionId)
        external
        view
        returns (bytes32 spaceId, bytes32 nextStateRoot, uint64 sequence, uint64 committedAt)
    {
        StoredTransition storage record = _transitions[transitionId];
        return (record.spaceId, record.nextStateRoot, record.sequence, record.committedAt);
    }

    function hashExperienceDelta(ExperienceDelta calldata delta) public pure returns (bytes32) {
        return keccak256(
            abi.encode(
                EXPERIENCE_DELTA_TYPEHASH,
                delta.spaceId,
                delta.sequence,
                delta.prevStateRoot,
                delta.deltaCommitment,
                delta.provenanceCommitment,
                delta.profileId,
                delta.locatorCommitment
            )
        );
    }

    function computeNextStateRoot(bytes32 prevStateRoot, bytes32 transitionId)
        public
        pure
        returns (bytes32)
    {
        return keccak256(abi.encode(MEMORY_STATE_TYPEHASH, prevStateRoot, transitionId));
    }

    function deriveSpaceId(address initialController, bytes32 salt) public pure returns (bytes32) {
        require(initialController != address(0), "ZERO_ADDRESS");
        return keccak256(abi.encode(MEMORY_SPACE_TYPEHASH, initialController, salt));
    }

    function hashSpaceRegistration(bytes32 spaceId, address controller, address authorizer)
        external
        pure
        returns (bytes32)
    {
        return keccak256(abi.encode(SPACE_REGISTRATION_TYPEHASH, spaceId, controller, authorizer));
    }

    function hashSpaceAuthorization(
        bytes32 spaceId,
        address newController,
        address newAuthorizer,
        uint64 nonce
    ) external pure returns (bytes32) {
        return keccak256(
            abi.encode(SPACE_AUTHORIZATION_TYPEHASH, spaceId, newController, newAuthorizer, nonce)
        );
    }

    function signingDigest(bytes32 structHash) external view returns (bytes32) {
        return _digest(structHash);
    }

    function computeDomainSeparator(uint256 chainId, address verifyingContract)
        public
        pure
        returns (bytes32)
    {
        return keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH,
                keccak256(bytes(EIP712_NAME)),
                keccak256(bytes(EIP712_VERSION)),
                chainId,
                verifyingContract
            )
        );
    }

    function computeSigningDigest(bytes32 structHash, uint256 chainId, address verifyingContract)
        external
        pure
        returns (bytes32)
    {
        return keccak256(
            abi.encodePacked("\x19\x01", computeDomainSeparator(chainId, verifyingContract), structHash)
        );
    }

    function domainSeparator() public view returns (bytes32) {
        return block.chainid == _cachedChainId ? _cachedDomainSeparator : _buildDomainSeparator();
    }

    function _space(bytes32 spaceId) private view returns (SpaceRecord storage space) {
        space = _spaces[spaceId];
        require(space.controller != address(0), "UNKNOWN_SPACE");
    }

    function _requireAuthorization(
        address signer,
        bytes32 message,
        bytes calldata signature,
        bool allowDirectCall
    ) private view {
        if (allowDirectCall && msg.sender == signer && signature.length == 0) return;
        if (signer.code.length != 0 && _isValidERC1271Signature(signer, message, signature)) return;
        require(signature.length == 65, "INVALID_AUTHORIZATION");
        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := calldataload(signature.offset)
            s := calldataload(add(signature.offset, 32))
            v := byte(0, calldataload(add(signature.offset, 64)))
        }
        if (v < 27) v += 27;
        require(v == 27 || v == 28, "INVALID_AUTHORIZATION");
        require(uint256(s) <= SECP256K1N_HALF && s != bytes32(0), "INVALID_AUTHORIZATION");
        require(ecrecover(message, v, r, s) == signer, "INVALID_AUTHORIZATION");
    }

    function _isValidERC1271Signature(address signer, bytes32 message, bytes calldata signature)
        private
        view
        returns (bool)
    {
        (bool success, bytes memory result) = signer.staticcall(
            abi.encodeWithSelector(IERC1271.isValidSignature.selector, message, signature)
        );
        return success && result.length >= 32 && abi.decode(result, (bytes4)) == ERC1271_MAGIC_VALUE;
    }

    function _digest(bytes32 structHash) private view returns (bytes32) {
        return keccak256(abi.encodePacked("\x19\x01", domainSeparator(), structHash));
    }

    function _buildDomainSeparator() private view returns (bytes32) {
        return computeDomainSeparator(block.chainid, address(this));
    }
}
