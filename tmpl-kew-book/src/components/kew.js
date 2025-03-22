export * from "./kew/kew_bg.js";
import { __wbg_set_wasm } from "./kew/kew_bg.js";
import * as kew_bg from "./kew/kew_bg.js";

const importObject = {
  "./kew_bg.js": kew_bg,
};

await load();

async function load() {
  const loader = typeof document == "undefined" ? loadNode() : loadBrowser();
  const { instance } = await loader;
  __wbg_set_wasm(instance.exports);
}

async function loadNode() {
  const { readFile } = await import("node:fs/promises");
  const { fileURLToPath } = await import("node:url");
  const path = fileURLToPath(import.meta.resolve("./kew/kew_bg.wasm"));
  const file = await readFile(path);
  return await WebAssembly.instantiate(file, importObject);
}

async function loadBrowser() {
  const url = import.meta.resolve("./kew/kew_bg.wasm");
  const req = fetch(url);
  return await WebAssembly.instantiateStreaming(req, importObject);
}

export default async function* kew(init, values) {
  if (init) {
    const tbl = await init;
    const rows = tbl.toArray();
    yield rows;
  }

  let kew;

  for await (let value of values) {
    const startTime = performance.now();

    if (!kew) kew = new kew_bg.Kew(value);
    else kew.update(value);

    // TODO try to reuse the buffer
    yield kew.to_arrow();

    console.log(
      "sim time",
      performance.mark("kew:sim", { startTime }).duration
    );
  }
}
