import { useEffect } from "react";
import { useLocation } from "preact-iso";
import { pathToRoute } from "./routes.tsx";

let currentListener = {
  fn(_event: KeyboardEvent) {},
};

export default function KeyBindings({ onEscape }: { onEscape: () => void }) {
  if (import.meta.env.SSR) return;

  const location = useLocation();
  let route = pathToRoute.get(location.path);

  currentListener.fn = (event: KeyboardEvent) => {
    switch (event.key) {
      case "ArrowLeft":
        const prev = route?.prev?.path;
        if (prev) location.route(prev);
        break;
      case "ArrowRight":
        const next = route?.next?.path;
        if (next) location.route(next);
        break;
      case "Escape":
        onEscape();
        break;
      default:
        break;
    }
  };

  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      currentListener.fn(event);
    }

    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  return null;
}
