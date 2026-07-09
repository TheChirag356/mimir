<script lang="ts">
	import { IconSearch } from '@tabler/icons-svelte';

	import { commandOpen } from '$lib/docs/stores';

	function openSearch() {
		commandOpen.set(true);
	}
</script>

<button
	type="button"
	onclick={openSearch}
	class="flex h-9 w-64 items-center rounded-lg border border-border bg-muted/40 px-3 transition-colors hover:bg-muted"
>
	<IconSearch class="mr-2 h-4 w-4 text-muted-foreground" />

	<span class="flex-1 text-left text-sm text-muted-foreground"> Search Documentation </span>

	<kbd class="rounded border bg-background px-1.5 py-0.5 text-[11px]"> ⌘K </kbd>
</button>
