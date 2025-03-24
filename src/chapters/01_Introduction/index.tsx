import Content from "./content.mdx";
import { useFifo } from "$/fifo";
import Page from "@/Page.tsx";
import { useState } from "react";

export default function () {
  const [depth, setDepth] = useState(10);
  const [seconds, setSeconds] = useState(5);

  const fifo = useFifo({ depth, seconds });

  if (!fifo) return;

  return (
    <Page>
      {({ components }) => (
        <Content
          depth={depth}
          setDepth={setDepth}
          seconds={seconds}
          setSeconds={setSeconds}
          fifo={fifo}
          components={components}
        />
      )}
    </Page>
  );
}
