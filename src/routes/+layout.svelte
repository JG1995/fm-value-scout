<script lang="ts">
	import "../styles/tokens.css";
	import { sidebar } from "$lib/design-tokens";
	import Sidebar from "$lib/components/Sidebar.svelte";

	const { children } = $props();
</script>

<div class="app">
	<Sidebar />
	<div class="content" style:margin-left={sidebar.width}>
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
		height: 100%;
		overflow: hidden;
	}
</style>
