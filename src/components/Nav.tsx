import clsx from "clsx";
import { ClassValue } from "clsx";
import { Link } from "./Link.tsx";
import { Route, rootRoutes } from "./routes.tsx";

const navItems = setupNav(rootRoutes);

export default function Nav() {
  return (
    <nav className="flex flex-col gap-8" aria-label="Title of contents">
      {navItems}
    </nav>
  );
}

function TopNavItems({ children }: { children: any }) {
  return <div className="flex flex-col gap-3">{children}</div>;
}

function NavItems({ children }: { children: any }) {
  return (
    <ol className="flex flex-col gap-2 border-l dark:border-[color-mix(in_oklab,_var(--color-gray-950),white_20%)] border-[color-mix(in_oklab,_var(--color-gray-950),white_90%)]">
      {children}
    </ol>
  );
}

interface ItemProps {
  id: string;
  title: string;
  fullPath: string;
  children: any;
}

function itemClasses(...v: ClassValue[]) {
  const className =
    "text-gray-600 hover:border-gray-950/25 hover:text-gray-950 dark:text-gray-300 dark:hover:border-white/25 dark:hover:text-white aria-[current]:border-gray-950 aria-[current]:font-semibold aria-[current]:text-gray-950 dark:aria-[current]:border-white dark:aria-[current]:text-white";
  return clsx(className, ...v);
}

function TopNavItem({ id, title, fullPath, children }: ItemProps) {
  return (
    <>
      <Link
        nav
        to={fullPath}
        className={itemClasses(
          "font-mono text-sm/6 font-medium tracking-widest uppercase sm:text-xs/6"
        )}
      >
        <h3>
          {id} {title}
        </h3>
      </Link>
      {children}
    </>
  );
}

function NavItem({ id, title, fullPath, children }: ItemProps) {
  return (
    <li className="-ml-px flex flex-col items-start gap-2">
      <Link
        nav
        className={itemClasses(
          "inline-block border-l border-transparent text-base/8 sm:text-sm/6 pl-5 sm:pl-4"
        )}
        to={fullPath}
      >
        {id} {title}
      </Link>
      {children}
    </li>
  );
}

function setupNav(routes: Route[], depth: number = 0) {
  const Items = depth == 0 ? TopNavItems : NavItems;
  const Item = depth == 0 ? TopNavItem : NavItem;

  return (
    <Items>
      {routes.map(({ id, title, path, children }, idx) => {
        return (
          <Item key={idx} id={id} title={title} fullPath={path}>
            {children && children.length > 0 && setupNav(children, depth + 1)}
          </Item>
        );
      })}
    </Items>
  );
}
