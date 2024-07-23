<script lang="ts">
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

{#each data.all_items.rows as item}
	<img src={item.thumbnail_url} alt="Image" />
	<img src={item.svg_url} alt="Image" />

	<p>{item.title}</p>
	<br />
	<p>{item.description}</p>
{/each}
