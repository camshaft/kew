import { defineConfig } from "vite";
import mdx from "@mdx-js/rollup";
import tailwindcss from "@tailwindcss/vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import Pages from "./plugins/pages.ts";
import StaticCrate from "./plugins/static-crate.ts";
import remarkGfm from "remark-gfm";
import { visualizer } from "rollup-plugin-visualizer";
import preact from "@preact/preset-vite";
import remarkMath from "remark-math";
import remarkMdxEnhanced from "remark-mdx-math-enhanced";
import rehypeKatex from "rehype-katex";

const isProd = process.env.NODE_ENV == "production";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    StaticCrate(),
    wasm(),
    topLevelAwait(),
    Pages(),
    tailwindcss(),
    mdx({
      jsx: false,
      rehypePlugins: [rehypeKatex],
      remarkPlugins: [
        remarkMath,
        [remarkMdxEnhanced, { component: "Math" }],
        [remarkGfm, {}],
      ],
    }),
    preact(),
    visualizer({
      filename: "target/build-stats.html",
    }),
  ],
  base: "/kew",
  resolve: {
    alias: {
      "@": "/src/components",
      $: "/src/crates",
      "~": "/src",
    },
  },
  server: {
    watch: {
      ignored: ["**/target/**", "**/*.rs"],
    },
  },
  build: {
    minify: isProd,
  },
});
