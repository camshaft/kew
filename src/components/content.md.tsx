import { Route } from "./routes";

export default (route: Route) => {
  let headerPrefix = "";
  for (let i = 0; i < route.depth; i++) {
    headerPrefix += "#";
  }
  return {
    h1({ children }: any) {
      return (
        <>
          {route.depth == 0 && `\\newpage{}`}
          {`\n${headerPrefix}# `}
          {children}
          {`\n\n`}
        </>
      );
    },
    p({ children }: any) {
      return (
        <>
          {children}
          {`\n`}
        </>
      );
    },
  };
};
