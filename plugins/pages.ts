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
      buildNav(routes);
      return routes;
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

  route.meta = {
    id,
    title,
    fullPath,
  };

  const index = route.children.find((r: any) => r.path == "" && r.element);

  const children = formatRoutes(route.children, stack);

  if (index) {
    if (children.length) {
      index.index = true;
      children.unshift(index);
    } else {
      // promote the index to itself
      route.element = index.element;
    }
  }

  route.children = children;

  return route;
}

function buildNav(route: any, prev: any = null, next: any = null) {
  if (Array.isArray(route)) {
    const routes = route;
    let parentNext: any;
    routes.forEach((route, idx) => {
      if (!route.meta) return;
      // the parent next is the first non-index route
      if (!parentNext) parentNext = route.meta.fullPath;
      const nextRoute = routes[idx + 1];
      const localNext = nextRoute?.meta?.fullPath || next;
      prev = buildNav(route, prev, localNext);
    });
    return parentNext;
  }

  if (!route.meta) return prev;

  route.meta.prev = prev;
  route.meta.next = next;

  if (route.children.length) {
    route.meta.next = buildNav(route.children, route.meta.fullPath, next);
    return route.children[route.children.length - 1].meta.fullPath;
  }

  return route.meta.fullPath;
}

interface StackItem {
  path: string;
  idx: number;
}
