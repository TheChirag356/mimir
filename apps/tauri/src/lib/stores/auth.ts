import { writable, derived } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

interface User {
  id: string;
  username: string;
  is_admin: boolean;
}

interface AuthState {
  token: string | null;
  user: User | null;
  serverUrl: string;
}

function createAuthStore() {
  const { subscribe, set, update } = writable<AuthState>({
    token: localStorage.getItem("token"),
    user: JSON.parse(localStorage.getItem("user") ?? "null"),
    serverUrl: localStorage.getItem("serverUrl") ?? "http://localhost:3333",
  });

  return {
    subscribe,
    login: async (serverUrl: string, username: string, password: string) => {
      const result = await invoke<{ token: string; user: User }>("login", {
        serverUrl,
        username,
        password,
      });
      localStorage.setItem("token", result.token);
      localStorage.setItem("user", JSON.stringify(result.user));
      localStorage.setItem("serverUrl", serverUrl);
      set({ token: result.token, user: result.user, serverUrl });
      return result;
    },
    logout: () => {
      localStorage.removeItem("token");
      localStorage.removeItem("user");
      set({ token: null, user: null, serverUrl: "http://localhost:3333" });
    },
  };
}

export const auth = createAuthStore();
export const isLoggedIn = derived(auth, ($auth) => $auth.token !== null);
