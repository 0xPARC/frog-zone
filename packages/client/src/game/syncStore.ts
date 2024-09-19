import type { EventStream } from "./createEventStream";
import useStore from "./store";

const syncStore = (eventStream$: EventStream) => {
	eventStream$.subscribe((event) => {
		const type = event[0];
		switch (type) {
			case "PlayerAdd": {
				const args = event[1];
				const [player, coord] = args;
				useStore.getState().addPlayer(player, coord);
				break;
			}
			case "PlayerMove": {
				const args = event[1];
				const [playerId, coord] = args;
				useStore.getState().movePlayer(playerId, coord);
				break;
			}
			case "ItemAdd": {
				const args = event[1];
				const [item, coord] = args;
				useStore.getState().addItem(item, coord);
				break;
			}
			case "ItemPickup": {
				const args = event[1];
				const [playerId, , oldHp, newHp, oldAtk, newAtk, coord] = args;
				useStore
					.getState()
					.pickupItem(playerId, coord, { oldHp, oldAtk, newHp, newAtk });
				break;
			}
		}
	});
};

export default syncStore;
