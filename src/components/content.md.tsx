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
    h2({ children }: any) {
      return (
        <>
          {`\n${headerPrefix}## `}
          {children}
          {`\n\n`}
        </>
      );
    },
    h3({ children }: any) {
      return (
        <>
          {`\n${headerPrefix}### `}
          {children}
          {`\n\n`}
        </>
      );
    },
    h4({ children }: any) {
      return (
        <>
          {`\n${headerPrefix}#### `}
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
    ul({ children }: any) {
      return (
        <>
          {`\n`}
          {children}
          {`\n`}
        </>
      );
    },
    Math({ children }: any) {
      return `$$${children}$$`;
    },
    FIFO() {
      return "FIFO";
    },
    LIFO() {
      return "LIFO";
    },
  };
};
