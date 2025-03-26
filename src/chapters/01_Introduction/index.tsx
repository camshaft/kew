import Content from "./content.mdx";
import { usePointToPoint } from "$/kew";
import Page from "@/Page.tsx";
import { useState } from "react";
import Queue from "@/Queue.tsx";
import { useInterval } from "@/use-interval.ts";
import Player from "@/Player.tsx";
import Slider from "@/Slider";
import Toggle from "@/Toggle";

export default function () {
  const [queue_capacity, setQueueCapacity] = useState(10);
  const [producer_capacity, setProducerCapacity] = useState(1);
  const [count, setCount] = useState(10);
  const [production_time, setProductionTime] = useState(1.0);
  const [consumption_time, setConsumptionTime] = useState(4.0);
  const [mode, setMode] = useState<"FIFO" | "LIFO">("FIFO");
  const [backpressure, setBackpressure] = useState(true);
  const [showAge, setShowAge] = useState(false);
  const [idx, _setIdx] = useState(0);
  const [playing, setPlaying] = useState(true);

  const states = usePointToPoint(
    {
      queue_capacity,
      count,
      production_time,
      producer_capacity,
      consumption_time,
      backpressure,
      lifo: mode == "LIFO",
    },
    (sim) => {
      return sim?.states() || [];
    }
  );

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

  if (!states.length) return;

  const time = states[idx]?.seconds || 0;

  return (
    <Page>
      {({ components }) => (
        <>
          <Content fifo={states} components={components} />

          <div className="grid grid-cols-2 gap-8 mt-10">
            <Slider min={1} max={50} value={count} onChange={setCount}>
              Item Count
            </Slider>
            <Slider
              min={1}
              max={20}
              value={producer_capacity}
              onChange={setProducerCapacity}
            >
              Producer Capacity
            </Slider>
            <Slider
              min={1}
              max={20}
              value={queue_capacity}
              onChange={setQueueCapacity}
            >
              Queue Capacity
            </Slider>
            <Slider
              min={1}
              max={10}
              unit="Seconds"
              value={production_time}
              onChange={setProductionTime}
            >
              Production Time
            </Slider>
            <Slider
              min={1}
              max={10}
              unit="Seconds"
              value={consumption_time}
              onChange={setConsumptionTime}
            >
              Consumption Time
            </Slider>
            <div className="grid grid-cols-3 gap-4">
              <Toggle value={backpressure} onChange={setBackpressure}>
                Backpressure
              </Toggle>
              <Toggle value={showAge} onChange={setShowAge}>
                Show Age
              </Toggle>
              <Toggle
                offLabel="FIFO"
                value={mode === "LIFO"}
                onChange={(v) => setMode(v ? "LIFO" : "FIFO")}
              >
                LIFO
              </Toggle>
            </div>
          </div>

          <Player
            max={states.length - 1}
            value={idx}
            onChange={setIdx}
            playing={playing}
            setPlaying={setPlaying}
            time={time}
          />

          <div className="h-96 max-h-96 overflow-y-scroll overflow-x-hidden">
            <Queue states={states} idx={idx} showAge={showAge}></Queue>
          </div>
        </>
      )}
    </Page>
  );
}
