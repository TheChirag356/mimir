<script lang="ts">
	import type { DocPage } from '$lib/docs/types';

	import DocsSidebarItem from './DocsSidebarItem.svelte';

	interface Props {
		title: string;
		pages: DocPage[];
	}

	let { title, pages }: Props = $props();

	let open = $state(true);
</script>

<div class="mb-8">
	<button
		type="button"
		onclick={() => (open = !open)}
		class="mb-3 flex w-full items-center px-4 text-left text-[11px] font-semibold uppercase tracking-[0.15em] text-muted-foreground"
	>
		{title}
	</button>

	{#if open}
		<div class="space-y-0.5">
			{#each pages as page}
				<DocsSidebarItem title={page.metadata.title} href={`/docs/${page.slug}`} />
			{/each}
		</div>
	{/if}
</div>
