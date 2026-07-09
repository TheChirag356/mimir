<script lang="ts">
	import { page } from '$app/state';
	import { toc } from '$lib/docs/toc';

	const items = $derived($toc);
</script>

<aside class="sticky top-20 hidden h-fit w-64 shrink-0 border-l border-border pl-6 xl:block">
	<p class="mb-4 text-sm font-medium">On this page</p>

	<nav class="space-y-2">
		{#each items as item}
			<a
				href={`${page.url.pathname}#${item.id}`}
				class:text-muted-foreground={item.level > 2}
				class="block text-sm transition-colors hover:text-primary"
				style={`padding-left:${(item.level - 2) * 12}px`}
			>
				{item.text}
			</a>
		{/each}
	</nav>
</aside>
