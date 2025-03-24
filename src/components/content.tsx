import html from "./content.html.tsx";
import md from "./content.md.tsx";

export default import.meta.env.SSR && import.meta.env.VITE_SSR_TARGET == "md"
  ? md
  : html;
