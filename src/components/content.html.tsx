import clsx from "clsx";
import { Route } from "./routes";

// TODO add deep links

export default (_route: Route) => ({
  h1({ className, ...props }: any) {
    return (
      <h1
        {...props}
        className={clsx(
          "mt-2 text-3xl font-medium tracking-tight text-gray-950 dark:text-white",
          className
        )}
      />
    );
  },
  p({ className, ...props }: any) {
    return (
      <p
        {...props}
        className={clsx(
          "mt-6 text-base/7 text-gray-700 dark:text-gray-300",
          className
        )}
      />
    );
  },
});
