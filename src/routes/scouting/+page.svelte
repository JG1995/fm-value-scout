<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { playerState } from "$lib/stores/players.svelte";
	import type { Player } from "$lib/types/player";
	import type { Archetype, ArchetypeFitScore } from "$lib/types/archetypes";
	import { ScoutingTable } from "$lib/components";

	const players = $derived((playerState.data?.players ?? []) as Player[]);
	const currency = $derived(playerState.data?.currency ?? "\u00A3");

	let archetypes = $state<Archetype[]>([]);
	let selectedArchetypeId = $state<string | null>(null);
	let scores = $state<Map<number, ArchetypeFitScore> | null>(null);
	let scoring = $state(false);

	// Load archetypes on mount
	$effect(() => {
		invoke<Archetype[]>("list_archetypes").then((list) => {
			archetypes = list;
		});
	});

	// Score when archetype changes
	$effect(() => {
		if (!selectedArchetypeId) {
			scores = null;
			return;
		}
		scoring = true;
		invoke<ArchetypeFitScore[]>("score_archetype_fit", {
			archetypeId: selectedArchetypeId,
		})
			.then((results) => {
				const map = new Map<number, ArchetypeFitScore>();
				for (const r of results) {
					map.set(r.playerUniqueId, r);
				}
				scores = map;
				scoring = false;
			})
			.catch(() => {
				scoring = false;
			});
	});

	function handleArchetypeChange(e: Event) {
		const select = e.target as HTMLSelectElement;
		selectedArchetypeId = select.value || null;
	}
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
			<div class="toolbar">
				<h1 class="title">Scouting</h1>
				<label class="archetype-label">
					<span class="label-text text-label-caps">Archetype</span>
					<select
						class="archetype-select"
						value={selectedArchetypeId ?? ""}
						onchange={handleArchetypeChange}
						disabled={scoring}
					>
						<option value="">-- None --</option>
						{#each archetypes as a}
							<option value={a.id}>{a.name} ({a.position})</option>
						{/each}
					</select>
				</label>
				{#if scoring}
					<span class="scoring-indicator text-body-md">Scoring...</span>
				{/if}
			</div>
			<ScoutingTable {players} {currency} {scores} />
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

	.toolbar {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.archetype-label {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.label-text {
		color: var(--color-on-surface-variant);
	}

	.archetype-select {
		background: var(--color-glass-bg);
		color: var(--color-on-surface);
		border: 1px solid rgba(181, 205, 180, 0.3);
		border-radius: var(--radius-button);
		padding: var(--space-2) var(--space-3);
		font-family: var(--font-family);
		font-size: var(--font-size-body-md);
		cursor: pointer;
		min-width: 240px;
	}

	.archetype-select:focus {
		outline: 2px solid var(--color-gold);
		outline-offset: 1px;
	}

	.scoring-indicator {
		color: var(--color-gold);
		animation: pulse 1s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.4;
		}
	}
</style>
