export const postNewLogIn = async ({
  publicKey,
  machineId,
}: {
  publicKey: string;
  machineId: string;
}) => {
  if (publicKey && machineId) {
    try {
      const response = await fetch("/api/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ machineId, publicKey }),
      });

      const data = await response.json();
      return data;
    } catch (error) {
      console.error("API error:", error);
    }
  }
};

export const fetchMachineStatus = async (machineId: string) => {
  if (machineId) {
    try {
      const response = await fetch(`/api/machine-status/${machineId}`, {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
      });

      if (!response.ok) {
        throw new Error("Failed to fetch machine status");
      }

      const data = await response.json();
      return data;
    } catch (error) {
      console.error("API error:", error);
      return { success: false, message: "Error fetching machine status" };
    }
  } else {
    console.error("Machine ID is required");
    return { success: false, message: "Machine ID is required" };
  }
};
