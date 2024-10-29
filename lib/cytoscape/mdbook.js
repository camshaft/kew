import { basename } from "node:path";
import { readFile, writeFile } from "node:fs/promises";

export async function renderPath(info, config) {
  const path = info.elements;
  //const dir = dirname(path);
  const contents = await readFile(path);
  const elements = JSON.parse(contents);

  /*
  const png = await toPng(elements, dir);

  const pngPath = path.replace(/\.json$/, ".png");
  await writeFile(pngPath, png);

  const url = basename(pngPath);
  */

  if (config.renderer != "html") return `TODO render cytoscape offline`;

  const compiledPath = path.replace(/\.json$/, ".min.json");

  let site_url = config.config.output.html["site-url"] || "/";
  if (!site_url.match(/\/$/)) site_url += "/";
  const resolve = (url) => (url.charAt(0) == "/" ? url : `${site_url}${url}`);

  await writeFile(compiledPath, JSON.stringify(elements));

  const jsonUrl = resolve(basename(compiledPath));
  return `<div style="height: 500px" data-cytoscape data-elements="${jsonUrl}"></div>`;
}
