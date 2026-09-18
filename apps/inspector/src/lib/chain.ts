import { createPublicClient, http, type Address, type Hex } from "viem";
import { sepolia } from "viem/chains";

const registryAbi = [
  {
    type: "function",
    name: "head",
    stateMutability: "view",
    inputs: [{ name: "spaceId", type: "bytes32" }],
    outputs: [
      { name: "transitionId", type: "bytes32" },
      { name: "stateRoot", type: "bytes32" },
      { name: "sequence", type: "uint64" },
    ],
  },
  {
    type: "function",
    name: "spaceAuthorization",
    stateMutability: "view",
    inputs: [{ name: "spaceId", type: "bytes32" }],
    outputs: [
      { name: "controller", type: "address" },
      { name: "authorizer", type: "address" },
      { name: "configNonce", type: "uint64" },
    ],
  },
] as const;

const publicClient = createPublicClient({
  chain: sepolia,
  transport: http(process.env.NEXT_PUBLIC_SEPOLIA_RPC_URL ?? "https://ethereum-sepolia-rpc.publicnode.com"),
});

export type LiveInspection =
  | {
      status: "LIVE";
      blockNumber: number;
      head: { transitionId: Hex; stateRoot: Hex; sequence: number };
      authority: { controller: Address; authorizer: Address; configNonce: number };
    }
  | { status: "FALLBACK"; message: string };

export async function inspectSepolia(registry: Address, spaceId: Hex): Promise<LiveInspection> {
  try {
    const [head, authority, blockNumber] = await Promise.all([
      publicClient.readContract({ address: registry, abi: registryAbi, functionName: "head", args: [spaceId] }),
      publicClient.readContract({ address: registry, abi: registryAbi, functionName: "spaceAuthorization", args: [spaceId] }),
      publicClient.getBlockNumber(),
    ]);
    return {
      status: "LIVE",
      blockNumber: Number(blockNumber),
      head: { transitionId: head[0], stateRoot: head[1], sequence: Number(head[2]) },
      authority: { controller: authority[0], authorizer: authority[1], configNonce: Number(authority[2]) },
    };
  } catch {
    return {
      status: "FALLBACK",
      message: "The browser could not reach the public RPC. Showing the published Sepolia reread instead.",
    };
  }
}
