<script lang="ts">
	interface Props {
		variant?: "primary" | "secondary";
		onclick?: (e: MouseEvent) => void;
		children: import("svelte").Snippet;
		class?: string;
	}

	let { variant = "primary", onclick, children, class: className = "" }: Props = $props();
</script>

<button class="btn btn-{variant} {className}" {onclick}>
	{@render children()}
</button>

<style>
	.btn {
		font-family: var(--font-family);
		font-size: var(--font-size-body-md);
		font-weight: 600;
		padding: var(--space-3) var(--space-6);
		border-radius: var(--radius-button);
		cursor: pointer;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease,
			background 0.2s ease,
			opacity 0.2s ease;
	}

	.btn-primary {
		background: var(--color-gold);
		color: var(--color-on-secondary);
		border: none;
	}

	.btn-primary:hover {
		opacity: 0.9;
		box-shadow:
			0 0 20px var(--color-gold-dim),
			0 0 40px var(--color-gold-glow);
	}

	.btn-primary:focus {
		outline: 2px solid var(--color-gold);
		outline-offset: 2px;
	}

	.btn-secondary {
		background: transparent;
		color: var(--color-gold);
		border: 1px solid var(--color-gold);
	}

	.btn-secondary:hover {
		background: var(--color-glass-bg);
		backdrop-filter: blur(var(--blur-panel));
		box-shadow:
			0 0 20px var(--color-gold-dim),
			0 0 40px var(--color-gold-glow);
	}

	.btn-secondary:focus {
		outline: 2px solid var(--color-gold);
		outline-offset: 2px;
	}
</style>
