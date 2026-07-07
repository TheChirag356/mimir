<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { auth, isLoggedIn } from "$lib/stores/auth";
  import { goto } from "$app/navigation";

  interface Library {
    id: string;
    name: string;
    media_type: string;
  }

  let libraries: Library[] = [];
  let loading = true;

  onMount(async () => {
    if (!$isLoggedIn) {
      goto("/login");
      return;
    }

    try {
      const result = await invoke<{ libraries: Library[] }>("get_libraries", {
        serverUrl: $auth.serverUrl,
        token: $auth.token,
      });
      libraries = result.libraries ?? result; // handle both shapes
    } finally {
      loading = false;
    }
  });
</script>

<div class="min-h-screen bg-background">
  <header class="border-b border-border px-6 py-4 flex items-center justify-between">
    <h1 class="text-lg font-semibold text-foreground tracking-tight">Mimir</h1>
    <button
      on:click={() => { auth.logout(); goto("/login"); }}
      class="text-sm text-muted-foreground hover:text-foreground transition-colors"
    >
      Sign out
    </button>
  </header>

  <main class="px-6 py-8 max-w-5xl mx-auto">
    {#if loading}
      <p class="text-muted-foreground text-sm">Loading libraries…</p>
    {:else if libraries.length === 0}
      <p class="text-muted-foreground text-sm">No libraries yet. Add one in mimir-core.</p>
    {:else}
      <h2 class="text-sm font-medium text-muted-foreground uppercase tracking-wider mb-4">
        Libraries
      </h2>
      <div class="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-4">
        {#each libraries as library}
            <a
            href="/library/{library.id}"
            class="bg-card border border-border rounded-lg p-4 hover:border-primary/50
                   transition-colors cursor-pointer"
          >
            <p class="font-medium text-foreground text-sm">{library.name}</p>
            <p class="text-xs text-muted-foreground mt-1 capitalize">{library.media_type}</p>
          </a>
        {/each}
      </div>
    {/if}
  </main>
</div>
