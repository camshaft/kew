import { Route, useRoute } from "./routes.tsx";
import { Link } from "./Link.tsx";
import { ChevronRightIcon } from "@heroicons/react/16/solid";

export function Breadcrumb() {
  const route = useRoute();

  // TODO figure out why this doesn't work
  const separator = import.meta.env.SSR ? (
    false
  ) : (
    <ChevronRightIcon className="size-4 fill-gray-950 dark:fill-gray-500" />
  );

  const children = [];
  if (route) {
    let r: Route | undefined = route;
    while (r) {
      const idx: number = children.length;

      const child = idx ? (
        <li key={idx} class="flex items-center gap-2">
          <span class="text-gray-500 dark:text-gray-400">
            <Link nav to={r}>
              {r.title}
            </Link>
          </span>
          {separator}
        </li>
      ) : (
        <li key={idx} className="truncate text-gray-950 dark:text-white">
          <Link nav to={r}>
            {r.title}
          </Link>
        </li>
      );

      children.unshift(child);
      r = r.parent;
    }
  }

  return (
    <ol className="sticky ml-4 flex min-w-0 items-center gap-2 text-sm/6 whitespace-nowrap">
      {children}
    </ol>
  );
}
