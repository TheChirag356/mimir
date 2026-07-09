<script lang="ts">
	import { getNavigation } from '$lib/docs/navigation';

	import DocsSidebarGroup from './DocsSidebarGroup.svelte';

	const navigation = getNavigation();
</script>

<aside
	class="sticky top-14 h-[calc(100vh-56px)] w-72 shrink-0 border-r border-border bg-background"
>
	<div class="flex h-full flex-col">
		<div class="border-b px-6 py-6">
			<h2 class="text-lg font-semibold tracking-tight">Documentation</h2>

			<p class="mt-2 text-sm text-muted-foreground">Mimir Documentation</p>
		</div>

		<div class="flex-1 overflow-y-auto px-4 py-6">
			{#each navigation as group}
				<DocsSidebarGroup {...group} />
			{/each}
		</div>
	</div>
</aside>
