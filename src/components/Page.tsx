import { useRoute } from "./routes";
import createComponents from "./content.tsx";

export interface Props {
  children: ({ components }: { components: any }) => any;
}

export default function Page({ children }: Props) {
  const route = useRoute();

  if (!route) return;

  const title = route.title;

  const components = createComponents(route);

  const content = children({ components });

  return (
    <>
      <components.h1>{title}</components.h1>
      {content}
    </>
  );
}
