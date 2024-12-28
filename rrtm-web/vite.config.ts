import { defineConfig } from "vite";

export default defineConfig({
  build: {
    target: "esnext",
  },
  optimizeDeps: {
    exclude: ["rrtm"],
  },
  server: {
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
    fs: {
      allow: ["..", "../.."],
    },
  },
});
