import { Plugin } from "vite";
import { serialize } from "../src/data/sim.ts";

export default function staticCrate(): Plugin {
  return {
    name: "static-crate",
    resolveId(source) {
      if (parseId(source)) return source;
      return null;
    },
    load: async (id) => {
      const url = parseId(id);
      if (!url) return null;

      const params = url.searchParams;
      const contents = await import(url.pathname);
      const expName = params.get("fn") || "default";
      const fn = contents[expName];

      if (!fn) {
        let msg = `could not find ${expName} in list of exports in ${id}`;
        throw new Error(msg);
      }

      const sim = fn(params);
      let out = serialize(sim);

      out += "\n";

      let p: any = {};
      params.forEach((value, name) => {
        p[name] = value;
      });
      out += `export const params = ${JSON.stringify(p)};\n`;

      return out;
    },
  };
}

function parseId(id: string): URL | null {
  const [path, query] = id.split("?");
  if (!path.endsWith(".static.js")) return null;

  const url = new URL("file:");

  url.pathname = path;
  if (query) url.search = query;

  return url;
}
