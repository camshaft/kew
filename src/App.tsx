import { Suspense, useEffect, useState } from "react";
import Loading from "@/Loading.tsx";
import Nav from "@/Nav.tsx";
import { router } from "@/routes";
import clsx from "clsx";
import KeyBindings from "@/KeyBindings";
import { Breadcrumb } from "@/Breadcrumb";
import { Bars3Icon, XMarkIcon } from "@heroicons/react/16/solid";
import { useLocation } from "preact-iso";

export default function App() {
  if (import.meta.env.VITE_SSR_TARGET == "md") return router;

  const [navVisible, setNavVisible] = useState(false);

  return (
    <>
      <KeyBindings onEscape={() => setNavVisible(false)} />
      <div className="isolate dark:bg-gray-900">
        <TopBar visible={navVisible} setVisible={setNavVisible} />
        <div className="grid min-h-dvh grid-cols-1 grid-rows-[1fr_1px_auto_1px_auto] pt-26.25 lg:grid-cols-[var(--container-2xs)_2.5rem_minmax(0,1fr)_2.5rem] lg:pt-14.25 xl:grid-cols-[var(--container-2xs)_2.5rem_minmax(0,1fr)_2.5rem]">
          <SideBar visible={navVisible} setVisible={setNavVisible} />
          <Content />
          <BottomBar />
        </div>
      </div>
    </>
  );
}

interface BarProps {
  visible: boolean;
  setVisible: (visible: boolean) => void;
}

function TopBar({ visible, setVisible }: BarProps) {
  let icon: any = visible ? <XMarkIcon /> : <Bars3Icon />;

  // TODO figure out why icons aren't working
  if (import.meta.env.SSR) icon = false;

  return (
    <div className="fixed inset-x-0 top-0 z-10 border-b border-gray-950/5 dark:border-white/10">
      <div className="bg-white dark:bg-gray-950">{/* TODO top nav? */}</div>
      <div className="flex h-14 items-center border-t border-gray-950/5 bg-white px-4 sm:px-6 lg:hidden dark:border-white/10 dark:bg-gray-950">
        <button
          className="relative inline-grid size-7 place-items-center rounded-md text-gray-950 hover:bg-gray-950/5 dark:text-white dark:hover:bg-white/10 -ml-1.5"
          type="button"
          aria-label="Open navigation menu"
          onClick={() => setVisible(!visible)}
        >
          {icon}
        </button>
        <Breadcrumb />
      </div>
    </div>
  );
}

function SideBar({ visible, setVisible }: BarProps) {
  if (!import.meta.env.SSR) {
    const location = useLocation();

    useEffect(() => {
      setVisible(false);
    }, [location.path]);
  }

  return (
    <div
      className={clsx(
        "relative col-start-1 row-span-full row-start-1 max-lg:absolute max-lg:top-0 max-lg:bottom-0 transition-transform",
        {
          "max-lg:-translate-x-96": !visible,
        }
      )}
    >
      {visible && (
        <div
          class="fixed z-40 inset-x-0 top-14 bottom-0 bg-white/80 dark:bg-gray-950/90"
          onClick={() => setVisible(false)}
        />
      )}
      <div className="absolute inset-0">
        <div className="z-50 sticky top-14 bottom-0 left-0 dark:bg-gray-900 h-full max-h-[calc(100dvh-(var(--spacing)*14.25))] w-2xs overflow-y-auto p-6">
          <Nav />
        </div>
      </div>
    </div>
  );
}

function Content() {
  return (
    <div className="relative row-start-1 grid grid-cols-subgrid lg:col-start-3">
      <div className="isolate mx-auto grid w-full max-w-2xl grid-cols-1 gap-10 pt-10 md:pb-24 xl:max-w-5xl">
        <div className="px-4 sm:px-6 text-base/7 text-gray-700 dark:text-gray-300">
          <Suspense fallback={<Loading />}>{router}</Suspense>
        </div>
      </div>
    </div>
  );
}

function BottomBar() {
  return (
    <div className="row-start-5 grid lg:col-start-3">
      <div className="px-2 pt-10 pb-10">
        <div className="mx-auto flex w-full flex-col items-start gap-6 sm:flex-row sm:items-center sm:justify-between sm:gap-8 max-w-2xl lg:max-w-5xl">
          <div>{/* TODO scheme selector */}</div>
          <div className="flex flex-col gap-4 text-sm/6 text-gray-700 sm:flex-row sm:gap-2 sm:pr-4 dark:text-gray-400">
            <span>Copyright @ 2025 Cameron Bytheway</span>
          </div>
        </div>
      </div>
    </div>
  );
}
