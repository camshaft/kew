import routes from "~react-pages";
import { Router as _Router, Route as _Route, useLocation } from "preact-iso";

export interface Props {
  children: any;
}

export interface Route {
  path: string;
  id: string;
  idx: number;
  title: string;
  depth: number;
  prev: Route | undefined;
  next: Route | undefined;
  element: any;
  children: Route[];
  parent?: Route;
}

export const pathToRoute: Map<string, Route> = new Map();

const routerRoutes = [];
const rootRoutes: Route[] = [];
let currentDepth = Number.MAX_SAFE_INTEGER;
let currentChildren: Route[] = [];

for (let idx = routes.length - 1; idx >= 0; idx--) {
  let r = routes[idx];
  r.children = [];

  if (!r.depth) rootRoutes.unshift(r);

  if (r.depth < currentDepth) {
    currentDepth = r.depth;
    currentChildren.forEach((child) => {
      child.parent = r;
    });
    r.children.push(...currentChildren);
    currentChildren = [r];
  } else if (r.depth > currentDepth) {
    currentDepth = r.depth;
    currentChildren = [r];
  } else {
    currentChildren.unshift(r);
  }

  routerRoutes.push(handleRoute(r, idx));
}

export const router = <_Router>{routerRoutes}</_Router>;

export { routes, rootRoutes };

function handleRoute(route: Route, idx: number) {
  let { path, element, title } = route;

  route.idx = idx;
  route.prev = routes[idx - 1];
  route.next = routes[idx + 1];
  path = `${import.meta.env.BASE_URL}${path}`;
  route.path = path;

  pathToRoute.set(path, route);
  pathToRoute.set(path.replace(/\/$/, ""), route);

  const component = () => {
    if (!import.meta.env.SSR) window.document.title = `Kew - ${title}`;
    return element;
  };

  return <_Route key={idx} path={path} component={component} />;
}

export function useRoute(): Route | undefined {
  const { path } = useLocation();
  return pathToRoute.get(path);
}
