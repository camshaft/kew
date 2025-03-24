import { useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { pathToRoute } from "./routes.tsx";

let currentListener = {
  fn(_event: KeyboardEvent) {},
};

export default function KeyBindings({ onEscape }: { onEscape: () => void }) {
  if (import.meta.env.SSR) return;

  const navigate = useNavigate();
  const location = useLocation();
  let route = pathToRoute.get(location.pathname);

  currentListener.fn = (event: KeyboardEvent) => {
    switch (event.key) {
      case "ArrowLeft":
        const prev = route?.meta?.prev;
        if (prev) navigate(prev);
        break;
      case "ArrowRight":
        const next = route?.meta?.next;
        if (next) navigate(next);
        break;
      case "Escape":
        onEscape();
        break;
      default:
        break;
    }

    switch (event.key) {
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
