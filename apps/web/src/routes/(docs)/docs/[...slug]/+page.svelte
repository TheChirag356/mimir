<script lang="ts">
	import type { PageData } from './$types';

	import Markdown from '$lib/components/docs/Markdown.svelte';
	import ArticleHeader from '$lib/components/docs/ArticleHeader.svelte';
	import DocsPagination from '$lib/components/docs/DocsPagination.svelte';
	import TocObserver from '$lib/components/docs/TocObserver.svelte';

	let { data }: { data: PageData } = $props();

	const Page = $derived(data.page.component);
</script>

<TocObserver />

<ArticleHeader metadata={data.page.metadata} />

<Markdown>
	<svelte:component this={Page} />
</Markdown>

<DocsPagination previous={data.pagination.previous} next={data.pagination.next} />
