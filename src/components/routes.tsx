import routes from "~react-pages";
import { NavItem, NavItems, TopNavItem, TopNavItems } from "@/Nav";

export interface Props {
  children: any;
}

export interface Route {
  meta: {
    id: string;
    title: string;
    fullPath: string;
    next: string | null;
    prev: string | null;
    depth: number;
  };
  children: undefined | Route[];
}

export const router = routes;

export const pathToRoute: Map<string, Route> = new Map();

export const navItems = setupNav(routes as Route[]);

function setupNav(routes: Route[], depth = 0) {
  const Items = depth == 0 ? TopNavItems : NavItems;
  const Item = depth == 0 ? TopNavItem : NavItem;

  return (
    <Items>
      {routes
        .filter(({ meta }) => !!meta)
        .map((route, idx) => {
          route.meta.depth = depth;
          const {
            meta: { id, title, fullPath },
            children,
          } = route;
          pathToRoute.set(fullPath, route);
          return (
            <Item key={idx} id={id} title={title} fullPath={fullPath}>
              {children && children.length > 0 && setupNav(children, depth + 1)}
            </Item>
          );
        })}
    </Items>
  );
}
