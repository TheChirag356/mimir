<script lang="ts">
	import { onMount } from 'svelte';

	import ThemeToggle from '$lib/components/ThemeToggle.svelte';
	import DocsSearchButton from './DocsSearchButton.svelte';

	import { commandOpen } from '$lib/docs/stores';

	import { IconBrandGithub } from '@tabler/icons-svelte';

	onMount(() => {
		const listener = (e: KeyboardEvent) => {
			if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
				e.preventDefault();
				commandOpen.set(true);
			}
		};

		window.addEventListener('keydown', listener);

		return () => {
			window.removeEventListener('keydown', listener);
		};
	});
</script>

<header class="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur-xl">
	<div class="mx-auto flex h-14 max-w-screen-2xl items-center px-6">
		<a href="/docs" class="mr-10 text-lg font-semibold tracking-tight"> Mimir Docs </a>

		<nav class="hidden items-center gap-8 lg:flex">
			<a href="/docs" class="text-sm text-muted-foreground transition-colors hover:text-foreground">
				Documentation
			</a>

			<a href="/api" class="text-sm text-muted-foreground transition-colors hover:text-foreground">
				API
			</a>

			<a href="/blog" class="text-sm text-muted-foreground transition-colors hover:text-foreground">
				Blog
			</a>
		</nav>

		<div class="ml-auto flex items-center gap-2">
			<DocsSearchButton />

			<a
				href="https://github.com/TheChirag356/mimir"
				class="rounded-md p-2 transition-colors hover:bg-muted"
			>
				<IconBrandGithub class="h-5 w-5" />
			</a>

			<ThemeToggle />
		</div>
	</div>
</header>
