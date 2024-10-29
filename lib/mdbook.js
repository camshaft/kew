import { renderPath as renderCytoscape } from "./cytoscape/mdbook.js";
import { renderPath as renderVega } from "./vega/mdbook.js";

async function* lines(str) {
  for (let l of str.split("\n")) {
    yield l;
  }
}

async function* vega(lines, config) {
  for await (let l of lines) {
    const match = l.match(/^\!VEGA(\{.*)/);

    if (!match) {
      yield l;
      continue;
    }

    const info = JSON.parse(match[1]);

    const out = await renderVega(info.path, config);

    yield out;
  }
}

async function* cytoscape(lines, config) {
  for await (let l of lines) {
    const match = l.match(/^\!CYTOSCAPE(\{.*)/);

    if (!match) {
      yield l;
      continue;
    }

    const info = JSON.parse(match[1]);

    const out = await renderCytoscape(info, config);

    yield out;
  }
}

async function join(lines) {
  let out = "";
  for await (let l of lines) {
    if (out.length) out += "\n";
    out += l;
  }
  return out;
}

async function pipeline(content, config) {
  let out = content;
  out = lines(out, config);
  out = vega(out, config);
  out = cytoscape(out, config);
  out = await join(out, config);
  return out;
}

async function renderSection(section, config) {
  const { Chapter } = section;

  if (Chapter) {
    Chapter.content = await pipeline(Chapter.content || "", config);

    const sub_items = Chapter.sub_items || [];
    for (let item of sub_items) {
      await renderSection(item, config);
    }
  }
}

export default async function (config, book) {
  for (let section of book.sections) {
    await renderSection(section, config);
  }
}
