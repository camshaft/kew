import Content from "./content.mdx";
import { usePointToPoint } from "$/kew";
import Page from "@/Page.tsx";
import { useState } from "react";
import Queue from "@/Queue.tsx";
import { useInterval } from "@/use-interval.ts";
import Player from "@/Player.tsx";
import Slider from "@/Slider";
import Toggle from "@/Toggle";
import { useLocation } from "preact-iso";

export default function () {
  const location = useLocation();
  const params = new URLSearchParams(location.query);

  function getInt(name: string, fallback: number): number {
    if (params.has(name)) return parseInt(params.get(name) as string);
    return fallback;
  }

  function getFloat(name: string, fallback: number): number {
    if (params.has(name)) return parseFloat(params.get(name) as string);
    return fallback;
  }

  function getBool(name: string, fallback: boolean): boolean {
    if (params.has(name)) return params.get(name) != "false";
    return fallback;
  }

  const [queue_capacity, setQueueCapacity] = useState(
    getInt("queue-capacity", 1)
  );
  const [producer_capacity, setProducerCapacity] = useState(
    getInt("producer-capacity", 1)
  );
  const [count, setCount] = useState(getInt("count", 10));
  const [production_time, setProductionTime] = useState(
    getFloat("production-time", 1.0)
  );
  const [consumption_time, setConsumptionTime] = useState(
    getFloat("consumption-time", 4.0)
  );
  const [mode, setMode] = useState<"FIFO" | "LIFO">(getBool('lifo', false) ? "LIFO" : "FIFO");
  const [backpressure, setBackpressure] = useState(
    getBool("backpressure", true)
  );
  const [showAge, setShowAge] = useState(getBool("show-age", false));
  const [prefer_recent, setPreferNew] = useState(
    getBool("prefer-recent", false)
  );
  const [idx, _setIdx] = useState(getInt("step", 0));
  const [playing, setPlaying] = useState(getBool("playing", true));

  const states = usePointToPoint(
    {
      queue_capacity,
      count,
      production_time,
      producer_capacity,
      consumption_time,
      backpressure,
      lifo: mode == "LIFO",
      prefer_recent,
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
          </div>

          <div className="grid grid-cols-4 gap-4">
            <Toggle value={backpressure} onChange={setBackpressure}>
              Backpressure
            </Toggle>
            <Toggle
              offLabel="FIFO"
              value={mode === "LIFO"}
              onChange={(v) => setMode(v ? "LIFO" : "FIFO")}
            >
              LIFO
            </Toggle>
            <Toggle value={showAge} onChange={setShowAge}>
              Show Age
            </Toggle>
            <Toggle
              offLabel="Oldest"
              value={prefer_recent}
              onChange={setPreferNew}
            >
              Newest
            </Toggle>
          </div>

          <Player
            max={states.length - 1}
            value={idx}
            onChange={setIdx}
            playing={playing}
            setPlaying={setPlaying}
            time={time}
          />

          <div className="">
            <Queue states={states} idx={idx} showAge={showAge}></Queue>
          </div>
        </>
      )}
    </Page>
  );
}
