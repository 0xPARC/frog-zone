import { ticketProofRequest } from "@parcnet-js/ticket-spec";
export function getTicketProofRequest() {
  return ticketProofRequest({
    classificationTuples: [
      {
        signerPublicKey: "YwahfUdUYehkGMaWh0+q3F8itx2h8mybjPmt8CmTJSs",
        eventId: "5074edf5-f079-4099-b036-22223c0c6995",
      },
    ],
    fieldsToReveal: {
      attendeeEmail: true,
      attendeeName: true,
      eventId: true,
    },
  });
}
