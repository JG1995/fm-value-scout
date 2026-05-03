class PlayerState {
	data = $state<{ players: unknown[]; currency: string } | null>(null);
}

export const playerState = new PlayerState();
