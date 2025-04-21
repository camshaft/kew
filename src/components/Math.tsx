import { useMemo } from "react";
import { default as katex, KatexOptions } from "katex";

export interface Props {
  className?: string;
  children: string;
  display?: boolean;
  options?: KatexOptions;
}

export default function Math({
  className = "",
  children = "",
  display = false,
  options = {},
}: Props) {
  const Wrapper = display ? "div" : "span";

  if (typeof children !== "string")
    throw new Error("Children prop must be a katex string");

  const renderedKatex = useMemo(() => {
    let result;

    try {
      result = katex.renderToString(children, {
        ...options,
        displayMode: display,
        throwOnError: true,
        globalGroup: true,
        trust: true,
        strict: false,
      });
    } catch (error) {
      console.error(error);
      result = katex.renderToString(children, {
        ...options,
        displayMode: display,
        throwOnError: false,
        strict: "ignore",
        globalGroup: true,
        trust: true,
      });
    }

    return result;
  }, [children]);

  return (
    <Wrapper
      className={className}
      dangerouslySetInnerHTML={{ __html: renderedKatex || "" }}
    />
  );
}
