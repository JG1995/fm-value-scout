<script lang="ts">
	import IconBase from "./IconBase.svelte";

	interface Props {
		class?: string;
	}

	let { class: className = "" }: Props = $props();
</script>

<IconBase class={className}>
	<path d="M3 3v16a2 2 0 0 0 2 2h16" />
	<path d="m19 9-5 5-4-4-3 3" />
</IconBase>
