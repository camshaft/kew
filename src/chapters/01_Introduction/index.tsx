import Content from "./coffee.mdx";
import Page from "@/Page.tsx";
import { usePointToPoint } from "$/kew";
import Queue from "@/Queue.tsx";
import { useInterval } from "@/use-interval.ts";
import Player from "@/Player.tsx";
import { useState, createContext } from "react";
import Slider from "@/Slider";

const SimContext = createContext({});

export default function () {
  const queue_capacity = 10;
  const count = 20;
  const [production_time, setProductionTime] = useState(1.0);
  const [consumption_time, setConsumptionTime] = useState(1.0);
  const [idx, _setIdx] = useState(0);
  const [playing, setPlaying] = useState(false);

  const states = usePointToPoint(
    {
      queue_capacity,
      count,
      production_time,
      consumption_time,
      backpressure: true,
      lifo: false,
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
    <SimContext.Provider
      value={{
        states,
        idx,
        setIdx,
        playing,
        setPlaying,
        time,
        production_time,
        consumption_time,
        setProductionTime,
        setConsumptionTime,
      }}
    >
      <Page>
        {({ components }) => (
          <>
            <Content components={{ ...components, Sim }} />
          </>
        )}
      </Page>
    </SimContext.Provider>
  );
}

function Sim({ showControls }) {
  return (
    <SimContext.Consumer>
      {({
        states,
        idx,
        setIdx,
        playing,
        setPlaying,
        time,
        production_time,
        consumption_time,
        setProductionTime,
        setConsumptionTime,
      }) => (
        <div className="mt-10 mb-10">
          {showControls && (
            <div>
              <Slider
                min={1}
                max={10}
                unit="Seconds"
                value={production_time}
                onChange={setProductionTime}
              >
                Arrival Rate
              </Slider>
              <Slider
                min={1}
                max={10}
                unit="Seconds"
                value={consumption_time}
                onChange={setConsumptionTime}
              >
                Service Rate
              </Slider>
            </div>
          )}

          <Player
            max={states.length - 1}
            value={idx}
            onChange={setIdx}
            playing={playing}
            setPlaying={setPlaying}
            time={time}
          />

          <div className="min-h-12">
            <Queue states={states} idx={idx}></Queue>
          </div>
        </div>
      )}
    </SimContext.Consumer>
  );
}
