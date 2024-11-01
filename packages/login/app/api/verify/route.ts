import { NextResponse } from "next/server";
import {
  boundConfigFromJSON,
  gpcVerify,
  revealedClaimsFromJSON,
} from "@pcd/gpc";
import path from "path";
import { DevconTicketProofRequest } from "../../../utils/DevconTicketProofRequest";
import { serializeProofResult } from "../../../utils/serialize";

const GPC_ARTIFACTS_PATH = path.join(
  __dirname,
  "..",
  "node_modules",
  "@pcd",
  "proto-pod-gpc-artifacts",
);

export async function OPTIONS() {
  return NextResponse.json(null, { status: 204 });
}

export async function POST(request: Request) {
  try {
    const proofResult = await request.json();

    const serializedProof = serializeProofResult(proofResult);

    // Deserialize values from the client
    const { serializedBoundConfig, serializedRevealedClaims, proof } =
      JSON.parse(serializedProof);
    const boundConfig = boundConfigFromJSON(serializedBoundConfig);
    const revealedClaims = revealedClaimsFromJSON(serializedRevealedClaims);

    // Get values from the proof request for verification
    const { proofConfig, membershipLists, externalNullifier, watermark } =
      DevconTicketProofRequest.getProofRequest();

    // Set circuit identifier to the one from the bound config
    proofConfig.circuitIdentifier = boundConfig.circuitIdentifier;

    // Set external nullifier and watermark
    if (revealedClaims.owner && externalNullifier) {
      revealedClaims.owner.externalNullifier = externalNullifier;
    }
    revealedClaims.watermark = watermark;
    // Set membership lists to values from the proof request
    revealedClaims.membershipLists = membershipLists;

    const result = await gpcVerify(
      proof,
      boundConfig,
      revealedClaims,
      GPC_ARTIFACTS_PATH,
    );

    return NextResponse.json({
      result,
    });
  } catch (error) {
    return NextResponse.json(
      {
        success: false,
        message: `Failed to verify proof: ${JSON.stringify(error)}`,
      },
      { status: 500 },
    );
  }
}
