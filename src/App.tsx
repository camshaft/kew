import { Suspense, useState } from "react";
import { useRoutes, useLocation } from "react-router";
import Loading from "@/Loading.tsx";
import Nav from "@/Nav.tsx";
import { router, navItems } from "@/routes";
import clsx from "clsx";
import KeyBindings from "@/KeyBindings";

export default function App() {
  if (import.meta.env.VITE_SSR_TARGET == "md") return useRoutes(router);

  const { pathname } = useLocation();
  const [navVisible, setNavVisible] = useState(false);

  return (
    <>
      <KeyBindings onEscape={() => setNavVisible(false)} />
      <div className="isolate dark:bg-gray-900">
        <div className="fixed inset-x-0 top-0 z-10 border-b border-gray-950/5 dark:border-white/10">
          <div className="bg-white dark:bg-gray-950">{/* TODO top nav? */}</div>
          <div className="flex h-14 items-center border-t border-gray-950/5 bg-white px-4 sm:px-6 lg:hidden dark:border-white/10 dark:bg-gray-950">
            <button
              className="relative inline-grid size-7 place-items-center rounded-md text-gray-950 hover:bg-gray-950/5 dark:text-white dark:hover:bg-white/10 -ml-1.5"
              type="button"
              aria-label="Open navigation menu"
              onClick={() => setNavVisible(!navVisible)}
            >
              Open
            </button>
            <ol className="sticky ml-4 flex min-w-0 items-center gap-2 text-sm/6 whitespace-nowrap">
              {/* TODO inject the bread crumbs */}
              <li className="truncate text-gray-950 dark:text-white">
                {pathname}
              </li>
            </ol>
          </div>
        </div>
        <div className="grid min-h-dvh grid-cols-1 grid-rows-[1fr_1px_auto_1px_auto] pt-26.25 lg:grid-cols-[var(--container-2xs)_2.5rem_minmax(0,1fr)_2.5rem] lg:pt-14.25 xl:grid-cols-[var(--container-2xs)_2.5rem_minmax(0,1fr)_2.5rem]">
          <div
            className={clsx(
              "relative col-start-1 row-span-full row-start-1 max-lg:absolute max-lg:top-0 max-lg:bottom-0 transition-transform",
              {
                "max-lg:-translate-x-96": !navVisible,
              }
            )}
          >
            <div className="absolute inset-0">
              <div className="sticky top-14 bottom-0 left-0 dark:bg-gray-900 z-10 h-full max-h-[calc(100dvh-(var(--spacing)*14.25))] w-2xs overflow-y-auto p-6">
                <Nav>{navItems}</Nav>
              </div>
            </div>
          </div>
          <div className="relative row-start-1 grid grid-cols-subgrid lg:col-start-3">
            <div className="isolate mx-auto grid w-full max-w-2xl grid-cols-1 gap-10 pt-10 md:pb-24 xl:max-w-5xl">
              <div className="px-4 sm:px-6 text-base/7 text-gray-700 dark:text-gray-300">
                <Suspense fallback={<Loading />}>{useRoutes(router)}</Suspense>
              </div>
            </div>
          </div>
          {/* TODO bottom nav */}
          <div className="row-start-5 grid lg:col-start-3">
            <div className="px-2 pt-10 pb-24">
              <div className="mx-auto flex w-full flex-col items-start gap-6 sm:flex-row sm:items-center sm:justify-between sm:gap-8 max-w-2xl lg:max-w-5xl">
                <div>{/* TODO scheme selector */}</div>
                <div className="flex flex-col gap-4 text-sm/6 text-gray-700 sm:flex-row sm:gap-2 sm:pr-4 dark:text-gray-400">
                  <span>Copyright @ 2025 Cameron Bytheway</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
