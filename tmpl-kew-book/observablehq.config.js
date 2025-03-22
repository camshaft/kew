import kewPlugin from "./lib/kew-md.js";
import parsePages from "./lib/pages.js";

const PAGES = parsePages`
Capacity Management
  Unbounded
  Backpressure
  Prefer Old
  Prefer New
Service Disciplines
  FIFO: First In, First Out|fifo
  LIFO: Last In, First Out|lifo
  Round Robin
  Priority
  Shortest-Job-First
Active Queue Managment|aqm
  CoDel
  CAKE
Congestion Control Algorithms|cca
  CUBIC
  BBR
`;

// See https://observablehq.com/framework/config for documentation.
export default {
  // The app’s title; used in the sidebar and webpage titles.
  title: "Kew",

  search: true,

  pages: PAGES,

  // Content to add to the head of the page, e.g. for a favicon:
  head: '<link rel="icon" href="observable.png" type="image/png" sizes="32x32">',

  // The path to the source root.
  root: "src",

  footer(config) {
    console.log(config);
    return "";
  },

  toc: true,

  // Some additional configuration options and their defaults:
  // theme: "default", // try "light", "dark", "slate", etc.
  // header: "", // what to show in the header (HTML)
  // footer: "Built with Observable.", // what to show in the footer (HTML)
  // sidebar: true, // whether to show the sidebar
  // toc: true, // whether to show the table of contents
  // pager: true, // whether to show previous & next links in the footer
  // output: "dist", // path to the output root for build
  // search: true, // activate search
  // linkify: true, // convert URLs in Markdown to links
  // typographer: false, // smart quotes and other typographic improvements
  // cleanUrls: true, // drop .html from URLs

  markdownIt(md) {
    kewPlugin(md);
    return md;
  },

  interpreters: {
    ".kew": ["cargo", "xtask", "compile"],
  },
};
