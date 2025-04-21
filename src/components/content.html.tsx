import clsx from "clsx";
import { Route } from "./routes";
import Math from "./Math";

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
  h2({ className, ...props }: any) {
    return (
      <h1
        {...props}
        className={clsx(
          "mt-6 text-2xl font-medium tracking-tight text-gray-950 dark:text-white",
          className
        )}
      />
    );
  },
  h3({ className, ...props }: any) {
    return (
      <h1
        {...props}
        className={clsx(
          "mt-6 text-xl font-medium tracking-tight text-gray-950 dark:text-white",
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
  ul({ className, ...props }: any) {
    return (
      <ul
        {...props}
        className={clsx(
          "mt-6 list-disc text-base/7 text-gray-700 dark:text-gray-300",
          className
        )}
      />
    );
  },
  ol({ className, ...props }: any) {
    return (
      <ol
        {...props}
        className={clsx(
          "mt-6 list-decimal text-base/7 text-gray-700 dark:text-gray-300",
          className
        )}
      />
    );
  },
  li({ className, ...props }: any) {
    return (
      <li
        {...props}
        className={clsx(
          "mt-2 text-base/7 text-gray-700 dark:text-gray-300",
          className
        )}
      />
    );
  },
  Math({ ...props }: any) {
    return <Math {...props} />;
  },
  FIFO() {
    // TODO add hover definition and link
    return <span>FIFO</span>;
  },
  LIFO() {
    // TODO add hover definition and link
    return <span>LIFO</span>;
  },
});
