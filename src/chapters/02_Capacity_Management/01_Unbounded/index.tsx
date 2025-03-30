import Content from "./content.mdx";
import Page from "@/Page.tsx";
import Plot from "@/Plot";
import { lineY } from "@observablehq/plot";
// @ts-ignore
import { model as states } from "$/kew.static.js?fn=PointToPoint&count=50&queue_capacity=1000&production_time=1&consumption_time=2";
import { Model } from "~/data/model";
import { useState } from "preact/compat";
import { useInterval } from "@/use-interval";
import Queue from "@/Queue";

export default function () {
  return (
    <Page>
      {({ components }) => (
        <Content components={{ ...components, ...localComponents }} />
      )}
    </Page>
  );
}

const localComponents = {
  FigureOne,
};

function FigureOne() {
  const [idx, _setIdx] = useState(0);
  const [playing, setPlaying] = useState(true);

  function setIdx(idx: number, pause?: boolean) {
    if (idx > states.length - 1) {
      idx = 0;
    }

    if (idx < 0) {
      idx = states.length - 1;
    }

    if (pause) {
      setPlaying(true);
    }

    _setIdx(idx);
  }

  useInterval(
    () => {
      setIdx(idx + 1);
    },
    1000,
    playing
  );

  const state: Model = states[idx];

  return (
    <>
      <div className="relative overflow-y-hidden h-60">
        <Queue states={states} idx={idx} />
        <div className="z-10 bg-linear-to-t from-white to-white/40 dark:from-gray-900 dark:to-gray-900/0 absolute bottom-0 h-20 w-full" />
      </div>
      <Plot
        marks={[
          lineY(state.queues[1].itemCounts.entries, {
            x: { value: "timestamp", label: "Time (s)" },
            y: { value: "value", label: "Items in queue" },
          }),
        ]}
      />
    </>
  );
}
