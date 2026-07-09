<script lang="ts">
	import '../layout.css';
	import mimir_logo from '$lib/assets/mimir_logo.png';
	import { ModeWatcher } from 'mode-watcher';
	import Footer from '$lib/components/Footer.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<title>Mimir</title>
	<link rel="icon" href={mimir_logo} />
</svelte:head>

<ModeWatcher />
<header
	class="sticky top-0 z-50 w-full border-b border-border/40 bg-background/75 backdrop-blur-xl transition-colors duration-500"
>
	<div class="mx-auto flex h-12 max-w-360 items-center px-6">
		<a
			href="/"
			class="text-[14px] font-semibold text-foreground transition-colors hover:text-primary"
		>
			Mimir
		</a>

		<nav class="ml-auto flex items-center gap-6 text-[14px]">
			<a href="/docs" class="text-muted-foreground transition-colors hover:text-foreground">
				Documentation
			</a>
			<a href="/api" class="text-muted-foreground transition-colors hover:text-foreground">
				API Reference
			</a>
			<a
				href="https://github.com/TheChirag356/mimir"
				class="text-muted-foreground transition-colors hover:text-foreground"
			>
				GitHub
			</a>
		</nav>
	</div>
</header>

<main>
	{@render children()}
</main>

<Footer />
