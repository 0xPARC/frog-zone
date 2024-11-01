import { ParcnetAPI } from "@parcnet-js/app-connector";
import { boundConfigToJSON, revealedClaimsToJSON } from "@pcd/gpc";

export type ProveResult = Extract<
  Awaited<ReturnType<ParcnetAPI["gpc"]["prove"]>>,
  { success: true }
>;

export function serializeProofResult(result: ProveResult) {
  const serializedProofResult = {
    proof: result.proof,
    serializedBoundConfig: boundConfigToJSON(result.boundConfig),
    serializedRevealedClaims: revealedClaimsToJSON(result.revealedClaims),
  };
  return serializedProofResult;
}
