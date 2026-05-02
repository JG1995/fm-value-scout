<script lang="ts">
	import "../styles/tokens.css";
	const { children } = $props();
</script>

<div class="layout">
	{@render children()}
</div>

<style>
	.layout {
		min-height: 100vh;
	}
</style>
