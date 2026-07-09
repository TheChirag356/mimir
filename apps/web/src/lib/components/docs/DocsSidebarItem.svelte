<script lang="ts">
	import { page } from '$app/state';

	interface Props {
		title: string;
		href: string;
	}

	let { title, href }: Props = $props();

	const active = $derived(page.url.pathname === href);
</script>

<a
	{href}
	class="group relative flex items-center rounded-md py-2 pl-4 pr-3 text-[14px] transition-colors"
	class:text-foreground={active}
	class:text-muted-foreground={!active}
	class:font-medium={active}
>
	{#if active}
		<div class="absolute left-0 top-1/2 h-5 w-[2px] -translate-y-1/2 rounded-full bg-primary"></div>
	{/if}

	<span class="transition-colors group-hover:text-foreground">
		{title}
	</span>
</a>
