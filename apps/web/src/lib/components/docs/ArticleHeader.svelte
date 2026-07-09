<script lang="ts">
	import type { DocMetadata } from '$lib/docs/types';

	interface Props {
		metadata: DocMetadata;
	}

	let { metadata }: Props = $props();
</script>

<header class="mb-14 border-b border-border pb-10">
	<p class="text-sm font-medium text-primary">
		{metadata.category}
	</p>

	<h1 class="mt-3 text-5xl font-semibold tracking-tight lg:text-6xl">
		{metadata.title}
	</h1>

	<p class="mt-6 max-w-3xl text-xl leading-relaxed text-muted-foreground">
		{metadata.description}
	</p>
</header>
