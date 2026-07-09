<script lang="ts">
	import '../../layout.css';
	import DocsHeader from '$lib/components/docs/DocsHeader.svelte';
	import DocsCommand from '$lib/components/docs/DocsCommand.svelte';
	import DocsArticleLayout from '$lib/components/docs/DocsArticleLayout.svelte';
	import mimir_logo from '$lib/assets/mimir_logo.png';

	let { children } = $props();
</script>

<svelte:head>
	<title>Mimir Docs</title>
	<link rel="icon" href={mimir_logo} />
</svelte:head>

<DocsCommand />

<div class="min-h-screen bg-background">
	<DocsHeader />

	<DocsArticleLayout>
		{@render children()}
	</DocsArticleLayout>
</div>
