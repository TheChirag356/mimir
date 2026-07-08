<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { auth, isLoggedIn } from "$lib/stores/auth";
  import { goto } from "$app/navigation";
  import { player } from "$lib/stores/player";
  import { get } from "svelte/store";

  interface Item {
    id: string;
    library_id: string;
    title: string;
    media_type: string;
    duration_seconds: number | null;
  }

  let items = $state<Item[]>([]);
  let loading = $state(true);
  let error = $state("");
  let filter = $state<"all" | "audiobooks" | "ebooks">("all");
  let heroIndex = $state(0);

  const genres = ["Fiction", "Sci-Fi", "Thriller", "Self-Help", "History", "Biography", "Business"];

  onMount(async () => {
    if (!$isLoggedIn || !$auth.token) { goto("/login"); return; }
    try {
      const result = await invoke<Item[]>("get_all_items", {
        serverUrl: $auth.serverUrl,
        token: $auth.token,
      });
      items = Array.isArray(result) ? result : [];
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  const filtered = $derived(
    filter === "all" ? items :
    filter === "audiobooks" ? items.filter(i => i.media_type === "book") :
    items.filter(i => i.media_type === "ebook")
  );

  const heroItems = $derived(filtered.slice(0, 5));
  const recentItems = $derived(filtered.slice(0, 8));

  function formatDuration(seconds: number | null): string {
    if (!seconds) return "";
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    return h > 0 ? `${h}h ${m}m` : `${m}m`;
  }

  async function playItem(itemId: string) {
      const auth_val = get(auth); // need get() from svelte/store
      try {
        const detail = await invoke<{
          id: string;
          title: string;
          duration_seconds: number;
          audio_file_id: string;
          library_id: string;
        }>("get_item", {
          serverUrl: $auth.serverUrl,
          token: $auth.token,
          itemId,
        });

        if (!detail.audio_file_id) {
          console.warn("no audio file for this item");
          return;
        }

        await player.play({
          id: detail.id,
          title: detail.title,
          author: "", // populated once we have author metadata
          duration_seconds: detail.duration_seconds ?? 0,
          audio_file_id: detail.audio_file_id,
          library_id: detail.library_id,
        });

        goto("/now-playing");
      } catch (e) {
        console.error("playItem failed:", e);
      }
    }
</script>

<div class="min-h-full bg-background text-foreground">

  <!-- Filter tabs -->
  <div class="flex justify-end gap-2 px-8 pt-10 pb-4">
    {#each [['all','All'],['audiobooks','Audiobooks'],['ebooks','E-Books']] as [val, label]}
      <button
        onclick={() => filter = val as typeof filter}
        class="flex items-center gap-1.5 px-4 py-1.5 rounded-full text-sm font-medium
               border transition-colors
               {filter === val
                 ? 'bg-foreground text-background border-foreground'
                 : 'text-muted-foreground border-border hover:text-foreground hover:border-foreground/30'}"
      >
        {label}
      </button>
    {/each}
  </div>

  {#if loading}
    <div class="flex items-center justify-center h-64">
      <div class="w-6 h-6 border-2 border-primary border-t-transparent rounded-full animate-spin" />
    </div>

  {:else if error}
    <p class="px-8 py-4 text-sm text-destructive">{error}</p>

  {:else if items.length === 0}
    <!-- Empty state -->
    <div class="flex flex-col items-center justify-center h-96 gap-3">
      <div class="w-16 h-16 rounded-2xl bg-muted flex items-center justify-center">
        <svg width="28" height="28" viewBox="0 0 24 24" fill="none">
          <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" stroke="currentColor" stroke-width="1.5"/>
        </svg>
      </div>
      <p class="text-base font-medium text-foreground">No books yet</p>
      <p class="text-sm text-muted-foreground text-center max-w-xs">
        Add a folder to your library in mimir-core and scan it to see your books here.
      </p>
    </div>

  {:else}

    <!-- ── Hero carousel ──────────────────────────────────────── -->
    <div class="relative mx-6 rounded-xl overflow-hidden mb-8" style="height: 200px">
      {#each heroItems as item, i}
        <div
          class="absolute inset-0 transition-opacity duration-500
                 {i === heroIndex ? 'opacity-100' : 'opacity-0'}"
        >
          <!-- gradient placeholder since we don't have cover art yet -->
          <div class="absolute inset-0 bg-gradient-to-br from-zinc-700 to-zinc-900" />
          <div class="absolute inset-0 bg-gradient-to-r from-black/80 via-black/40 to-transparent" />

          <div class="absolute bottom-0 left-0 p-6">
            <p class="text-xs font-semibold uppercase tracking-widest text-primary mb-1">
              {item.media_type === 'book' ? 'Audiobook' : 'E-Book'}
            </p>
            <h2 class="text-2xl font-bold text-white mb-0.5 tracking-tight">{item.title}</h2>
            <div class="flex gap-3 mt-3">
                <button
                  onclick={() => playItem(heroItems[heroIndex].id)}
                  class="flex items-center gap-2 px-4 py-1.5 bg-white text-black
                         rounded-full text-sm font-semibold hover:bg-white/90 transition-colors"
                >
                  <svg width="10" height="12" viewBox="0 0 10 12" fill="currentColor">
                    <path d="M0 0l10 6-10 6V0z"/>
                  </svg>
                  Play
                </button>
              <button class="flex items-center gap-2 px-4 py-1.5 bg-white/20 text-white
                             rounded-full text-sm font-semibold hover:bg-white/30 transition-colors
                             backdrop-blur-sm border border-white/20">
                Add to Library
              </button>
            </div>
          </div>
        </div>
      {/each}

      <!-- carousel dots -->
      {#if heroItems.length > 1}
        <div class="absolute bottom-4 right-4 flex gap-1.5">
          {#each heroItems as _, i}
            <button
              onclick={() => heroIndex = i}
              class="w-1.5 h-1.5 rounded-full transition-colors
                     {i === heroIndex ? 'bg-white' : 'bg-white/30'}"
            />
          {/each}
        </div>
      {/if}

      <!-- prev/next -->
      {#if heroItems.length > 1}
        <button
          onclick={() => heroIndex = (heroIndex - 1 + heroItems.length) % heroItems.length}
          class="absolute left-3 top-1/2 -translate-y-1/2 w-7 h-7 rounded-full
                 bg-black/40 backdrop-blur-sm flex items-center justify-center
                 text-white hover:bg-black/60 transition-colors">
          <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
            <path d="M10 4L6 8l4 4" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
        <button
          onclick={() => heroIndex = (heroIndex + 1) % heroItems.length}
          class="absolute right-3 top-1/2 -translate-y-1/2 w-7 h-7 rounded-full
                 bg-black/40 backdrop-blur-sm flex items-center justify-center
                 text-white hover:bg-black/60 transition-colors">
          <svg width="12" height="12" viewBox="0 0 16 16" fill="none">
            <path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      {/if}
    </div>

    <!-- ── Continue Listening ─────────────────────────────────── -->
    <section class="mb-8">
      <div class="flex items-center justify-between px-8 mb-3">
        <h2 class="text-lg font-semibold text-foreground tracking-tight">Continue Listening</h2>
        <button class="text-sm font-medium text-primary hover:text-primary/80 transition-colors">
          See All
        </button>
      </div>
      <div class="flex gap-4 px-8 overflow-x-auto pb-2 scrollbar-none">
        {#each filtered.slice(0, 6) as item}
          <a href="/item/{item.id}"
             class="flex-none w-36 group cursor-pointer">
            <!-- Cover placeholder -->
            <div class="relative w-36 h-36 rounded-lg overflow-hidden mb-2 bg-zinc-800">
              <div class="w-full h-full bg-gradient-to-br from-zinc-600 to-zinc-800" />
              <!-- media type badge -->
              <div class="absolute top-1.5 right-1.5 flex gap-1">
                <div class="w-5 h-5 rounded bg-black/60 backdrop-blur-sm flex items-center justify-center">
                  <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
                    <path d="M8 2a6 6 0 1 0 0 12A6 6 0 0 0 8 2z" stroke="white" stroke-width="1.5"/>
                    <path d="M6.5 5.5L11 8l-4.5 2.5V5.5z" fill="white"/>
                  </svg>
                </div>
              </div>
              <!-- progress bar -->
              <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-black/40">
                <div class="h-full bg-primary" style="width: 35%" />
              </div>
            </div>
            <p class="text-sm font-medium text-foreground truncate">{item.title}</p>
            <p class="text-xs text-muted-foreground truncate mt-0.5">
              {formatDuration(item.duration_seconds)}
            </p>
          </a>
        {/each}
      </div>
    </section>

    <!-- ── Recently Added ─────────────────────────────────────── -->
    <section class="mb-8">
      <div class="flex items-center justify-between px-8 mb-3">
        <h2 class="text-lg font-semibold text-foreground tracking-tight">Recently Added</h2>
        <button class="text-sm font-medium text-primary hover:text-primary/80 transition-colors">
          See All
        </button>
      </div>
      <div class="flex gap-4 px-8 overflow-x-auto pb-2 scrollbar-none">
        {#each recentItems as item}
          <a href="/item/{item.id}"
             class="flex-none w-32 group cursor-pointer">
            <div class="relative w-32 h-32 rounded-lg overflow-hidden mb-2 bg-zinc-800">
              <div class="w-full h-full bg-gradient-to-br from-zinc-700 to-zinc-900" />
              <!-- New badge -->
              <div class="absolute top-1.5 left-1.5">
                <span class="px-1.5 py-0.5 bg-primary text-primary-foreground
                             text-[10px] font-bold rounded">New</span>
              </div>
              <div class="absolute top-1.5 right-1.5 flex gap-1">
                <div class="w-5 h-5 rounded bg-black/60 backdrop-blur-sm flex items-center justify-center">
                  <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
                    <path d="M8 2a6 6 0 1 0 0 12A6 6 0 0 0 8 2z" stroke="white" stroke-width="1.5"/>
                    <path d="M6.5 5.5L11 8l-4.5 2.5V5.5z" fill="white"/>
                  </svg>
                </div>
              </div>
            </div>
            <p class="text-sm font-medium text-foreground truncate">{item.title}</p>
            <p class="text-xs text-muted-foreground truncate mt-0.5">
              {formatDuration(item.duration_seconds)}
            </p>
          </a>
        {/each}
      </div>
    </section>

    <!-- ── Browse by Genre ────────────────────────────────────── -->
    <section class="px-8 mb-8">
      <h2 class="text-lg font-semibold text-foreground tracking-tight mb-3">Browse by Genre</h2>
      <div class="flex flex-wrap gap-2">
        {#each genres as genre}
          <button
            class="flex items-center gap-2 px-4 py-2 rounded-full border border-border
                   text-sm text-foreground hover:border-primary/50 hover:text-primary
                   transition-colors"
          >
            {genre}
          </button>
        {/each}
      </div>
    </section>

  {/if}
</div>

<!-- hide scrollbars on the horizontal rows -->
<style>
  .scrollbar-none {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
  .scrollbar-none::-webkit-scrollbar {
    display: none;
  }
</style>
