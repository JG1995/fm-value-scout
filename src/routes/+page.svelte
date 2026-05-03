<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";
	import { invoke } from "@tauri-apps/api/core";
	import { playerState } from "$lib/stores/players.svelte";

	let status = $state<"idle" | "loading" | "success" | "error">("idle");
	let playerCount = $state(0);
	let errorMessage = $state("");

	async function handleClick() {
		status = "loading";
		errorMessage = "";

		const filePath = await open({
			multiple: false,
			filters: [{ name: "CSV", extensions: ["csv"] }],
		});

		if (!filePath) {
			status = "idle";
			return;
		}

		try {
			const result = await invoke<{ players: unknown[]; currency: string }>("parse_csv", {
				path: filePath,
			});
			playerCount = result.players.length;
			playerState.data = { players: result.players, currency: result.currency };
			status = "success";
		} catch (err) {
			errorMessage = typeof err === "string" ? err : "Failed to parse CSV file";
			status = "error";
		}
	}
</script>

<div class="page">
	<div
		class="upload-card"
		class:loading={status === "loading"}
		class:success={status === "success"}
		class:error={status === "error"}
		role="button"
		tabindex="0"
		onclick={handleClick}
		onkeydown={(e) => e.key === "Enter" && handleClick()}
	>
		<div class="upload-inner">
			<svg
				class="upload-icon"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
				aria-hidden="true"
			>
				<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
				<polyline points="17 8 12 3 7 8" />
				<line x1="12" y1="3" x2="12" y2="15" />
			</svg>

			<h1 class="title">Import Squad Export</h1>

			{#if status === "idle"}
				<p class="subtitle">Click to select your FM CSV export</p>
			{:else if status === "loading"}
				<p class="subtitle">Parsing file...</p>
			{:else if status === "success"}
				<p class="subtitle success-text">{playerCount} players imported</p>
			{:else if status === "error"}
				<p class="subtitle error-text">{errorMessage}</p>
			{/if}

			<span class="file-hint">CSV files only</span>
		</div>
	</div>
</div>

<style>
	.page {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		box-sizing: border-box;
	}

	.upload-card {
		background: var(--color-glass-bg);
		backdrop-filter: blur(var(--blur-panel));
		-webkit-backdrop-filter: blur(var(--blur-panel));
		border: 1px solid rgba(181, 205, 180, 0.2);
		border-radius: var(--radius-panel);
		padding: var(--space-6) 2.5rem;
		cursor: pointer;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease,
			background 0.2s ease;
	}

	.upload-card:hover {
		border-color: var(--color-gold);
		box-shadow:
			0 0 20px var(--color-gold-dim),
			0 0 40px var(--color-gold-glow);
	}

	.upload-card:focus {
		outline: 2px solid var(--color-gold);
		outline-offset: 2px;
	}

	.upload-card.loading {
		cursor: wait;
		border-color: var(--color-gold);
		box-shadow: 0 0 20px var(--color-gold-dim);
		animation: pulse 2s ease-in-out infinite;
	}

	.upload-card.success {
		border-color: var(--color-primary);
		box-shadow:
			0 0 20px rgba(181, 205, 180, 0.15),
			0 0 40px rgba(181, 205, 180, 0.2);
	}

	.upload-card.error {
		border-color: var(--color-error);
		box-shadow:
			0 0 20px rgba(255, 180, 171, 0.15),
			0 0 40px rgba(255, 180, 171, 0.2);
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.7;
		}
	}

	.upload-inner {
		border: 2px dashed var(--color-outline-variant);
		border-radius: var(--radius-md);
		padding: var(--space-stack-lg) 4rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-stack-sm);
		transition: border-color 0.2s ease;
	}

	.upload-card:hover .upload-inner {
		border-color: rgba(233, 195, 73, 0.6);
	}

	.upload-card.success .upload-inner {
		border-color: rgba(181, 205, 180, 0.6);
	}

	.upload-card.error .upload-inner {
		border-color: rgba(255, 180, 171, 0.6);
	}

	.upload-icon {
		color: var(--color-gold);
		margin-bottom: var(--space-2);
		transition: opacity 0.2s ease;
	}

	.upload-card:hover .upload-icon {
		opacity: 0.9;
	}

	.upload-card.success .upload-icon {
		color: var(--color-primary);
	}

	.upload-card.error .upload-icon {
		color: var(--color-error);
	}

	.title {
		font-size: var(--font-size-headline-md);
		font-weight: var(--font-weight-headline-md);
		color: var(--color-gold);
		margin: 0;
		text-align: center;
	}

	.subtitle {
		font-size: var(--font-size-body-md);
		font-weight: var(--font-weight-body-md);
		color: var(--color-on-surface-variant);
		margin: 0;
		text-align: center;
	}

	.success-text {
		color: var(--color-primary);
	}

	.error-text {
		color: var(--color-error);
	}

	.file-hint {
		font-size: var(--font-size-label-caps);
		font-weight: var(--font-weight-label-caps);
		text-transform: uppercase;
		letter-spacing: var(--font-letter-spacing-label-caps);
		color: var(--color-on-surface-variant);
		margin-top: var(--space-1);
	}
</style>
