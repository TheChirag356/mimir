<script lang="ts">
  import { player } from "$lib/stores/player";
  import { goto } from "$app/navigation";

  const speeds = [0.5, 0.75, 1, 1.25, 1.5, 2];

  function formatTime(seconds: number): string {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = Math.floor(seconds % 60);
    if (h > 0) return `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
    return `${m}:${String(s).padStart(2, "0")}`;
  }

  function handleSeek(e: Event) {
    const input = e.target as HTMLInputElement;
    player.seek(Number(input.value));
  }

  const progress = $derived(
    $player.duration > 0 ? ($player.currentTime / $player.duration) * 100 : 0
  );
</script>

<div class="flex h-full bg-background text-foreground overflow-hidden">

  <!-- ── Left: player panel ─────────────────────────────────────── -->
  <div class="flex-1 flex flex-col items-center justify-center px-12 py-8 min-w-0">

    <button
      onclick={() => goto("/")}
      class="self-start flex items-center gap-2 text-muted-foreground
             hover:text-foreground transition-colors mb-8 text-sm"
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
        <path d="M10 4L6 8l4 4" stroke="currentColor" stroke-width="1.6"
              stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      Now Playing
    </button>

    {#if !$player.item}
      <div class="flex flex-col items-center gap-3 text-center">
        <div class="w-20 h-20 rounded-2xl bg-muted flex items-center justify-center">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="1.5"/>
            <path d="M10 8l6 4-6 4V8z" fill="currentColor"/>
          </svg>
        </div>
        <p class="text-base font-medium">Nothing playing</p>
        <p class="text-sm text-muted-foreground">Pick a book from your library to start</p>
        <button
          onclick={() => goto("/")}
          class="mt-2 px-4 py-2 bg-primary text-primary-foreground rounded-full
                 text-sm font-medium hover:opacity-90 transition-opacity"
        >
          Go to Library
        </button>
      </div>

    {:else}

      <!-- Cover art -->
      <div class="w-56 h-56 rounded-2xl bg-zinc-800 shadow-2xl mb-8 overflow-hidden shrink-0">
        <div class="w-full h-full bg-gradient-to-br from-zinc-600 to-zinc-900
                    flex items-center justify-center">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" class="text-white/20">
            <path d="M9 19V6l12-3v13" stroke="currentColor" stroke-width="1.2"/>
            <circle cx="6" cy="18" r="3" stroke="currentColor" stroke-width="1.2"/>
            <circle cx="18" cy="16" r="3" stroke="currentColor" stroke-width="1.2"/>
          </svg>
        </div>
      </div>

      <!-- Title + actions -->
      <div class="w-full max-w-sm flex items-start justify-between mb-6">
        <div class="flex-1 min-w-0">
          <h1 class="text-xl font-bold text-foreground tracking-tight truncate">
            {$player.item.title}
          </h1>
          <p class="text-sm text-muted-foreground mt-0.5 truncate">
            {$player.item.author}
          </p>
        </div>
        <div class="flex gap-3 ml-4 shrink-0">
          <button class="text-muted-foreground hover:text-foreground transition-colors">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
              <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" stroke="currentColor"
                    stroke-width="1.5" stroke-linecap="round"/>
              <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"
                    stroke="currentColor" stroke-width="1.5"/>
            </svg>
          </button>
          <button class="text-muted-foreground hover:text-primary transition-colors">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
              <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78L12 21.23l8.84-8.84a5.5 5.5 0 0 0 0-7.78z"
                    stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- Scrubber -->
      <div class="w-full max-w-sm mb-5">
        <div class="relative h-1 bg-muted rounded-full mb-2">
          <div class="h-full bg-primary rounded-full transition-all"
               style="width: {progress}%" />
          <input
            type="range"
            min="0"
            max={$player.duration}
            value={$player.currentTime}
            oninput={handleSeek}
            class="absolute inset-0 w-full opacity-0 cursor-pointer h-full"
          />
        </div>
        <div class="flex justify-between text-xs text-muted-foreground tabular-nums">
          <span>{formatTime($player.currentTime)}</span>
          <span>-{formatTime($player.duration - $player.currentTime)}</span>
        </div>
      </div>

      <!-- Transport controls -->
      <div class="flex items-center gap-6 mb-6">
        <button
          onclick={() => {
            const idx = speeds.indexOf($player.speed);
            player.setSpeed(speeds[(idx + 1) % speeds.length]);
          }}
          class="w-9 h-9 flex items-center justify-center text-muted-foreground
                 hover:text-foreground transition-colors text-xs font-semibold tabular-nums"
        >
          {$player.speed}×
        </button>

        <button
          onclick={() => player.skipBack(30)}
          class="text-foreground/80 hover:text-foreground transition-colors"
        >
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
            <path d="M12 5V1L7 6l5 5V7a7 7 0 1 1-7 7h-2a9 9 0 1 0 9-9z" fill="currentColor"/>
            <text x="12" y="16.5" text-anchor="middle" font-size="5.5" font-weight="600"
                  fill="var(--background)" font-family="system-ui">30</text>
          </svg>
        </button>

        <button
          onclick={() => $player.isPlaying ? player.pause() : player.resume()}
          class="w-16 h-16 rounded-full bg-foreground flex items-center justify-center
                 hover:bg-foreground/90 transition-colors shadow-lg"
        >
          {#if $player.isPlaying}
            <svg width="20" height="20" viewBox="0 0 16 16" fill="none">
              <rect x="3" y="2" width="3.5" height="12" rx="1.2" fill="var(--background)"/>
              <rect x="9.5" y="2" width="3.5" height="12" rx="1.2" fill="var(--background)"/>
            </svg>
          {:else}
            <svg width="20" height="20" viewBox="0 0 16 16" fill="none">
              <path d="M4 2l10 6-10 6V2z" fill="var(--background)"/>
            </svg>
          {/if}
        </button>

        <button
          onclick={() => player.skipForward(30)}
          class="text-foreground/80 hover:text-foreground transition-colors"
        >
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none">
            <path d="M12 5V1l5 5-5 5V7a7 7 0 1 0 7 7h2a9 9 0 1 1-9-9z" fill="currentColor"/>
            <text x="12" y="16.5" text-anchor="middle" font-size="5.5" font-weight="600"
                  fill="var(--background)" font-family="system-ui">30</text>
          </svg>
        </button>

        <button class="w-9 h-9 flex items-center justify-center
                       text-muted-foreground hover:text-foreground transition-colors">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="1.5"/>
            <path d="M12 7v5l3 3" stroke="currentColor" stroke-width="1.5"
                  stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      </div>

      <!-- Volume -->
      <div class="flex items-center gap-3 w-full max-w-sm">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none"
             class="text-muted-foreground shrink-0">
          <path d="M11 5L6 9H2v6h4l5 4V5z" stroke="currentColor" stroke-width="1.5"
                stroke-linejoin="round"/>
        </svg>
        <div class="flex-1 relative h-1 bg-muted rounded-full">
          <div class="h-full bg-foreground/60 rounded-full"
               style="width: {$player.volume * 100}%" />
          <input
            type="range" min="0" max="1" step="0.01"
            value={$player.volume}
            oninput={(e) => player.setVolume(Number((e.target as HTMLInputElement).value))}
            class="absolute inset-0 w-full opacity-0 cursor-pointer h-full"
          />
        </div>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none"
             class="text-muted-foreground shrink-0">
          <path d="M11 5L6 9H2v6h4l5 4V5z" stroke="currentColor" stroke-width="1.5"
                stroke-linejoin="round"/>
          <path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"
                stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </div>

    {/if}
  </div>

  <!-- ── Right: chapter panel ───────────────────────────────────── -->
  {#if $player.item}
    <div class="w-72 border-l border-border flex flex-col shrink-0">

      <!-- Read / Listen toggle -->
      <div class="px-4 pt-4 pb-3 border-b border-border shrink-0">
        <div class="flex items-center gap-1 bg-muted rounded-lg p-1">
          <button
            onclick={() => goto(`/reader/${$player.item?.id}`)}
            class="flex-1 py-1.5 rounded-md text-sm font-medium
                   text-muted-foreground hover:text-foreground transition-colors"
          >
            Read
          </button>
          <button
            class="flex-1 py-1.5 rounded-md text-sm font-medium
                   bg-card text-foreground shadow-sm"
          >
            Listen
          </button>
        </div>
      </div>

      <!-- Cover -->
      <div class="mx-4 mt-4 aspect-square rounded-xl bg-zinc-800 overflow-hidden shrink-0">
        <div class="w-full h-full bg-gradient-to-br from-zinc-600 to-zinc-900" />
      </div>

      <!-- Reading progress -->
      <div class="px-4 mt-4 mb-4 shrink-0">
        <div class="flex items-center justify-between text-sm mb-2">
          <span class="text-muted-foreground">Listening Progress</span>
          <span class="text-primary font-medium">{Math.round(progress)}%</span>
        </div>
        <div class="h-1 bg-muted rounded-full overflow-hidden">
          <div class="h-full bg-primary rounded-full transition-all"
               style="width: {progress}%" />
        </div>
      </div>

      <!-- Chapters -->
      <p class="text-xs font-semibold uppercase tracking-widest text-muted-foreground px-4 mb-2 shrink-0">
        Chapters
      </p>
      <div class="flex-1 overflow-y-auto px-2 space-y-0.5 pb-4">
        {#each Array(6) as _, i}
          <div class="flex items-center gap-2.5 px-3 py-2 rounded-lg transition-colors
                      {i === 2 ? 'bg-accent text-foreground' : 'text-muted-foreground hover:bg-accent/50'}
                      cursor-pointer">
            {#if i < 2}
              <svg width="12" height="12" viewBox="0 0 16 16" fill="none"
                   class="text-muted-foreground/50 shrink-0">
                <path d="M3 8l4 4 6-6" stroke="currentColor" stroke-width="1.8"
                      stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            {:else if i === 2}
              <div class="w-3 h-3 rounded-full bg-primary shrink-0" />
            {:else}
              <span class="w-3 text-center text-xs text-muted-foreground/50 shrink-0">{i + 1}</span>
            {/if}
            <span class="text-sm {i === 2 ? 'font-medium' : ''}">Chapter {i + 1}</span>
          </div>
        {/each}
      </div>

    </div>
  {/if}

</div>
