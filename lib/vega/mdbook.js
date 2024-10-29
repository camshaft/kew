import { basename, dirname, join } from "node:path";
import { readFile, writeFile } from "node:fs/promises";
import vega from "vega";
import vegaLite from "vega-lite";

function compile(json) {
  return json["$schema"].includes("lite") ? vegaLite.compile(json).spec : json;
}

export async function renderPath(path, config) {
  const dir = dirname(path);
  const contents = await readFile(path);
  const compiled = compile(JSON.parse(contents));
  const svg = await toSvg(compiled, dir);

  const svgPath = path.replace(/\.json$/, ".svg");
  await writeFile(svgPath, svg);

  const url = basename(svgPath);

  if (config.renderer != "html") return `![](${url})`;

  const compiledPath = path.replace(/\.json$/, ".min.json");

  let site_url = config.config.output.html["site-url"] || "/";
  if (!site_url.match(/\/$/)) site_url += "/";
  const resolve = (url) => (url.charAt(0) == "/" ? url : `${site_url}${url}`);

  compiled.data.forEach((data) => {
    if (data.url) data.url = resolve(data.url);
  });

  await writeFile(compiledPath, JSON.stringify(compiled));

  const jsonUrl = resolve(basename(compiledPath));
  return `<img data-vega="${jsonUrl}" src="${resolve(url)}"/>`;
}

export async function toSvg(json, dir) {
  const logger = {
    level() {
      console.error("LEVEL", ...arguments);
    },
    error() {
      console.error("ERROR", ...arguments);
    },
    warn() {
      console.error("WARN", ...arguments);
    },
    info() {
      //console.error('INFO', ...arguments);
    },
    debug() {
      //console.error('DEBUG', ...arguments);
    },
  };

  const parsed = vega.parse(json);

  const loader = {
    sanitize() {},
    load: async function load(file) {
      const resolved = join(dir, file);
      return await readFile(resolved);
    },
    fileAccess: true,
    file: async function file() {
      throw new Error();
    },
    http: async function http() {
      throw new Error();
    },
  };

  const view = new vega.View(parsed, {
    renderer: "svg",
    logLevel: vega.DEBUG,
    loader,
    logger,
  });

  return await view.toSVG();
}
