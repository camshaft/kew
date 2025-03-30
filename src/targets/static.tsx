import { StrictMode } from "react";
import { renderToStringAsync as renderToString } from "preact-render-to-string";
import { LocationProvider } from "preact-iso";
import { locationStub } from "preact-iso/prerender";
import App from "../App.tsx";

export { routes } from "@/routes.tsx";

export async function render({ url, title }: { url: string; title: string }) {
  console.log(`pre-rendering ${url}`);

  const head = await renderToString(
    <>
      <title>{title}</title>
    </>
  );

  locationStub(url);

  const html = await renderToString(
    <StrictMode>
      <LocationProvider scope={import.meta.env.BASE_URL}>
        <App />
      </LocationProvider>
    </StrictMode>
  );

  if (!html) throw new Error(`could not render ${url}`);

  return { html, head };
}
