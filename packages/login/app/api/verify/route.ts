import { deserializeProofResult } from "@/utils/serialize";
import { getTicketProofRequest } from "@/utils/ticketProof";
import { gpcVerify } from "@pcd/gpc";
import { NextResponse } from "next/server";
// @ts-expect-error ffjavascript does not have types
import { getCurveFromName } from "ffjavascript";
import urljoin from "url-join";
import path from "path";
import fs from "fs";

// const GPC_ARTIFACTS_PATH = path.join(
//   "/var/task",
//   ".output",
//   "public",
//   "artifacts",
// );

export async function OPTIONS() {
  return NextResponse.json(null, { status: 204 });
}

export async function POST(request: Request) {
  try {
    const { proof: proofResult } = await request.json();
    const { boundConfig, revealedClaims, proof } =
      deserializeProofResult(proofResult);
    const proofRequest = getTicketProofRequest().getProofRequest();

    console.log("PROOF REQUEST", proofRequest);

    // Multi-threaded verification seems to be broken in NextJS, so we need to
    // initialize the curve in single-threaded mode.

    // @ts-expect-error ffjavascript does not have types
    if (!globalThis.curve_bn128) {
      // @ts-expect-error ffjavascript does not have types
      globalThis.curve_bn128 = getCurveFromName("bn128", {
        singleThread: true,
      });
    }

    throw new Error(fs.readdirSync(path.join(process.cwd())).join("|"));

    console.log("VERIFY REQ");

    const res = await gpcVerify(
      proof,
      {
        ...proofRequest.proofConfig,
        circuitIdentifier: boundConfig.circuitIdentifier,
      },
      revealedClaims,
      urljoin(new URL(request.url).origin, "artifacts"),
    );

    console.log("GCP VERIFY", res);

    return NextResponse.json({
      verified: res,
    });
  } catch (error) {
    console.log("----> ERROR", error);
    return NextResponse.json(
      {
        success: false,
        message: `Failed to verify proof: ${JSON.stringify(error)}`,
      },
      { status: 500 },
    );
  }
}
