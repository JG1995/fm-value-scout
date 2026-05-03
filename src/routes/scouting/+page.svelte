<script lang="ts">
	import { playerState } from "$lib/stores/players.svelte";
	import type { Player } from "$lib/types/player";
	import { ScoutingTable } from "$lib/components";

	const players = $derived((playerState.data?.players ?? []) as Player[]);

	const currency = $derived(playerState.data?.currency ?? "\u00A3");
</script>

<div class="page">
	{#if players.length === 0}
		<div class="empty-state">
			<div class="empty-card">
				<h1 class="title">No Squad Data</h1>
				<p class="subtitle">Upload a CSV export first to view player data.</p>
				<a href="/" class="back-link">Upload CSV</a>
			</div>
		</div>
	{:else}
		<div class="data-container">
			<h1 class="title">Scouting</h1>
			<ScoutingTable {players} {currency} />
		</div>
	{/if}
</div>

<style>
	.page {
		display: flex;
		flex-direction: column;
		height: 100%;
		box-sizing: border-box;
	}

	/* Empty state — centered */
	.empty-state {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
	}

	.empty-card {
		text-align: center;
	}

	.title {
		font-size: var(--font-size-headline-md);
		font-weight: var(--font-weight-headline-md);
		color: var(--color-gold);
		margin: 0 0 var(--space-stack-sm);
	}

	.subtitle {
		font-size: var(--font-size-body-md);
		font-weight: var(--font-weight-body-md);
		color: var(--color-on-surface-variant);
		margin: 0 0 var(--space-6);
	}

	.back-link {
		display: inline-block;
		font-size: var(--font-size-body-md);
		font-weight: 500;
		color: var(--color-gold);
		text-decoration: none;
		border: 1px solid var(--color-gold);
		border-radius: var(--radius-button);
		padding: var(--space-2) var(--space-5);
		transition:
			background 0.2s ease,
			color 0.2s ease;
	}

	.back-link:hover {
		background: var(--color-gold);
		color: var(--color-on-secondary);
	}

	/* Data display */
	.data-container {
		display: flex;
		flex-direction: column;
		height: 100%;
		box-sizing: border-box;
		padding: var(--space-6);
		gap: var(--space-stack-md);
	}
</style>
