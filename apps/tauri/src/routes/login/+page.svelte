<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { goto } from "$app/navigation";

  // Svelte 5 uses $state() instead of plain `let` for reactive variables
  // that need two-way binding (bind:value). Plain `let` is no longer
  // reactive in Svelte 5 components.
  let serverUrl = $state("http://localhost:3333");
  let username = $state("");
  let password = $state("");
  let error = $state("");
  let loading = $state(false);

  async function handleLogin() {
    loading = true;
    error = "";
    try {
      await auth.login(serverUrl, username, password);
      goto("/");
    } catch (e) {
      error = "Invalid username or password";
    } finally {
      loading = false;
    }
  }
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
