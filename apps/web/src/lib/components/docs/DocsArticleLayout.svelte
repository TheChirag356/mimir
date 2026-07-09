<script lang="ts">
	import DocsSidebar from './DocsSidebar.svelte';
	import DocsToc from './DocsToc.svelte';

	let { children } = $props();
</script>

<div class="mx-auto flex max-w-screen-2xl">
	<DocsSidebar />

	<main class="min-w-0 flex-1">
		<div class="mx-auto flex max-w-7xl">
			<div class="min-w-0 flex-1 px-8 py-14 lg:px-16">
				{@render children()}
			</div>

			<DocsToc />
		</div>
	</main>
</div>
