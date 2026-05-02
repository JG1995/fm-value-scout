<script lang="ts">
	interface Props {
		children: import("svelte").Snippet;
		class?: string;
	}

	let { children, class: className = "" }: Props = $props();
</script>

<div class="glass-panel {className}">
	{@render children()}
</div>

<style>
	.glass-panel {
		background: var(--color-glass-bg);
		backdrop-filter: blur(var(--blur-panel));
		-webkit-backdrop-filter: blur(var(--blur-panel));
		border: 1px solid rgba(181, 205, 180, 0.2);
		border-radius: var(--radius-panel);
	}

	.glass-panel:hover {
		border-color: var(--color-gold);
		box-shadow:
			0 0 20px var(--color-gold-dim),
			0 0 40px var(--color-gold-glow);
	}
</style>
