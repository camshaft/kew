import { StrictMode } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { StaticRouter } from "react-router";
import App from "../App";

export { default as routes } from "~react-pages";

export async function render({ url }: { url: string }) {
  const head = "";

  // TODO swap out the html tags for native markdown syntax
  const html = renderToStaticMarkup(
    <StrictMode>
      <StaticRouter basename="/kew" location={url}>
        <App />
      </StaticRouter>
    </StrictMode>
  );

  return { html, head };
}
