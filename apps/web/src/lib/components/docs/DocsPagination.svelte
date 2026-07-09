<script lang="ts">
	import { IconChevronLeft, IconChevronRight } from '@tabler/icons-svelte';
	import type { DocPage } from '$lib/docs/types';

	interface Props {
		previous?: DocPage;
		next?: DocPage;
	}

	let { previous, next }: Props = $props();
</script>

<nav class="mt-16 grid gap-4 border-t border-border pt-8 sm:grid-cols-2">
	{#if previous}
		<a
			href={`/docs/${previous.slug}`}
			class="group rounded-lg border border-border p-5 transition-colors hover:border-primary"
		>
			<div class="flex items-center gap-2 text-sm text-muted-foreground">
				<IconChevronLeft class="h-4 w-4" />
				Previous
			</div>

			<p class="mt-2 font-medium transition-colors group-hover:text-primary">
				{previous.metadata.title}
			</p>
		</a>
	{:else}
		<div></div>
	{/if}

	{#if next}
		<a
			href={`/docs/${next.slug}`}
			class="group rounded-lg border border-border p-5 text-right transition-colors hover:border-primary"
		>
			<div class="flex items-center justify-end gap-2 text-sm text-muted-foreground">
				Next
				<IconChevronRight class="h-4 w-4" />
			</div>

			<p class="mt-2 font-medium transition-colors group-hover:text-primary">
				{next.metadata.title}
			</p>
		</a>
	{/if}
</nav>
