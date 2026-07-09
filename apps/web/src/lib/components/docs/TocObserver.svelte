<script lang="ts">
	import { onMount } from 'svelte';

	import { toc } from '$lib/docs/toc';

	onMount(() => {
		const headings = [...document.querySelectorAll('main h2, main h3')];

		toc.set(
			headings.map((heading) => ({
				id: heading.id,
				text: heading.textContent ?? '',
				level: Number(heading.tagName.substring(1))
			}))
		);
	});
</script>
