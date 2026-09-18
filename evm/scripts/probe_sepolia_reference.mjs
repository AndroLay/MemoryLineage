import { Contract, JsonRpcProvider } from 'ethers';

const RPC_URL = process.env.SEPOLIA_RPC_URL ?? 'https://ethereum-sepolia-rpc.publicnode.com';
const REFERENCE = '0xDdf21937ba80b5fF973610877A0955b320C91241';
const ABI = [
  'function EXPERIENCE_DELTA_TYPEHASH() view returns (bytes32)',
  'function MEMORY_STATE_TYPEHASH() view returns (bytes32)',
  'function MEMORY_SPACE_TYPEHASH() view returns (bytes32)',
];
const provider = new JsonRpcProvider(RPC_URL, { name: 'sepolia', chainId: 11155111 }, { staticNetwork: true });
const network = await provider.getNetwork();
const blockNumber = await provider.getBlockNumber();
const registry = new Contract(REFERENCE, ABI, provider);
const result = {
  evidenceType: 'external_reference_live_read_only',
  chainId: network.chainId.toString(),
  rpcUrl: RPC_URL,
  blockNumber,
  referenceRegistry: REFERENCE,
  typehashes: {
    experienceDelta: await registry.EXPERIENCE_DELTA_TYPEHASH(),
    memoryState: await registry.MEMORY_STATE_TYPEHASH(),
    memorySpace: await registry.MEMORY_SPACE_TYPEHASH(),
  },
  note: 'This is the upstream reference deployment; it is not workspace-owned deployment evidence.',
};
console.log(JSON.stringify(result, null, 2));
