<script lang="ts">
	import type { UploadRow, UserRow } from '$lib/server/db';

	export let item: UserRow & UploadRow;
</script>

<div class="card w-96 bg-base-100 shadow-xl">
	<figure>
		<img src={item.thumbnail_url} alt="Shoes" class="h-36 max-w-full" />
	</figure>
	<div class="card-body">
		<h2 class="card-title">{item.title}</h2>
		<div class="badge badge-success">
			<div class="avatar mr-1 h-full">
				<div class="h-full rounded-full">
					<img class="h-full" src={item.avatar_url} />
				</div>
			</div>
			{item.nickname ?? item.username}
		</div>
		<p>{item.description}</p>
		<div class="card-actions justify-end">
			<a class="btn btn-primary" href={item.svg_url} download="{item.title}.svg">Get this design</a>
		</div>
	</div>
</div>
