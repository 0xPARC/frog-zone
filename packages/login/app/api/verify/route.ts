import { deserializeProofResult } from "@/utils/serialize";
import { getTicketProofRequest } from "@/utils/ticketProof";
import { gpcVerify } from "@pcd/gpc";
import { NextResponse } from "next/server";
// @ts-expect-error ffjavascript does not have types
import { getCurveFromName } from "ffjavascript";
import path from "path";
import fs from "fs";

const GPC_ARTIFACTS_PATH = path.join(
  process.cwd(),
  "..",
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

    function readDirRecursively(dir: fs.PathLike) {
      const entries = fs.readdirSync(dir);
      console.log(`Contents of ${dir}:`);
      entries.forEach(entry => {
        const fullPath = path.join(dir.toString(), entry);
        const stats = fs.statSync(fullPath);
        if (stats.isDirectory()) {
          readDirRecursively(fullPath);
        } else {
          console.log(entry);
        }
      });
    }

    readDirRecursively(path.join(process.cwd(), "..", ".."));

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


    console.log("VERIFY REQ");

    const res = await gpcVerify(
      proof,
      {
        ...proofRequest.proofConfig,
        circuitIdentifier: boundConfig.circuitIdentifier,
      },
      revealedClaims,
      GPC_ARTIFACTS_PATH,
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
