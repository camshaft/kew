import clsx from "clsx";
import { ClassValue } from "clsx";
import { NavLink } from "react-router";

export interface Props {
  children: any;
}

export default function Nav({ children }: Props) {
  return (
    <nav className="flex flex-col gap-8" aria-label="Title of contents">
      {children}
    </nav>
  );
}

export function TopNavItems({ children }: { children: any }) {
  return <div className="flex flex-col gap-3">{children}</div>;
}

export function NavItems({ children }: { children: any }) {
  return (
    <ol className="flex flex-col gap-2 border-l dark:border-[color-mix(in_oklab,_var(--color-gray-950),white_20%)] border-[color-mix(in_oklab,_var(--color-gray-950),white_90%)]">
      {children}
    </ol>
  );
}

export interface ItemProps {
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

export function TopNavItem({ id, title, fullPath, children }: ItemProps) {
  return (
    <>
      <NavLink
        to={fullPath}
        className={itemClasses(
          "font-mono text-sm/6 font-medium tracking-widest uppercase sm:text-xs/6"
        )}
      >
        <h3>
          {id} {title}
        </h3>
      </NavLink>
      {children}
    </>
  );
}

export function NavItem({ id, title, fullPath, children }: ItemProps) {
  return (
    <li className="-ml-px flex flex-col items-start gap-2">
      <NavLink
        className={itemClasses(
          "inline-block border-l border-transparent text-base/8 sm:text-sm/6 pl-5 sm:pl-4"
        )}
        to={fullPath}
      >
        {id} {title}
      </NavLink>
      {children}
    </li>
  );
}
