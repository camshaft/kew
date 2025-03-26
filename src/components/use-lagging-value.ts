import { useRef } from "react";

export function useLaggingValue<T>(current: T): T {
  const ref = useRef<T>(current);
  const prev = ref.current;
  ref.current = current;
  return prev;
}
