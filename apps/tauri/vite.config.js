import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { internalIpV4 } from "internal-ip";

export default defineConfig(async () => ({
  plugins: [
    tailwindcss(),
    sveltekit(),
  ],
  // Tauri needs a fixed port to talk to the Vite dev server
  server: {
    host: "0.0.0.0",
    port: 1420,
    strictPort: true,
    hmr: {
      protocol: "ws",
      host: await internalIpV4(),
      port: 1421,
    },
  },
}));
