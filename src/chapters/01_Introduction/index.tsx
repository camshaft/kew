import Content from "./content.mdx";
import { useFifo } from "$/fifo";
import Page from "@/Page.tsx";
import { useId, useState } from "react";
import Queue from "@/Queue.tsx";
import { useInterval } from "@/use-interval.ts";
import {
  PlayIcon,
  BackwardIcon,
  ForwardIcon,
  PauseIcon,
} from "@heroicons/react/16/solid";

export default function () {
  const [depth, setDepth] = useState(10);
  const [count, setCount] = useState(10);
  const [arrival_rate, setArrivalRate] = useState(1.0);
  const [departure_rate, setDepartureRate] = useState(4.0);
  const [idx, _setIdx] = useState(0);
  const [playing, setPlaying] = useState(true);

  const states = useFifo(
    { depth, count, arrival_rate, departure_rate },
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

  return (
    <Page>
      {({ components }) => (
        <>
          <Content fifo={states} components={components} />

          <div className="grid grid-cols-2 gap-8 mt-10">
            <Slider min={1} max={20} value={depth} onChange={setDepth}>
              Max Queue Depth
            </Slider>
            <Slider min={1} max={50} value={count} onChange={setCount}>
              Item Count
            </Slider>
            <Slider
              min={1}
              max={10}
              unit="Seconds"
              value={arrival_rate}
              onChange={setArrivalRate}
            >
              Production Time
            </Slider>
            <Slider
              min={1}
              max={10}
              unit="Seconds"
              value={departure_rate}
              onChange={setDepartureRate}
            >
              Consumption Time
            </Slider>
          </div>

          <Player
            max={states.length - 1}
            value={idx}
            onChange={setIdx}
            playing={playing}
            setPlaying={setPlaying}
            time={states[idx].seconds}
          />

          <div className="h-96 max-h-96 overflow-y-scroll overflow-x-hidden">
            <Queue states={states} idx={idx}></Queue>
          </div>
        </>
      )}
    </Page>
  );
}

interface SliderProps {
  min: number;
  max: number;
  value: number;
  onChange: (value: number) => void;
  unit?: string;
  children: any;
}

function Slider({ min, max, value, onChange, children, unit }: SliderProps) {
  const id = useId();
  return (
    <div className="mb-4 grid grid-cols-3">
      <label
        htmlFor={id}
        className="mb-2 text-sm col-span-2 font-medium text-gray-900 dark:text-white"
      >
        {children}
      </label>
      <span className="mb-2 text-sm col-span-1 font-medium text-gray-900 dark:text-white text-right">
        {value} {unit && `${unit}`}
      </span>
      <input
        id={id}
        type="range"
        min={min}
        max={max}
        className="col-span-3 h-3 bg-gray-200 rounded-lg cursor-pointer range-lg dark:bg-gray-700 accent-red-600"
        value={value}
        onChange={(evt) => onChange(parseInt(evt.target.value))}
      />
    </div>
  );
}

interface PlayerProps {
  max: number;
  value: number;
  onChange: (value: number) => void;
  playing: boolean;
  setPlaying: (playing: boolean) => void;
  time: number;
}

function Player({
  max,
  value,
  onChange,
  playing,
  setPlaying,
  time,
}: PlayerProps) {
  const minutes = Math.floor(time / 60);
  let seconds = `${Math.floor(time % 60)}`;
  if (seconds.length == 1) seconds = `0${seconds}`;

  return (
    <div className="grid grid-cols-5 bg-amber-50 dark:bg-slate-950 p-4 rounded-md mt-2 mb-8">
      <input
        type="range"
        className="mb-4 col-span-5 h-3 bg-gray-200 rounded-lg cursor-pointer range-lg dark:bg-gray-700 accent-red-600"
        value={value}
        min="0"
        max={max}
        onChange={(e) => {
          setPlaying(false);
          onChange(parseInt(e.target.value));
        }}
      />
      <div className="col-start-2 flex justify-center">
        <button
          onClick={() => {
            setPlaying(false);
            onChange(value - 1);
          }}
        >
          <BackwardIcon className="w-10" />
        </button>
      </div>
      <div className="flex justify-center">
        <button onClick={() => setPlaying(!playing)}>
          {playing ? (
            <PauseIcon className="w-10" />
          ) : (
            <PlayIcon className="w-10" />
          )}
        </button>
      </div>
      <div className="flex justify-center">
        <button
          onClick={() => {
            setPlaying(false);
            onChange(value + 1);
          }}
        >
          <ForwardIcon className="w-10" />
        </button>
      </div>
      <div className="text-center text-lg pt-1 pr-10">
        {minutes}:{seconds}
      </div>
    </div>
  );
}
