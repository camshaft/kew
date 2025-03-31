import Pages from "vite-plugin-pages";

export default function () {
  return Pages({
    dirs: "src/chapters",
    extensions: ["tsx", "mdx", "md"],
    resolver: "react",
    importMode() {
      return "sync";
    },
    onRoutesGenerated: (routes) => {
      routes = formatRoutes(routes, []);
      const list = [];
      flattenRoutes(routes, list);
      return list;
    },
  });
}

function formatRoutes(routes: any, parents: StackItem[]) {
  if (!routes) return [];
  const out: any = [];
  routes.sort((a: any, b: any) => a.path.localeCompare(b.path));
  routes.forEach((route: any) => {
    route = formatRoute(route, out.length, parents);
    if (route) out.push(route);
  });
  return out;
}

function formatRoute(route: any, idx: number, parents: StackItem[]) {
  if (route.element) return;

  // remove any number prefixes from the path
  route.path = route.path.replace(/^[\d_]+/, "");

  const title = route.path.split("_").join(" ");

  if (parents.length == 0 && route.path === "Introduction") {
    route.path = "";
  }

  // lowercase the path and replace any non-alphanumeric chars
  route.path = route.path
    .toLowerCase()
    .replace(/[^a-zA-Z0-9]/g, "-")
    .replace(/-+/g, "-");

  let stack = [...parents, { path: route.path, idx }];

  const id = stack.map((item) => item.idx + 1).join(".") + ".";
  let fullPath = "/" + stack.map((item) => item.path).join("/");

  if (!fullPath.endsWith("/")) fullPath += "/";

  route.id = id;
  route.title = title;
  route.depth = parents.length;
  route.path = fullPath;
  delete route.caseSensitive;

  const index = route.children.find((r: any) => r.path == "" && r.element);
  if (index) {
    route.element = index.element;
  }

  const children = formatRoutes(route.children, stack);

  route.children = children;

  return route;
}

function flattenRoutes(route: any, out: any[]) {
  if (Array.isArray(route)) {
    route.forEach((route, idx) => {
      if (!route.element) return;
      flattenRoutes(route, out);
    });
  }

  if (!route.element) return;

  const children = route.children;
  out.push(route);
  flattenRoutes(children, out);
  delete route.children;
}

interface StackItem {
  path: string;
  idx: number;
}
