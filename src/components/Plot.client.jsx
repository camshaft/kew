import * as Plot from "@observablehq/plot";
import { useEffect, useRef } from "react";

// Based on https://codesandbox.io/p/sandbox/plot-react-csr-p4cr7t?file=%2Fsrc%2FPlotFigure.jsx%3A1%2C1-18%2C1

export default function PlotFigure({ className, ...options }) {
  const containerRef = useRef(null);

  className = className ? className.split(/ +/) : undefined;

  useEffect(() => {
    if (options == null) return;
    const plot = Plot.plot(options);
    if (className) plot.classList.add(...className);
    containerRef.current.append(plot);
    return () => plot.remove();
  }, [options]);

  return <div ref={containerRef} />;
}
