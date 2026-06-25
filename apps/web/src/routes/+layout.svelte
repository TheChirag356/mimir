<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import { ModeWatcher } from 'mode-watcher';
	import Tabs from '$lib/components/ui/tabs/tabs.svelte';
	import ThemeToggle from '$lib/components/ThemeToggle.svelte';

	let { children } = $props();
</script>

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

<footer class="bg-zinc-50 dark:bg-zinc-900/50">
	<div class="mx-auto max-w-[1200px] px-6 py-12 md:py-16">
		<div class="grid grid-cols-2 gap-8 md:grid-cols-4 lg:grid-cols-5">
			<div class="col-span-2 lg:col-span-2">
				<a href="/" class="flex items-center gap-2">
					<!-- <BookAudio class="h-5 w-5 text-foreground" /> -->
					<span class="text-[15px] font-semibold tracking-tight text-foreground"> Mimir </span>
				</a>
				<p class="mt-4 text-[13px] leading-relaxed text-muted-foreground max-w-xs">
					The premier open-source, self-hosted streaming server for your audiobooks and podcasts.
				</p>
			</div>

			<div>
				<h3 class="text-[12px] font-semibold text-foreground uppercase tracking-wider mb-4">
					Project
				</h3>
				<ul class="space-y-3 text-[13px] text-muted-foreground">
					<li><a href="/about" class="hover:text-foreground hover:underline">About</a></li>
					<li><a href="/releases" class="hover:text-foreground hover:underline">Releases</a></li>
					<li><a href="/blog" class="hover:text-foreground hover:underline">Blog</a></li>
					<li><a href="/donate" class="hover:text-foreground hover:underline">Support Us</a></li>
				</ul>
			</div>

			<div>
				<h3 class="text-[12px] font-semibold text-foreground uppercase tracking-wider mb-4">
					Resources
				</h3>
				<ul class="space-y-3 text-[13px] text-muted-foreground">
					<li><a href="/docs" class="hover:text-foreground hover:underline">Documentation</a></li>
					<li><a href="/api" class="hover:text-foreground hover:underline">API Reference</a></li>
					<li>
						<a href="/guides" class="hover:text-foreground hover:underline">Deployment Guides</a>
					</li>
					<li><a href="/faq" class="hover:text-foreground hover:underline">FAQ</a></li>
				</ul>
			</div>

			<div>
				<h3 class="text-[12px] font-semibold text-foreground uppercase tracking-wider mb-4">
					Community
				</h3>
				<ul class="space-y-3 text-[13px] text-muted-foreground">
					<li>
						<a
							href="https://discord.gg/audiobookshelf"
							class="hover:text-foreground hover:underline">Discord</a
						>
					</li>
					<li>
						<a
							href="https://github.com/advplyr/audiobookshelf/discussions"
							class="hover:text-foreground hover:underline">Discussions</a
						>
					</li>
					<li>
						<a
							href="https://reddit.com/r/audiobookshelf"
							class="hover:text-foreground hover:underline">Reddit</a
						>
					</li>
					<li>
						<ThemeToggle />
					</li>
				</ul>
			</div>
		</div>

		<Separator class="my-8" />

		<div
			class="flex flex-col items-center justify-between gap-4 md:flex-row text-[12px] text-muted-foreground"
		>
			<p>
				&copy; {new Date().getFullYear()} Mimir Contributors. Open Source under the GPL-3.0 License.
			</p>
			<div class="flex gap-4">
				<a href="/privacy" class="hover:text-foreground hover:underline">Privacy Policy</a>
				<a href="/terms" class="hover:text-foreground hover:underline">Terms of Service</a>
			</div>
		</div>
	</div>
</footer>
