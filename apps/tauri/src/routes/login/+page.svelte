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
  let error = "";

  onMount(async () => {
    if (!$isLoggedIn || !$auth.token) {
      goto("/login");
      return;
    }

    try {
      const result = await invoke<Library[]>("get_libraries", {
        serverUrl: $auth.serverUrl,
        token: $auth.token,
      });
      // handle both array and {libraries: []} shapes from the server
      libraries = Array.isArray(result) ? result : (result as any).libraries ?? [];
    } catch (e) {
      error = String(e);
      console.error("get_libraries failed:", e);
    } finally {
      loading = false;
    }
  });
</script>

<div class="min-h-screen bg-background flex items-center justify-center">
  <div class="w-full max-w-sm bg-card border border-border rounded-lg p-8 shadow-sm">

    <div class="mb-8 text-center">
      <h1 class="text-2xl font-semibold text-foreground tracking-tight">Mimir</h1>
      <p class="text-sm text-muted-foreground mt-1">Sign in to your server</p>
    </div>

    <div class="space-y-4">
      <div>
        <label class="text-sm font-medium text-foreground" for="server">
          Server URL
        </label>
        <input
          id="server"
          type="url"
          bind:value={serverUrl}
          class="mt-1 w-full px-3 py-2 text-sm bg-background border border-input rounded-md
                 text-foreground placeholder:text-muted-foreground
                 focus:outline-none focus:ring-2 focus:ring-ring"
          placeholder="http://localhost:3333"
        />
      </div>

      <div>
        <label class="text-sm font-medium text-foreground" for="username">
          Username
        </label>
        <input
          id="username"
          type="text"
          bind:value={username}
          class="mt-1 w-full px-3 py-2 text-sm bg-background border border-input rounded-md
                 text-foreground placeholder:text-muted-foreground
                 focus:outline-none focus:ring-2 focus:ring-ring"
          placeholder="charlie"
        />
      </div>

      <div>
        <label class="text-sm font-medium text-foreground" for="password">
          Password
        </label>
        <input
          id="password"
          type="password"
          bind:value={password}
          class="mt-1 w-full px-3 py-2 text-sm bg-background border border-input rounded-md
                 text-foreground placeholder:text-muted-foreground
                 focus:outline-none focus:ring-2 focus:ring-ring"
          placeholder="••••••••"
        />
      </div>

      {#if error}
        <p class="text-sm text-destructive">{error}</p>
      {/if}

      <button
        on:click={handleLogin}
        disabled={loading}
        class="w-full py-2 px-4 bg-primary text-primary-foreground text-sm font-medium
               rounded-md hover:opacity-90 transition-opacity disabled:opacity-50"
      >
        {loading ? "Signing in…" : "Sign in"}
      </button>
    </div>
  </div>
</div>
