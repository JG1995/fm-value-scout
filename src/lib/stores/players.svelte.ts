import type { Player } from "$lib/types/player";

class PlayerState {
	data = $state<{ players: Player[]; currency: string } | null>(null);
}

export const playerState = new PlayerState();
