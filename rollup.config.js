import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import { nodeResolve } from "@rollup/plugin-node-resolve";

export default {
  input: "lib/index.js",
  output: {
    dir: "target/book-src/js",
    format: "iife",
    sourcemap: true,
          // TODO fix path
    sourcemapBaseUrl: "http://localhost:3000/js",
  },
  plugins: [
    nodeResolve({
      browser: true,
      mainFields: ["module", "main"],
    }),
    commonjs(),
    json(),
  ],
};
