<script lang="ts">
	import { goto } from '$app/navigation';

	import * as Dialog from '$lib/components/ui/dialog';
	import * as Command from '$lib/components/ui/command';

	import { commandOpen } from '$lib/docs/stores';
	import { getSearchIndex } from '$lib/docs/search';

	const searchItems = getSearchIndex();

	function navigate(href: string) {
		commandOpen.set(false);
		goto(href);
	}
</script>

<Dialog.Root bind:open={$commandOpen}>
	<Dialog.Content class="overflow-hidden p-0 sm:max-w-xl">
		<Command.Root>
			<Command.Input placeholder="Search documentation..." />

			<Command.List>
				<Command.Empty>No results found.</Command.Empty>

				<Command.Group heading="Documentation">
					{#each searchItems as item}
						<Command.Item value={item.title} onSelect={() => navigate(item.href)}>
							<div class="flex flex-col">
								<span>
									{item.title}
								</span>

								<span class="text-xs text-muted-foreground">
									{item.description}
								</span>
							</div>
						</Command.Item>
					{/each}
				</Command.Group>
			</Command.List>
		</Command.Root>
	</Dialog.Content>
</Dialog.Root>
