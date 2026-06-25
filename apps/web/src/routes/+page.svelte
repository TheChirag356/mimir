<script lang="ts">
	import { ChevronRight, Monitor, Smartphone, Copy, Check } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';

	let copied = $state(false);
	const dockerCommand = 'docker run -d -p 13378:8080 -v /path/to/audiobooks:/audiobooks mimir';

	function copyToClipboard() {
		navigator.clipboard.writeText(dockerCommand);
		copied = true;
		setTimeout(() => (copied = false), 2000);
	}
</script>

<div class="bg-background">
	<section class="mx-auto max-w-6xl px-6 pt-32 pb-24">
		<div class="mx-auto max-w-4xl text-center">
			<Badge
				variant="secondary"
				class="mb-6 rounded-full px-3 py-1 text-[12px] font-medium tracking-wide"
			>
				Open Source & Self-Hosted
			</Badge>

			<h1 class="text-5xl font-semibold tracking-[-0.06em] text-balance sm:text-7xl lg:text-8xl">
				Your audiobook library,
				<br />
				organized.
			</h1>

			<p class="mx-auto mt-8 max-w-2xl text-lg leading-relaxed text-muted-foreground">
				Host, stream, and manage your audiobooks and podcasts from your own server. Continue
				listening across devices, organize massive libraries, and keep complete ownership of your
				collection.
			</p>

			<div class="mt-10 flex flex-col justify-center gap-3 sm:flex-row">
				<Button size="lg" class="h-11 rounded-full px-6">Get Started</Button>

				<Button
					href="https://demo.audiobookshelf.org"
					variant="outline"
					size="lg"
					class="rounded-full px-8 text-[15px] font-medium border-muted"
				>
					Explore Live Demo <ChevronRight class="ml-1 h-4 w-4" />
				</Button>
			</div>

			<div
				class="mx-auto mt-16 w-full max-w-2xl rounded-xl border border-border/60 bg-zinc-50 p-4 text-left font-mono text-[13px] dark:bg-zinc-900/40"
			>
				<div class="flex items-center justify-between gap-4 overflow-x-auto whitespace-nowrap pl-1">
					<code class="text-foreground">{dockerCommand}</code>
					<button
						onclick={copyToClipboard}
						class="text-muted-foreground hover:text-foreground transition-colors p-1"
						aria-label="Copy code"
					>
						{#if copied}
							<Check class="h-4 w-4 text-emerald-500" />
						{:else}
							<Copy class="h-4 w-4" />
						{/if}
					</button>
				</div>
			</div>
		</div>

		<div class="mt-20 overflow-hidden rounded-2xl border bg-card shadow-sm">
			<img src="/dashboard.png" alt="Mimir Dashboard" class="w-full" />
		</div>
	</section>

	<section class="mx-auto max-w-6xl px-6 py-32">
		<div class="grid gap-12 md:grid-cols-2">
			<div>
				<p class="text-sm text-muted-foreground uppercase tracking-wide">Metadata</p>

				<h2 class="mt-3 text-4xl font-semibold tracking-tight text-balance">
					Automatic library organization.
				</h2>
			</div>

			<div>
				<p class="text-lg leading-relaxed text-muted-foreground">
					Mimir automatically extracts chapters, authors, narrators, series information, and cover
					artwork from your audiobook files, keeping even the largest collections organized with
					minimal effort.
				</p>
			</div>
		</div>

		<div class="mt-12 overflow-hidden rounded-2xl border bg-card">
			<img src="/metadata.png" alt="Metadata Management" class="w-full" />
		</div>
	</section>

	<section class="mx-auto max-w-6xl px-6 py-32 border-t">
		<div class="grid gap-12 md:grid-cols-2">
			<div>
				<p class="text-sm text-muted-foreground uppercase tracking-wide">Sync</p>

				<h2 class="mt-3 text-4xl font-semibold tracking-tight text-balance">
					Pick up where you left off.
				</h2>
			</div>

			<div>
				<p class="text-lg leading-relaxed text-muted-foreground">
					Your listening progress syncs instantly between devices. Start a chapter on desktop and
					continue from the exact position on your phone.
				</p>
			</div>
		</div>

		<div class="mt-12 overflow-hidden rounded-2xl border bg-card">
			<img src="/sync.png" alt="Sync" class="w-full" />
		</div>
	</section>

	<section class="mx-auto max-w-6xl px-6 py-32 border-t">
		<div class="max-w-2xl">
			<p class="text-sm text-muted-foreground uppercase tracking-wide">Available Everywhere</p>

			<h2 class="mt-3 text-4xl font-semibold tracking-tight">Listen on any device.</h2>
		</div>

		<div class="grid grid-cols-1 gap-6 sm:grid-cols-3">
			<div
				class="rounded-xl border border-border/40 bg-background p-6 flex flex-col justify-between transition-all duration-300 hover:border-border"
			>
				<div class="flex items-start gap-4">
					<Smartphone class="h-5 w-5 text-muted-foreground mt-0.5" />
					<div>
						<h4 class="font-medium text-[15px]">Apple iOS</h4>
						<p class="mt-1 text-[13px] text-muted-foreground leading-normal">
							Available via TestFlight distribution builds.
						</p>
					</div>
				</div>
				<a
					href="/docs/apps"
					class="mt-6 text-[13px] font-medium text-primary flex items-center hover:underline"
				>
					Join TestFlight <ChevronRight class="ml-0.5 h-3.5 w-3.5" />
				</a>
			</div>

			<div
				class="rounded-xl border border-border/40 bg-background p-6 flex flex-col justify-between transition-all duration-300 hover:border-border"
			>
				<div class="flex items-start gap-4">
					<Smartphone class="h-5 w-5 text-muted-foreground mt-0.5" />
					<div>
						<h4 class="font-medium text-[15px]">Google Android</h4>
						<p class="mt-1 text-[13px] text-muted-foreground leading-normal">
							Download directly via Google Play Store or raw APK packages.
						</p>
					</div>
				</div>
				<a
					href="/docs/apps"
					class="mt-6 text-[13px] font-medium text-primary flex items-center hover:underline"
				>
					Get on Google Play <ChevronRight class="ml-0.5 h-3.5 w-3.5" />
				</a>
			</div>

			<div
				class="rounded-xl border border-border/40 bg-background p-6 flex flex-col justify-between transition-all duration-300 hover:border-border"
			>
				<div class="flex items-start gap-4">
					<Monitor class="h-5 w-5 text-muted-foreground mt-0.5" />
					<div>
						<h4 class="font-medium text-[15px]">Desktop Clients</h4>
						<p class="mt-1 text-[13px] text-muted-foreground leading-normal">
							Cross-platform companion wrappers built for native execution.
						</p>
					</div>
				</div>
				<a
					href="/docs/apps"
					class="mt-6 text-[13px] font-medium text-primary flex items-center hover:underline"
				>
					Download Binaries <ChevronRight class="ml-0.5 h-3.5 w-3.5" />
				</a>
			</div>
		</div>
	</section>

	<section class="mx-auto max-w-4xl px-6 py-32 text-center">
		<h2 class="text-4xl font-semibold tracking-tight sm:text-5xl">
			Start building your audiobook library.
		</h2>

		<p class="mx-auto mt-6 max-w-2xl text-lg text-muted-foreground">
			Free, self-hosted, and designed specifically for audiobook collectors.
		</p>

		<div class="mt-10">
			<Button size="lg" class="rounded-full px-8">Get Started</Button>
		</div>
	</section>
</div>
