import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import mdx from "@mdx-js/rollup";
import tailwindcss from "@tailwindcss/vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import Pages from "./plugins/pages.ts";
import remarkGfm from "remark-gfm";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    Pages(),
    tailwindcss(),
    mdx({ jsx: false, remarkPlugins: [[remarkGfm, {}]] }),
    react({
      parserConfig(id) {
        if (id.endsWith(".mdx") || id.endsWith(".jsx"))
          return { jsx: true, syntax: "ecmascript" };
        if (id.endsWith(".ts")) return { syntax: "typescript" };
        if (id.endsWith(".tsx")) return { tsx: true, syntax: "typescript" };
        if (id.endsWith(".css")) return;
        return { syntax: "ecmascript" };
      },
    }),
  ],
  base: "/kew",
  resolve: {
    alias: {
      "@": "/src/components",
      $: "/src/sims",
      "~": "/src",
    },
  },
  server: {
    watch: {
      ignored: ["**/target/**", "**/*.rs"],
    },
  },
});
