import { StrictMode } from "react";
import { renderToStringAsync as renderToString } from "preact-render-to-string";
import { LocationProvider } from "preact-iso";
import { locationStub } from "preact-iso/prerender";
import App from "../App.tsx";

export { routes } from "@/routes.tsx";

export async function render({ url }: { url: string }) {
  console.log(`rendering ${url} to markdown`);

  const head = "";

  locationStub(url);

  const html = await renderToString(
    <StrictMode>
      <LocationProvider scope={import.meta.env.BASE_URL}>
        <App />
      </LocationProvider>
    </StrictMode>
  );

  return { html, head };
}
