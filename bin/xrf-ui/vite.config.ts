import * as path from "path";

import { default as react } from "@vitejs/plugin-react";
import { default as observerPlugin } from "mobx-react-observer/vite-plugin";
import { defineConfig, Plugin } from "vite";

function reactObserverPlugin(): Plugin {
  const plugin: Plugin = observerPlugin() as Plugin;
  const transform = plugin.transform;

  return {
    ...plugin,
    name: "mobx-react-observer-tsx",
    transform(code, id) {
      if (id.endsWith(".tsx") && typeof transform === "function") {
        return transform.call(this, code, id);
      }

      return null;
    },
  };
}

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react(), reactObserverPlugin()],
  build: {
    outDir: "target",
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
