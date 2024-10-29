import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import swc from "@rollup/plugin-swc";
import { nodeResolve } from "@rollup/plugin-node-resolve";

const { NODE_ENV } = process.env;
const IS_PROD = NODE_ENV == "production";

const plugins = [
  nodeResolve({
    browser: true,
    mainFields: ["module", "main"],
  }),
  commonjs(),
  json(),
];

if (IS_PROD) {
  plugins.push(swc());
}

export default {
  input: "lib/index.js",
  output: {
    dir: "target/book-src/js",
    format: "iife",
    sourcemap: true,
    sourcemapBaseUrl: IS_PROD
      ? "https://camshaft.github.io/kew/js"
      : "http://localhost:300/js",
  },
  plugins,
};
