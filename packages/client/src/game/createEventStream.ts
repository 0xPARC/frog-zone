import { ReplaySubject } from "rxjs";
import type { Coord, Item, Player } from "./store";
export const serverUrl = import.meta.env.VITE_SERVER_URL;

if (!serverUrl) {
	throw new Error("[sync] VITE_SERVER_URL is not defined");
}

type PlayerAdd = ["PlayerAdd", [Player, Coord]];
type PlayerMove = ["PlayerMove", [number, Coord]];
type ItemAdd = ["ItemAdd", [Item, Coord]];
type ItemPickup = [
	"ItemPickup",
	[number, Item, number, number, number, number, Coord],
];

type Event = PlayerAdd | PlayerMove | ItemAdd | ItemPickup;

type ServerStateResponse = Event[];

const POLL_INTERVAL = 500;

export type EventStream = ReturnType<typeof createEventStream>;

const createEventStream = () => {
	const eventStream$ = new ReplaySubject<Event>(-1);

	let lastEventId = -1;

	const poll = async () => {
		const response = await fetch(`${serverUrl}/state`);
		// biome-ignore lint/suspicious/noExplicitAny: server types
		const events = (await response.json()).events.map((a: any) => [
			Object.keys(a)[0],
			Object.values(a).flat(),
		]) as ServerStateResponse;
		const finalEventId = events.length - 1;
		for (let i = lastEventId + 1; i <= finalEventId; i++) {
			const event = events[i];
			eventStream$.next(event);
		}
		lastEventId = finalEventId;
	};

	setInterval(poll, POLL_INTERVAL);

	eventStream$.subscribe((event) => {
		console.log("[sync] event", event);
	});

	return eventStream$;
};

export default createEventStream;
