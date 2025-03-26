import { useEffect, useRef } from "react";

export function useInterval(
  callback: () => void,
  delay: number,
  isActive?: boolean
) {
  const savedCallback = useRef(callback);

  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  useEffect(() => {
    if (isActive === false) return;
    function tick() {
      savedCallback.current();
    }
    if (delay !== null) {
      let id = setInterval(tick, delay);
      return () => clearInterval(id);
    }
  }, [delay, isActive]);
}
