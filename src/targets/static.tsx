import { StrictMode } from "react";
import { renderToString } from "react-dom/server";
import { StaticRouter } from "react-router";
import App from "../App";

export { default as routes } from "~react-pages";

export async function render({ url, title }: { url: string; title: string }) {
  const head = renderToString(
    <>
      <title>{title}</title>
    </>
  );

  const html = renderToString(
    <StrictMode>
      <StaticRouter basename="/kew" location={url}>
        <App />
      </StaticRouter>
    </StrictMode>
  );

  return { html, head };
}
