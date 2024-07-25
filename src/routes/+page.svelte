<script lang="ts">
	import ViewCard from '$lib/components/ViewCard.svelte';
	import type { PageServerData } from './$types';
	import { items } from '$lib/stores';

	export let data: PageServerData;

	$items = data.initial_items;
</script>

<div class="flex w-full justify-between bg-slate-900 px-10 py-5">
	<img
		src="https://makeuoa.nz/content/images/2024/04/aumc-logo-w.svg"
		class="h-7"
		alt="MakerClub Logo"
	/>
	<h1>Maker Club Jewellery Repository {data.is_admin ? 'Admin' : ''}</h1>
</div>

<hr />

<div class="flex flex-wrap justify-center gap-10 p-10">
	{#each $items as _, index}
		<ViewCard {index} admin={data.is_admin} />
	{/each}
</div>
