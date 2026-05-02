<script lang="ts">
	import IconBase from "./IconBase.svelte";

	interface Props {
		class?: string;
	}

	let { class: className = "" }: Props = $props();
</script>

<IconBase class={className}>
	<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
	<path d="M12 12v9" />
	<path d="m16 16-4-4-4 4" />
</IconBase>
