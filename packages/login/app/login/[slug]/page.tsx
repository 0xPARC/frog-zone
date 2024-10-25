"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import { initializeZConnection } from "@/lib/zupass/connector";
import { postNewLogIn, fetchMachineStatus } from "@/utils/api";

export default function Home() {
  const [isClient, setIsClient] = useState(false);
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [isZInstanceInitialized, setIsZInstanceInitialized] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
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
    if (!isZInstanceInitialized && !publicKey) {
      setIsConnecting(true);
      const zInstance = await initializeZConnection("app-connector");

      if (zInstance) {
        const pKey = await zInstance.identity.getPublicKey();
        setPublicKey(pKey);

        const loginData = await postNewLogIn({
          publicKey: pKey,
          machineId: machineId as string,
        });

        console.log("Login response:", loginData);

        checkMachineStatus();

        setIsZInstanceInitialized(true);
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
      <div className="absolute top-0 p-3">
        {publicKey ? (
          <>
            <h1 className="font-bold text-xl">Game started!</h1>
            <p>Logged in with public key: {publicKey}</p>
            <p>Player: Frog #{machineId}</p>
          </>
        ) : (
          <div>
            <p>
              Do you want to connect your Zupass to device with ID: {machineId}?
            </p>
            <button
              onClick={handleInitializeZInstance}
              className="px-4 py-2 bg-green-500 text-white font-bold rounded"
            >
              {isConnecting ? "Connecting..." : "Connect"}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
