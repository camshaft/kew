/// <reference types="vite/client" />
/// <reference types="vite-plugin-pages/client-react" />

import { Model } from "./data/model";
import { Sim } from "./data/sim";

interface ImportMetaEnv {
  readonly VITE_SSR_TARGET: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

declare module "*?fn=*" {
  export const sim: Sim;
  export const model: Model;
  export const params: any;

  export default model;
}
