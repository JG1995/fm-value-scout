<script lang="ts">
	const { children } = $props();
</script>

<div class="layout">
	{@render children()}
</div>

<style>
	:global(html, body) {
		margin: 0;
		padding: 0;
	}

	:global(body) {
		background: #000000;
		color: #e2e2e2;
		font-family: "Space Grotesk", sans-serif;
	}

	.layout {
		min-height: 100vh;
	}
</style>
