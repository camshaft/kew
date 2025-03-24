/// <reference types="vite/client" />
/// <reference types="vite-plugin-pages/client-react" />

interface ImportMetaEnv {
  readonly VITE_SSR_TARGET: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
