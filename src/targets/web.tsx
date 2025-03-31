import { StrictMode } from "react";
import { createRoot, hydrateRoot } from "react-dom/client";
import { LocationProvider } from "preact-iso";
import "./web/index.css";
import App from "../App.tsx";

const container = document.getElementById("root")!;

const root = (
  <StrictMode>
    <LocationProvider scope={import.meta.env.BASE_URL}>
      <App />
    </LocationProvider>
  </StrictMode>
);

if (import.meta.env.PROD) {
  hydrateRoot(container, root);
} else {
  createRoot(container).render(root);
}
