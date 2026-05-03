<script lang="ts">
	import type { Player } from "$lib/types/player";
	import { formatValue, formatWage } from "$lib/utils/format";

	let { players, currency }: { players: Player[]; currency: string } = $props();
</script>

<div class="table-wrapper">
	<table>
		<thead>
			<tr>
				<th class="text-label-caps">Name</th>
				<th class="text-label-caps">Nation</th>
				<th class="text-label-caps">Club</th>
				<th class="text-label-caps">Positions</th>
				<th class="text-label-caps">Age</th>
				<th class="text-label-caps">Height</th>
				<th class="text-label-caps">Transfer Value</th>
				<th class="text-label-caps">Wage</th>
			</tr>
		</thead>
		<tbody>
			{#if players.length === 0}
				<tr>
					<td colspan="8" class="empty-msg text-body-md">No players loaded</td>
				</tr>
			{:else}
				{#each players as player}
					<tr>
						<td class="text-body-md">{player.name}</td>
						<td class="text-body-md">{player.nation}</td>
						<td class="text-body-md">{player.club}</td>
						<td class="text-body-md">{player.position}</td>
						<td class="text-body-md">{player.age}</td>
						<td class="text-body-md">{player.height_cm} cm</td>
						<td class="text-body-md">{formatValue(player.transfer_value, currency)}</td>
						<td class="text-body-md">{formatWage(player.weekly_wage, currency)}</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>

<style>
	.table-wrapper {
		overflow: auto;
		max-height: calc(100vh - 200px);
		background: var(--color-glass-bg);
		backdrop-filter: blur(var(--blur-panel));
		-webkit-backdrop-filter: blur(var(--blur-panel));
		border: 1px solid rgba(181, 205, 180, 0.2);
		border-radius: var(--radius-panel);
	}

	table {
		width: 100%;
		border-collapse: collapse;
		font-family: var(--font-family);
	}

	th {
		position: sticky;
		top: 0;
		z-index: 1;
		background: var(--color-glass-bg);
		backdrop-filter: blur(var(--blur-panel));
		color: var(--color-gold);
		padding: var(--space-3) var(--space-4);
		text-align: left;
		border-bottom: 1px solid rgba(233, 195, 73, 0.3);
		white-space: nowrap;
	}

	td {
		color: var(--color-on-surface);
		padding: var(--space-2) var(--space-4);
		border-bottom: 1px solid rgba(181, 205, 180, 0.1);
		white-space: nowrap;
	}

	tbody tr:hover {
		background: var(--color-gold-dim);
	}

	tbody tr:last-child td {
		border-bottom: none;
	}

	.empty-msg {
		color: var(--color-on-surface-variant);
		padding: var(--space-6) var(--space-4);
		text-align: center;
	}
</style>
