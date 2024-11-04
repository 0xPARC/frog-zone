"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import { connect, Zapp } from "@parcnet-js/app-connector";
import { postNewLogIn, fetchMachineStatus, verifyProof } from "@/utils/api";
import { getTicketProofRequest } from "@/utils/ticketProof";

const myApp: Zapp = {
  name: "Devcon Ticket Authentication",
  permissions: {
    REQUEST_PROOF: { collections: ["Devcon SEA"] },
    READ_PUBLIC_IDENTIFIERS: {},
  },
};

export default function Home() {
  const [isClient, setIsClient] = useState(false);
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [isZInstanceInitialized, setIsZInstanceInitialized] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const params = useParams();
  const machineId = params.slug as string;

  useEffect(() => {
    setIsClient(true);
  }, []);

  const checkMachineStatus = async () => {
    if (machineId) {
      const status = await fetchMachineStatus(machineId);
      if (status.success && status.machine) {
        setPublicKey(status.machine.publicKey);
      }
    }
  };

  useEffect(() => {
    if (isClient) {
      checkMachineStatus();
    }
  }, [isClient, checkMachineStatus]);

  const handleInitializeZInstance = async () => {
    setError(null);
    if (!isZInstanceInitialized && !publicKey) {
      setIsConnecting(true);
      const zInstance = await await connect(
        myApp,
        document.querySelector<HTMLDivElement>("#app-connector")!,
        "https://zupass.org",
      );

      console.log("APP CONNECTED: zinstance", zInstance);

      if (zInstance) {
        const pKey = await zInstance.identity.getPublicKey();
        const proofRequest = getTicketProofRequest();
        // gpc proof code follows this sample: https://github.com/robknight/gpc-sample/
        const proof = await zInstance?.gpc.prove({
          request: proofRequest.schema,
          collectionIds: ["Devcon SEA"],
        });

        if (!proof?.success) {
          console.error("Failed to prove ticket.", proof);
          setIsConnecting(false);
          setError(
            "Sorry. It looks like we failed to find your ticket. Please try again and make sure your account is correct.",
          );
          return;
        }

        const result = await verifyProof({ proof });

        if (!result.verified) {
          console.error("Failed to verify proof.", result);
          setError(
            "Sorry. It looks like we failed to verify your ticket. Please try again.",
          );
          setIsConnecting(false);
          return;
        }

        const loginData = await postNewLogIn({
          publicKey: pKey,
          machineId: machineId as string,
        });

        console.log("LOGIN", loginData);

        checkMachineStatus();
        setIsZInstanceInitialized(true);
        setPublicKey(pKey);
        setIsConnecting(false);
      } else {
        console.error("Failed to initialize Z instance.");
      }
      setIsConnecting(false);
    }
  };

  if (!isClient) {
    return null;
  }

  return (
    <div className="bg-black white">
      <div
        className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]"
        id="app-connector"
      ></div>
      <div className="absolute top-0 p-3 max-w-md">
        {publicKey ? (
          <>
            <h1 className="font-bold text-xl">Game started!</h1>
            <p>Logged in with public key: {publicKey}</p>
            <p>Player: Frog #{machineId}</p>
          </>
        ) : (
          <div>
            <p className="mb-2">
              Please connect to verify your ticket via Zupass.
            </p>
            <p className="text-sm text-gray-400 mb-2">Device id: {machineId}</p>
            <button
              onClick={handleInitializeZInstance}
              className="px-4 py-2 bg-green-500 text-white font-bold rounded"
            >
              {isConnecting ? "Connecting..." : "Connect"}
            </button>
            {error && <p className="text-red-400 text-xs mb-2 mt-3">{error}</p>}
          </div>
        )}
      </div>
    </div>
  );
}
