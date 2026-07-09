<script lang="ts">
	import { getNavigation } from '$lib/docs/navigation';

	const navigation = getNavigation();
</script>

<svelte:head>
	<title>Documentation | Mimir</title>
	<meta name="description" content="Learn how to install, configure, and use Mimir." />
</svelte:head>

<section class="border-b border-border pb-12">
	<p class="text-sm font-medium text-primary">Documentation</p>

	<h1 class="mt-3 text-5xl font-semibold tracking-tight lg:text-6xl">Mimir Documentation</h1>

	<p class="mt-6 max-w-3xl text-xl leading-relaxed text-muted-foreground">
		Everything you need to install, configure, and use Mimir. Browse the guides below to get
		started.
	</p>
</section>

<section class="mt-14 grid gap-6 sm:grid-cols-2">
	{#each navigation as group}
		<a
			href={`/docs/${group.pages[0].slug}`}
			class="group rounded-2xl border border-border bg-card p-6 transition-colors hover:border-primary"
		>
			<h2 class="text-xl font-semibold">
				{group.title}
			</h2>

			<p class="mt-2 text-sm text-muted-foreground">
				{group.pages.length}
				{group.pages.length === 1 ? ' article' : ' articles'}
			</p>

			<ul class="mt-6 space-y-2">
				{#each group.pages.slice(0, 3) as page}
					<li class="text-sm text-muted-foreground transition-colors group-hover:text-foreground">
						• {page.metadata.title}
					</li>
				{/each}

				{#if group.pages.length > 3}
					<li class="text-sm text-primary">
						+{group.pages.length - 3} more
					</li>
				{/if}
			</ul>
		</a>
	{/each}
</section>
