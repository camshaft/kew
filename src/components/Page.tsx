import { Route, useRoute } from "./routes";
import createComponents from "./content.tsx";
import { ChevronRightIcon, ChevronLeftIcon } from "@heroicons/react/16/solid";

export interface Props {
  children: ({ components }: { components: any }) => any;
}

export default function Page({ children }: Props) {
  const route = useRoute();

  if (!route) return;

  const { title, prev, next } = route;

  const components = createComponents(route);

  const content = children({ components });

  return (
    <>
      <components.h1>{title}</components.h1>
      {content}
      <footer class="mt-16 text-sm leading-6">
        <div class="flex items-center justify-between gap-2 text-gray-700 dark:text-gray-200">
          <NavItem next={false} route={prev} />
          <NavItem next={true} route={next} />
        </div>
      </footer>
    </>
  );
}

function NavItem({ next, route }: { next: boolean; route: Route | undefined }) {
  if (!route) return <div className="group flex gap-2" />;

  // TODO fix the icons in SSR
  return (
    <a
      class="group flex items-center gap-2 hover:text-gray-900 dark:hover:text-white"
      href={route.path}
    >
      {!import.meta.env.SSR && !next && <ChevronLeftIcon className="size-4" />}
      <span>{route.title}</span>
      {!import.meta.env.SSR && next && <ChevronRightIcon className="size-4" />}
    </a>
  );
}
