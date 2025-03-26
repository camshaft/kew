import { PlotOptions } from "@observablehq/plot";
// @ts-ignore
import Client from "./Plot.client.jsx";
// @ts-ignore
import Server from "./Plot.server.jsx";

const Inner = import.meta.env.SSR ? Server : Client;

export default function Plot(props: PlotOptions) {
  return <Inner {...props} />;
}
