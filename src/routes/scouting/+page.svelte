<script lang="ts">
	import { playerState } from "$lib/stores/players.svelte";
	import { GlassPanel } from "$lib/components";

	const firstPlayer = $derived(playerState.data?.players[0] ?? null);
</script>

<div class="page">
	{#if !firstPlayer}
		<div class="empty-state">
			<div class="empty-card">
				<h1 class="title">No Squad Data</h1>
				<p class="subtitle">Upload a CSV export first to view player data.</p>
				<a href="/" class="back-link">Upload CSV</a>
			</div>
		</div>
	{:else}
		<div class="data-container">
			<h1 class="title">First Player Data</h1>
			<GlassPanel>
				{#snippet children()}
					<div class="json-scroll">
						<pre class="json-output">{JSON.stringify(firstPlayer, null, 2)}</pre>
					</div>
				{/snippet}
			</GlassPanel>
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

	.json-scroll {
		overflow: auto;
		max-height: calc(100vh - 200px);
		padding: var(--space-4);
	}

	.json-output {
		font-size: var(--font-size-code-data);
		font-weight: var(--font-weight-code-data);
		line-height: var(--font-line-height-code-data);
		letter-spacing: var(--font-letter-spacing-code-data);
		color: var(--color-on-surface);
		margin: 0;
		white-space: pre-wrap;
		word-break: break-word;
	}
</style>
