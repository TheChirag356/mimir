<script lang="ts">
  import { page } from "$app/stores";
  import { auth, isLoggedIn } from "$lib/stores/auth";
  import { goto } from "$app/navigation";
  import "../app.css";
  import { player } from "$lib/stores/player";

  // Icons as inline SVG components — no icon library dependency needed
  // for this small set, keeps the bundle lean.
  const isActive = (path: string) => $page.url.pathname === path;
  const isPrefix = (path: string) => $page.url.pathname.startsWith(path);

  function formatTime(seconds: number): string {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = Math.floor(seconds % 60);
    if (h > 0) return `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
    return `${m}:${String(s).padStart(2, "0")}`;
  }
</script>

{#if !$isLoggedIn}
  <!-- Auth pages get no chrome at all -->
  <slot />
{:else}
<div class="flex h-screen bg-background overflow-hidden dark">

    <!-- ── Sidebar (desktop only) ───────────────────────────────── -->
    <aside class="hidden md:flex flex-col w-48 bg-sidebar shrink-0 border-r border-sidebar-border">

        <div data-tauri-drag-region class="h-8 w-full shrink-0"></div>
      <!-- Search -->
      <div class="px-3 pt-1 pb-2">
        <button class="flex items-center gap-2 w-full px-3 py-1.5 rounded-md
                       bg-sidebar-accent text-sidebar-foreground/60 text-sm
                       hover:text-sidebar-foreground transition-colors">
          <svg width="13" height="13" viewBox="0 0 16 16" fill="none">
            <circle cx="6.5" cy="6.5" r="5" stroke="currentColor" stroke-width="1.5"/>
            <path d="m10.5 10.5 3.5 3.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
          Search
        </button>
      </div>

      <!-- Primary nav -->
      <nav class="px-2 py-2 space-y-0.5">
        <a href="/"
           class="flex items-center gap-2.5 px-3 py-1.5 rounded-md text-sm transition-colors
                  {isActive('/') ? 'bg-sidebar-accent text-sidebar-foreground font-medium' : 'text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent/50'}">
          <svg width="15" height="15" viewBox="0 0 16 16" fill="none">
            <path d="M2 6.5L8 2l6 4.5V14H2V6.5Z" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/>
          </svg>
          Listen Now
        </a>
        <a href="/library"
           class="flex items-center gap-2.5 px-3 py-1.5 rounded-md text-sm transition-colors
                  {isPrefix('/library') ? 'bg-sidebar-accent text-sidebar-foreground font-medium' : 'text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent/50'}">
          <svg width="15" height="15" viewBox="0 0 16 16" fill="none">
            <rect x="2" y="2" width="3.5" height="12" rx="1" stroke="currentColor" stroke-width="1.4"/>
            <rect x="6.5" y="2" width="3.5" height="12" rx="1" stroke="currentColor" stroke-width="1.4"/>
            <rect x="11" y="2" width="3" height="12" rx="1" stroke="currentColor" stroke-width="1.4"/>
          </svg>
          Library
        </a>
        <a href="/browse"
           class="flex items-center gap-2.5 px-3 py-1.5 rounded-md text-sm transition-colors
                  {isPrefix('/browse') ? 'bg-sidebar-accent text-sidebar-foreground font-medium' : 'text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent/50'}">
          <svg width="15" height="15" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.4"/>
            <path d="M8 2v2M8 12v2M2 8h2M12 8h2" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
          </svg>
          Browse
        </a>
        <a href="/radio"
           class="flex items-center gap-2.5 px-3 py-1.5 rounded-md text-sm transition-colors
                  {isPrefix('/radio') ? 'bg-sidebar-accent text-sidebar-foreground font-medium' : 'text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent/50'}">
          <svg width="15" height="15" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="9" r="2" stroke="currentColor" stroke-width="1.4"/>
            <path d="M4.5 5.5a5 5 0 0 1 7 0M2.5 3.5a8 8 0 0 1 11 0" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
          </svg>
          Radio
        </a>
        <a href="/now-playing"
           class="flex items-center gap-2.5 px-3 py-1.5 rounded-md text-sm transition-colors
                  {isPrefix('/radio') ? 'bg-sidebar-accent text-sidebar-foreground font-medium' : 'text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent/50'}">
          <svg width="15" height="15" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="9" r="2" stroke="currentColor" stroke-width="1.4"/>
            <path d="M4.5 5.5a5 5 0 0 1 7 0M2.5 3.5a8 8 0 0 1 11 0" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/>
          </svg>
          Now Playing
        </a>
      </nav>

      <!-- Divider -->
      <div class="mx-3 my-2 border-t border-sidebar-border"></div>

      <!-- My Library section -->
      <div class="px-3 mb-1">
        <p class="text-[10px] font-semibold uppercase tracking-widest text-sidebar-foreground/40 px-2 mb-1">
          My Library
        </p>
      </div>
      <nav class="px-2 space-y-0.5">
        {#each [
          { href: '/library/recent', label: 'Recently Added' },
          { href: '/library/audiobooks', label: 'Audiobooks' },
          { href: '/library/ebooks', label: 'E-Books' },
          { href: '/library/finished', label: 'Finished' },
          { href: '/library/wishlist', label: 'Wishlist' },
        ] as item}
          <a href={item.href}
             class="flex items-center px-3 py-1.5 rounded-md text-sm transition-colors
                    {isActive(item.href) ? 'bg-sidebar-accent text-sidebar-foreground font-medium' : 'text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent/50'}">
            {item.label}
          </a>
        {/each}
      </nav>

      <!-- Divider -->
      <div class="mx-3 my-2 border-t border-sidebar-border" />

      <!-- Playlists section -->
      <div class="px-3 mb-1">
        <p class="text-[10px] font-semibold uppercase tracking-widest text-sidebar-foreground/40 px-2 mb-1">
          Playlists
        </p>
      </div>
      <nav class="px-2 space-y-0.5 flex-1 overflow-y-auto">
        {#each ['Sci-Fi Series', 'Self Improvement', 'Thrillers'] as playlist}
          <a href="/playlist/{playlist.toLowerCase().replace(' ', '-')}"
             class="flex items-center px-3 py-1.5 rounded-md text-sm transition-colors
                    text-sidebar-foreground/70 hover:text-sidebar-foreground hover:bg-sidebar-accent/50">
            {playlist}
          </a>
        {/each}
      </nav>

      <!-- User -->
      <div class="mt-auto px-3 py-3 border-t border-sidebar-border">
        <button
          onclick={async () => { auth.logout(); await goto("/login"); }}
          class="flex items-center gap-2.5 w-full px-2 py-1.5 rounded-md
                 hover:bg-sidebar-accent/50 transition-colors group"
        >
          <div class="w-6 h-6 rounded-full bg-primary flex items-center justify-center
                      text-xs font-semibold text-primary-foreground shrink-0">
            {$auth.user?.username?.[0]?.toUpperCase() ?? 'U'}
          </div>
          <span class="text-sm text-sidebar-foreground truncate flex-1 text-left">
            {$auth.user?.username ?? 'User'}
          </span>
          <!-- chevron -->
          <svg width="12" height="12" viewBox="0 0 16 16" fill="none"
               class="text-sidebar-foreground/30 group-hover:text-sidebar-foreground/60 transition-colors">
            <path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
      </div>
    </aside>

    <!-- ── Main area ─────────────────────────────────────────────── -->
    <div class="flex flex-col flex-1 min-w-0">

      <!-- Scrollable content -->
      <main class="flex-1 overflow-y-auto pb-24 md:pb-20">
        <slot />
      </main>

      <!-- ── Mini-player (shown when something is playing) ─────── -->
      {#if $player.item}
      <div class="fixed bottom-0 left-0 right-0 md:left-48
                  border-t border-border bg-card/90 backdrop-blur-xl z-40">
        <div class="flex items-center gap-3 px-4 py-2.5">

          <div class="w-10 h-10 rounded bg-zinc-800 shrink-0 overflow-hidden">
            <div class="w-full h-full bg-gradient-to-br from-zinc-600 to-zinc-800" />
          </div>

          <a href="/now-playing" class="flex-1 min-w-0 hover:opacity-80 transition-opacity">
            <p class="text-sm font-medium text-foreground truncate">{$player.item.title}</p>
            <p class="text-xs text-muted-foreground truncate">{$player.item.author}</p>
          </a>

          <div class="flex items-center gap-3 shrink-0">
            <button onclick={() => player.skipBack(30)}
                    class="text-foreground/70 hover:text-foreground transition-colors">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
                <path d="M11 17l-5-5 5-5" stroke="currentColor" stroke-width="1.8"
                      stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M18 17l-5-5 5-5" stroke="currentColor" stroke-width="1.8"
                      stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>

            <button
              onclick={() => $player.isPlaying ? player.pause() : player.resume()}
              class="w-8 h-8 rounded-full bg-foreground flex items-center justify-center
                     hover:bg-foreground/90 transition-colors">
              {#if $player.isPlaying}
                <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
                  <rect x="3" y="2" width="3.5" height="12" rx="1" fill="var(--background)"/>
                  <rect x="9.5" y="2" width="3.5" height="12" rx="1" fill="var(--background)"/>
                </svg>
              {:else}
                <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
                  <path d="M4 2l8 6-8 6V2z" fill="var(--background)"/>
                </svg>
              {/if}
            </button>

            <button onclick={() => player.skipForward(30)}
                    class="text-foreground/70 hover:text-foreground transition-colors">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
                <path d="M13 7l5 5-5 5" stroke="currentColor" stroke-width="1.8"
                      stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M6 7l5 5-5 5" stroke="currentColor" stroke-width="1.8"
                      stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </button>
          </div>

          <div class="hidden md:flex items-center gap-2 w-48 shrink-0">
            <span class="text-xs text-muted-foreground tabular-nums">
              {formatTime($player.currentTime)}
            </span>
            <div class="flex-1 h-1 bg-muted rounded-full overflow-hidden">
              <div class="h-full bg-primary rounded-full transition-all"
                   style="width: {$player.duration > 0 ? ($player.currentTime / $player.duration) * 100 : 0}%" />
            </div>
            <span class="text-xs text-muted-foreground tabular-nums">
              {formatTime($player.duration)}
            </span>
          </div>

        </div>
        <div class="md:hidden h-0.5 bg-muted">
          <div class="h-full bg-primary transition-all"
               style="width: {$player.duration > 0 ? ($player.currentTime / $player.duration) * 100 : 0}%" />
        </div>
      </div>
      {/if}

      <!-- ── Bottom tab bar (mobile only) ─────────────────────── -->
      <nav class="md:hidden fixed bottom-0 left-0 right-0 z-50
                  bg-sidebar border-t border-sidebar-border
                  flex items-center justify-around px-2 py-2 safe-area-pb">
        {#each [
          { href: '/', label: 'Listen Now', icon: 'M2 6.5L8 2l6 4.5V14H2V6.5Z' },
          { href: '/library', label: 'Library', icon: 'M3 3h2.5v10H3zM7 3h2.5v10H7zM11.5 3H14v10h-2.5z' },
          { href: '/browse', label: 'Browse', icon: 'M8 2a6 6 0 1 0 0 12A6 6 0 0 0 8 2z' },
          { href: '/radio', label: 'Radio', icon: 'M4.5 5.5a5 5 0 0 1 7 0M2.5 3.5a8 8 0 0 1 11 0M8 11a2 2 0 1 0 0-4 2 2 0 0 0 0 4z' },
          { href: '/search', label: 'Search', icon: 'M6.5 11.5a5 5 0 1 0 0-10 5 5 0 0 0 0 10zM14 14l-3-3' },
        ] as tab}
          <a href={tab.href}
             class="flex flex-col items-center gap-0.5 px-3 py-1 rounded-md transition-colors
                    {isActive(tab.href) || (tab.href !== '/' && isPrefix(tab.href))
                      ? 'text-primary'
                      : 'text-sidebar-foreground/50 hover:text-sidebar-foreground'}">
            <svg width="22" height="22" viewBox="0 0 16 16" fill="none">
              <path d={tab.icon} stroke="currentColor" stroke-width="1.4"
                    stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <span class="text-[10px] font-medium">{tab.label}</span>
          </a>
        {/each}
      </nav>

    </div>
  </div>
{/if}
