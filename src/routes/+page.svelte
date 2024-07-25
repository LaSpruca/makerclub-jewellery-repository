<script lang="ts">
	import ViewCard from '$lib/components/ViewCard.svelte';
	import type { PageServerData } from './$types';
	import { items } from '$lib/stores';
	import Fuse from 'fuse.js';

	export let data: PageServerData;

	let showPublished = true;
	let searchText = '';

	$items = data.initial_items;

	$: withHidden = showPublished
		? $items.map((item, index) => ({ ...item, index }))
		: $items.map((item, index) => ({ ...item, index })).filter(({ published }) => !published);
	$: searchable = new Fuse(withHidden, { keys: ['title', 'description', 'username', 'nickname'] });
	$: searched =
		searchText.trim() === ''
			? withHidden.map(({ index }) => index)
			: searchable.search(searchText.trim()).map(({ item: { index } }) => index);

	$: console.log({ withHidden, searchable, searched });
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

<div class="align-center flex w-full justify-around py-10">
	{#if data.is_admin}
		<div class="form-control">
			<label class="label cursor-pointer">
				<span class="label-text pr-5">Show Published</span>
				<input type="checkbox" bind:checked={showPublished} class="checkbox" />
			</label>
		</div>
	{/if}
	<label class="input input-bordered flex items-center gap-2">
		<input type="text" class="grow" placeholder="Search" bind:value={searchText} />
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 16 16"
			fill="currentColor"
			class="h-4 w-4 opacity-70"
		>
			<path
				fill-rule="evenodd"
				d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z"
				clip-rule="evenodd"
			/>
		</svg>
	</label>
</div>

<div class="flex flex-wrap justify-center gap-10 p-10">
	{#each searched as index}
		<ViewCard {index} admin={data.is_admin} />
	{/each}
</div>
