import { useLocation } from "preact-iso";
import { pathToRoute, Route } from "./routes";

export interface Props
  extends Omit<React.AnchorHTMLAttributes<HTMLAnchorElement>, "href"> {
  to: Route | string;
  nav?: boolean;
}

export function Link({ to, nav, ...props }: Props) {
  const location = useLocation();
  const currentRoute = pathToRoute.get(location.path);
  const targetRoute = typeof to === "string" ? pathToRoute.get(to) : to;

  if (import.meta.env.DEV && !targetRoute) {
    throw new Error(`Route not found: ${to}`);
  }

  const active =
    targetRoute && currentRoute && currentRoute.path === targetRoute.path;

  const onClick = (evt: Event) => {
    const path = targetRoute?.path;

    if (!path) return;

    evt.preventDefault();
    evt.stopPropagation();

    if (active) return;

    location.route(path);
  };

  return (
    <a
      aria-current={nav && active ? "page" : undefined}
      href={targetRoute?.path}
      onClick={onClick}
      {...props}
    />
  );
}
