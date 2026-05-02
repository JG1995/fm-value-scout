<script lang="ts">
	import "../styles/tokens.css";
	import Sidebar from "$lib/components/Sidebar.svelte";

	const { children } = $props();
</script>

<div class="app">
	<Sidebar />
	<div class="content">
		{@render children()}
	</div>
</div>

<style>
	.app {
		display: flex;
		height: 100dvh;
		width: 100%;
	}

	.content {
		flex: 1;
		margin-left: 72px; /* sidebar width: 48px icon + 24px padding */
		height: 100%;
		overflow: hidden;
	}
</style>
