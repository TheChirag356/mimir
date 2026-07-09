<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import { auth } from "$lib/stores/auth";
  import { player } from "$lib/stores/player";
  import ePub from "epubjs";
  import {
      registerReaderControls,
      unregisterReaderControls,
    } from "$lib/keyboard";

  const itemId = $page.params.id;

  let bookTitle = $state("Loading…");
  let bookAuthor = $state("");
  let totalPages = $state(0);
  let progressPercent = $state(0);
  let chapters = $state<{ label: string; href: string }[]>([]);
  let currentChapterIndex = $state(0);
  let fontSize = $state(16);
  let isDark = $state(true);
  let showChapters = $state(true);
  let timeRemaining = $state("");

  let book: any = null;
  let rendition: any = null;
  let readerEl: HTMLElement;

  // Add vertical scroll state
  let isVertical = $state(false);

  function toggleScrollMode() {
    isVertical = !isVertical;
    rendition?.flow(isVertical ? "scrolled-doc" : "paginated");
  }

  // Update onMount — after rendition.display(), add:
  // Register controls so global keyboard handler can call them
  registerReaderControls({
    prevPage: () => rendition?.prev(),
    nextPage: () => rendition?.next(),
    setScrollMode: (vertical: boolean) => {
      isVertical = vertical;
      rendition?.flow(vertical ? "scrolled-doc" : "paginated");
    },
  });

  // Update onDestroy:
  onDestroy(() => {
    unregisterReaderControls();
    rendition?.destroy();
    book?.destroy();
  });

  async function syncProgress(cfi: string, percent: number) {
    try {
      await invoke("sync_ebook_progress", {
        serverUrl: $auth.serverUrl,
        token: $auth.token,
        itemId,
        cfi,
        progressPercent: percent * 100,
      });
    } catch (e) {
      console.error("ebook sync failed:", e);
    }
  }

  onMount(async () => {
    const ebookUrl = `${$auth.serverUrl}/api/items/${itemId}/ebook?token=${$auth.token}`;

    const response = await fetch(ebookUrl);
    if (!response.ok) {
      console.error("failed to fetch ebook:", response.status);
      bookTitle = "Failed to load";
      return;
    }
    const arrayBuffer = await response.arrayBuffer();

    book = ePub(arrayBuffer);
    rendition = book.renderTo(readerEl, {
      width: "100%",
      height: "100%",
      flow: "paginated",
      spread: "none",
    });

    rendition.themes.register("mimir-dark", {
      body: {
        background: "#171717",
        color: "#f0f0f0",
        "font-family": "-apple-system, BlinkMacSystemFont, 'SF Pro Text', system-ui, sans-serif",
        "font-size": `${fontSize}px`,
        "line-height": "1.75",
        "max-width": "680px",
        margin: "0 auto",
        padding: "48px 32px",
      },
      p: { "margin-bottom": "1.2em" },
      h1: { "font-size": "1.6em", "font-weight": "700", "letter-spacing": "-0.02em" },
      h2: { "font-size": "1.3em", "font-weight": "600", "letter-spacing": "-0.015em" },
    });

    rendition.themes.register("mimir-light", {
      body: {
        background: "#fafafa",
        color: "#1a1a1a",
        "font-family": "-apple-system, BlinkMacSystemFont, 'SF Pro Text', system-ui, sans-serif",
        "font-size": `${fontSize}px`,
        "line-height": "1.75",
        "max-width": "680px",
        margin: "0 auto",
        padding: "48px 32px",
      },
      p: { "margin-bottom": "1.2em" },
    });

    rendition.themes.select(isDark ? "mimir-dark" : "mimir-light");

    await book.ready;
    const meta = book.package?.metadata;
    if (meta) {
      bookTitle = meta.title ?? bookTitle;
      bookAuthor = meta.creator ?? "";
    }

    const nav = await book.loaded.navigation;
    chapters = nav.toc.map((item: any) => ({
      label: item.label.trim(),
      href: item.href,
    }));

    await rendition.display();

    rendition.on("locationChanged", (location: any) => {
      const percent = book.locations.percentageFromCfi(location.start.cfi);
      progressPercent = Math.round((percent ?? 0) * 100);

      const chapter = book.navigation?.get(location.start.href);
      if (chapter) {
        currentChapterIndex = chapters.findIndex(
          (c: { label: string; href: string }) => c.href === location.start.href
        );
      }

      if (percent !== undefined && percent < 1) {
        const minutesLeft = Math.ceil((1 - percent) * 100) * 2;
        timeRemaining = minutesLeft > 60
          ? `~${Math.floor(minutesLeft / 60)}h ${minutesLeft % 60}m left`
          : `~${minutesLeft}m left`;
      }

      syncProgress(location.start.cfi, percent ?? 0);
    });

    book.locations.generate(1000).then(() => {
      totalPages = book.locations.total;
    });

    rendition.on("keyup", (e: KeyboardEvent) => {
      if (e.key === "ArrowRight" || e.key === "ArrowDown") rendition.next();
      if (e.key === "ArrowLeft" || e.key === "ArrowUp") rendition.prev();
    });
  });

  onDestroy(() => {
    rendition?.destroy();
    book?.destroy();
  });

  function setFontSize(size: number) {
    fontSize = Math.max(12, Math.min(28, size));
    rendition?.themes.fontSize(`${fontSize}px`);
  }

  function toggleTheme() {
    isDark = !isDark;
    rendition?.themes.select(isDark ? "mimir-dark" : "mimir-light");
  }

  async function jumpToChapter(href: string, index: number) {
    await rendition?.display(href);
    currentChapterIndex = index;
  }
</script>

<div class="flex h-full bg-background text-foreground overflow-hidden">

  <!-- ── Main reading area ──────────────────────────────────────── -->
  <div class="flex-1 flex flex-col min-w-0">

    <!-- Top bar -->
    <div class="flex items-center justify-between px-6 py-3 border-b border-border shrink-0">
      <div class="flex items-center gap-3">
        <button
          onclick={() => goto("/")}
          class="text-muted-foreground hover:text-foreground transition-colors"
        >
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path d="M10 4L6 8l4 4" stroke="currentColor" stroke-width="1.6"
                  stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </button>
        <div>
          <p class="text-sm font-semibold text-foreground leading-tight">{bookTitle}</p>
          <p class="text-xs text-muted-foreground">{bookAuthor}</p>
        </div>
      </div>

      <!-- Font + theme controls -->
      <div class="flex items-center gap-3">
        <button
          onclick={() => setFontSize(fontSize - 2)}
          class="text-muted-foreground hover:text-foreground transition-colors
                 text-sm font-medium w-7 h-7 flex items-center justify-center"
        >A−</button>
        <span class="text-muted-foreground text-base font-medium">Aa</span>
        <button
          onclick={() => setFontSize(fontSize + 2)}
          class="text-muted-foreground hover:text-foreground transition-colors
                 text-lg font-medium w-7 h-7 flex items-center justify-center"
        >A+</button>
        <div class="w-px h-4 bg-border" />
        <button
          onclick={toggleTheme}
          class="text-muted-foreground hover:text-foreground transition-colors
                 w-7 h-7 flex items-center justify-center"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="4" stroke="currentColor" stroke-width="1.5"/>
            <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"
                  stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
        <button
          class="text-muted-foreground hover:text-foreground transition-colors
                 w-7 h-7 flex items-center justify-center"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
            <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"
                  stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/>
          </svg>
        </button>
        <!-- scroll mode toggle -->
        <button
          onclick={toggleScrollMode}
          title={isVertical ? "Switch to paginated" : "Switch to scroll"}
          class="text-muted-foreground hover:text-foreground transition-colors
                 w-7 h-7 flex items-center justify-center
                 {isVertical ? 'text-primary' : ''}"
        >
          {#if isVertical}
            <!-- vertical scroll icon -->
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
              <path d="M12 5v14M5 12l7 7 7-7" stroke="currentColor" stroke-width="1.5"
                    stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          {:else}
            <!-- paginated icon -->
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
              <rect x="3" y="3" width="8" height="18" rx="1" stroke="currentColor" stroke-width="1.5"/>
              <rect x="13" y="3" width="8" height="18" rx="1" stroke="currentColor" stroke-width="1.5"/>
            </svg>
          {/if}
        </button>
        <button
          onclick={() => showChapters = !showChapters}
          class="text-muted-foreground hover:text-foreground transition-colors
                 w-7 h-7 flex items-center justify-center"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
            <path d="M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01"
                  stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- epub.js render target -->
    <div
      bind:this={readerEl}
      class="flex-1 overflow-hidden"
      style="background: {isDark ? '#171717' : '#fafafa'}"
    />

    <!-- Bottom navigation -->
    <div class="flex items-center justify-between px-6 py-3 border-t border-border shrink-0">
      <button
        onclick={() => rendition?.prev()}
        class="text-muted-foreground hover:text-foreground transition-colors p-1"
      >
        <svg width="18" height="18" viewBox="0 0 16 16" fill="none">
          <path d="M10 4L6 8l4 4" stroke="currentColor" stroke-width="1.6"
                stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </button>

      <span class="text-sm text-muted-foreground tabular-nums">
        {#if totalPages > 0}
          Page {Math.round(progressPercent / 100 * totalPages)} of {totalPages}
        {:else}
          {progressPercent}% complete
        {/if}
      </span>

      <button
        onclick={() => rendition?.next()}
        class="text-muted-foreground hover:text-foreground transition-colors p-1"
      >
        <svg width="18" height="18" viewBox="0 0 16 16" fill="none">
          <path d="M6 4l4 4-4 4" stroke="currentColor" stroke-width="1.6"
                stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </button>
    </div>

    <!-- Progress strip -->
    <div class="flex items-center gap-3 px-6 pb-3 shrink-0">
      <span class="text-xs text-muted-foreground tabular-nums w-12">
        {progressPercent}%
      </span>
      <div class="flex-1 h-0.5 bg-muted rounded-full overflow-hidden">
        <div class="h-full bg-primary rounded-full transition-all"
             style="width: {progressPercent}%" />
      </div>
      <span class="text-xs text-muted-foreground w-24 text-right">{timeRemaining}</span>
    </div>
  </div>

  <!-- ── Right panel ────────────────────────────────────────────── -->
  {#if showChapters}
    <div class="w-72 border-l border-border flex flex-col shrink-0">

      <!-- Read / Listen toggle at top -->
      <div class="px-4 pt-4 pb-3 border-b border-border shrink-0">
        <div class="flex items-center gap-1 bg-muted rounded-lg p-1">
          <button
            class="flex-1 py-1.5 rounded-md text-sm font-medium
                   bg-card text-foreground shadow-sm"
          >
            Read
          </button>
          <button
            onclick={() => goto("/now-playing")}
            class="flex-1 py-1.5 rounded-md text-sm font-medium
                   text-muted-foreground hover:text-foreground transition-colors"
          >
            Listen
          </button>
        </div>
      </div>

      <!-- Cover -->
      <div class="mx-4 mt-4 rounded-xl overflow-hidden aspect-square bg-zinc-800 shrink-0">
        <div class="w-full h-full bg-gradient-to-br from-zinc-600 to-zinc-900" />
      </div>

      <!-- Reading progress -->
      <div class="px-4 mt-4 mb-4 shrink-0">
        <div class="flex items-center justify-between text-sm mb-2">
          <span class="text-muted-foreground">Reading Progress</span>
          <span class="text-primary font-medium">{progressPercent}%</span>
        </div>
        <div class="h-1 bg-muted rounded-full overflow-hidden">
          <div class="h-full bg-primary rounded-full transition-all"
               style="width: {progressPercent}%" />
        </div>
      </div>

      <!-- Chapters -->
      <p class="text-xs font-semibold uppercase tracking-widest
                text-muted-foreground px-4 mb-2 shrink-0">
        Chapters
      </p>
      <div class="flex-1 overflow-y-auto px-2 space-y-0.5 pb-4">
        {#each chapters as chapter, i}
          <button
            onclick={() => jumpToChapter(chapter.href, i)}
            class="w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-left
                   transition-colors
                   {i === currentChapterIndex
                     ? 'bg-accent text-foreground'
                     : 'text-muted-foreground hover:bg-accent/50 hover:text-foreground'}"
          >
            {#if i < currentChapterIndex}
              <svg width="12" height="12" viewBox="0 0 16 16" fill="none"
                   class="text-muted-foreground/50 shrink-0">
                <path d="M3 8l4 4 6-6" stroke="currentColor" stroke-width="1.8"
                      stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            {:else if i === currentChapterIndex}
              <div class="w-3 h-3 rounded-full bg-primary shrink-0" />
            {:else}
              <span class="w-3 text-center text-xs text-muted-foreground/50 shrink-0">
                {i + 1}
              </span>
            {/if}
            <span class="text-sm truncate">{chapter.label}</span>
          </button>
        {/each}
      </div>

    </div>
  {/if}

</div>
