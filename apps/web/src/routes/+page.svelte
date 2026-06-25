<script lang="ts">
	import {
		ShieldCheck,
		Users,
		RefreshCw,
		Radio,
		HardDrive,
		Smartphone,
		Monitor,
		ChevronRight,
		Copy,
		Check
	} from '@lucide/svelte';
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

<div class="relative min-h-screen bg-background text-foreground selection:bg-primary/20">
	<section
		class="relative flex flex-col items-center justify-center px-6 pt-24 pb-20 text-center sm:pt-32 sm:pb-28 lg:px-8"
	>
		<div
			class="absolute inset-0 -z-10 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-zinc-100/70 via-background to-background dark:from-zinc-900/30"
		></div>

		<div class="mx-auto max-w-4xl">
			<Badge
				variant="secondary"
				class="mb-6 rounded-full px-3 py-1 text-[12px] font-medium tracking-wide"
			>
				Open Source & Self-Hosted
			</Badge>
			<h1 class="text-5xl font-bold tracking-tight text-foreground sm:text-7xl lg:text-8xl">
				Your audiobooks.<br />On your terms.
			</h1>
			<p class="mx-auto mt-8 max-w-2xl text-lg leading-relaxed text-muted-foreground sm:text-xl">
				A fully featured, self-hosted streaming server designed specifically for audiobooks and
				podcasts. Manage your library, track progress, and stream seamlessly across any device.
			</p>

			<div class="mt-10 flex flex-col items-center justify-center gap-4 sm:flex-row">
				<Button href="/docs" size="lg" class="rounded-full px-8 text-[15px] font-medium">
					Get Started
				</Button>
				<Button
					href="https://demo.audiobookshelf.org"
					variant="outline"
					size="lg"
					class="rounded-full px-8 text-[15px] font-medium"
				>
					Explore Live Demo <ChevronRight class="ml-1 h-4 w-4" />
				</Button>
			</div>
		</div>

		<div
			class="mx-auto mt-16 w-full max-w-2xl rounded-xl border border-border/60 bg-zinc-50 p-4 text-left font-mono text-[13px] dark:bg-zinc-900/40"
		>
			<div class="flex items-center justify-between border-b border-border/40 pb-2 mb-3">
				<div class="flex gap-1.5">
					<span class="h-3 w-3 rounded-full bg-zinc-300 dark:bg-zinc-700"></span>
					<span class="h-3 w-3 rounded-full bg-zinc-300 dark:bg-zinc-700"></span>
					<span class="h-3 w-3 rounded-full bg-zinc-300 dark:bg-zinc-700"></span>
				</div>
				<span class="text-[11px] text-muted-foreground font-sans">Quick Launch</span>
			</div>
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
	</section>

	<section class="mx-auto max-w-[1200px] px-6 py-20 sm:py-28">
		<div class="border-t border-border/40 pt-16">
			<h2 class="text-3xl font-bold tracking-tight sm:text-4xl mb-12">
				Engineered for collection management.
			</h2>

			<div class="grid grid-cols-1 gap-6 md:grid-cols-3">
				<div
					class="md:col-span-2 rounded-2xl border border-border/50 bg-card p-8 flex flex-col justify-between min-h-[300px]"
				>
					<div class="max-w-md">
						<HardDrive class="h-6 w-6 text-primary mb-4" />
						<h3 class="text-xl font-semibold">Automatic Metadata Extraction</h3>
						<p class="mt-2 text-[15px] leading-relaxed text-muted-foreground">
							Automatically parses embedded audio tags, extracts chapter breakdowns, and fetches
							rich descriptive assets from open sources including Audible and Open Library.
						</p>
					</div>
				</div>

				<div
					class="rounded-2xl border border-border/50 bg-card p-8 flex flex-col justify-between min-h-[300px]"
				>
					<div>
						<Users class="h-6 w-6 text-primary mb-4" />
						<h3 class="text-xl font-semibold">Multi-User Control</h3>
						<p class="mt-2 text-[15px] leading-relaxed text-muted-foreground">
							Create unique profiles for family members with independent listener progress queues
							and custom access permissions.
						</p>
					</div>
				</div>

				<div
					class="rounded-2xl border border-border/50 bg-card p-8 flex flex-col justify-between min-h-[300px]"
				>
					<div>
						<RefreshCw class="h-6 w-6 text-primary mb-4" />
						<h3 class="text-xl font-semibold">Active Position Sync</h3>
						<p class="mt-2 text-[15px] leading-relaxed text-muted-foreground">
							Pause your playback on the desktop browser and immediately resume from the exact
							sentence on your mobile client application.
						</p>
					</div>
				</div>

				<div
					class="md:col-span-2 rounded-2xl border border-border/50 bg-card p-8 flex flex-col justify-between min-h-[300px]"
				>
					<div class="max-w-md">
						<Radio class="h-6 w-6 text-primary mb-4" />
						<h3 class="text-xl font-semibold">Integrated Podcast Client</h3>
						<p class="mt-2 text-[15px] leading-relaxed text-muted-foreground">
							Subscribe to RSS feeds, parse episode manifests automatically, downmix raw media
							formats, and manage sequential playback tracking within a uniform dashboard interface.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="bg-zinc-50/60 dark:bg-zinc-900/20 py-20 sm:py-28">
		<div class="mx-auto max-w-[1200px] px-6">
			<div class="max-w-2xl mb-16">
				<h2 class="text-3xl font-bold tracking-tight sm:text-4xl">Carry your library anywhere.</h2>
				<p class="mt-4 text-[16px] text-muted-foreground leading-relaxed">
					Stream content smoothly via localized web apps, native desktop wrappers, or highly
					tailored mobile frameworks maintaining internal localized sync mappings.
				</p>
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
		</div>
	</section>
</div>
