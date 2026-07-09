<script lang="ts">
	import { getNavigation } from '$lib/docs/navigation';
	import DocsSidebarGroup from './DocsSidebarGroup.svelte';

	const navigation = getNavigation();
</script>

<aside class="h-[calc(100vh-56px)] w-[300px] shrink-0 overflow-y-auto border-r border-border">
	<div class="p-6">
		<h1 class="text-3xl font-semibold tracking-tight">Documentation</h1>

		<p class="mt-2 text-sm text-muted-foreground">Browse guides, tutorials and API references.</p>
	</div>

	{#each navigation as [category, pages]}
		<DocsSidebarGroup title={category} items={pages} />
	{/each}
</aside>
