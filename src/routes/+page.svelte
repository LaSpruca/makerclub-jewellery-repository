<script lang="ts">
	import ViewCard from '$lib/components/ViewCard.svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;

	let items = data.initial_items;
</script>

<div class="flex px-10">
	{#each items as item}
		<ViewCard {item} />
	{/each}
</div>
