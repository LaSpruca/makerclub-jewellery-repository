<script lang="ts">
	import { items } from '$lib/stores';

	export let index: number;
	export let admin: boolean;

	$: item = $items[index];
</script>

<div class="card w-96 bg-base-100 shadow-xl">
	<figure>
		<img src={item.thumbnail_url} alt="Shoes" class="h-36 max-w-full" />
	</figure>
	<div class="card-body">
		<h2 class="card-title">{item.title}</h2>
		<div class="badge badge-primary">
			<div class="avatar mr-1 h-full">
				<div class="h-full rounded-full">
					<img class="h-full" src={item.avatar_url} />
				</div>
			</div>
			{item.nickname ?? item.username}
		</div>
		<p>{item.description}</p>
		<div class="card-actions justify-end">
			{#if admin}
				{#if !item.published}
					<button
						class="btn btn-success"
						on:click={async () => {
							const result = await fetch(`/${item.id}/publish`, { method: 'POST' });
							if (result.status != 200) {
								alert(`Could not publish item ${await result.text()} (${result.status})`);
								return;
							}

							$items[index] = { ...item, published: true };
						}}
					>
						Accept
					</button>
					<button
						class="btn btn-error"
						on:click={async () => {
							if (confirm('Do you really want to delete this item. This action is irreversable')) {
								const result = await fetch(`/${item.id}/reject`, { method: 'POST' });
								if (result.status != 200) {
									alert(`Could not publish item ${await result.text()} (${result.status})`);
									return;
								}

								items.update((items) => {
									items.splice(index, 1);
									return items;
								});
							}
						}}
					>
						Reject
					</button>
				{:else}
					<button
						class="btn btn-error"
						on:click={async () => {
							const result = await fetch(`/${item.id}/unpublish`, { method: 'POST' });
							if (result.status != 200) {
								alert(`Could not publish item ${await result.text()} (${result.status})`);
								return;
							}

							$items[index] = { ...item, published: false };
						}}
					>
						Unpublish
					</button>
				{/if}
				<a class="btn btn-primary" href={item.svg_url} download="{item.title}.svg">
					Download design file
				</a>
			{:else}
				<a class="btn btn-primary" href={item.svg_url} download="{item.title}.svg">
					Get this design
				</a>
			{/if}
		</div>
	</div>
</div>
