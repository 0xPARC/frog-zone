import { connect, ParcnetAPI, Zapp } from "@parcnet-js/app-connector";

const FrogZoneLogin: Zapp = {
  name: "FrogZone Login",
  permissions: {
    READ_PUBLIC_IDENTIFIERS: {},
  },
};

// Function to initialize the connection and return the z instance
export function initializeZConnection(
  elementId: string,
): Promise<ParcnetAPI> | null {
  // Ensure this runs in a browser environment
  if (typeof window !== "undefined") {
    const element = document.getElementById(elementId) as HTMLElement;

    if (element) {
      const clientUrl = "https://zupass.org";
      const z = connect(FrogZoneLogin, element, clientUrl);
      return z;
    } else {
      console.error(`Element with id "${elementId}" not found.`);
    }
  }

  return null;
}
