import {
  PlayIcon,
  BackwardIcon,
  ForwardIcon,
  PauseIcon,
} from "@heroicons/react/16/solid";

export interface Props {
  max: number;
  value: number;
  onChange: (value: number) => void;
  playing: boolean;
  setPlaying: (playing: boolean) => void;
  time: number;
}

export default function Player({
  max,
  value,
  onChange,
  playing,
  setPlaying,
  time,
}: Props) {
  const minutes = Math.floor(time / 60);
  let seconds = `${Math.floor(time % 60)}`;
  if (seconds.length == 1) seconds = `0${seconds}`;

  return (
    <div className="grid grid-cols-5 bg-amber-50 dark:bg-slate-950 p-4 rounded-md mt-2 mb-8">
      <input
        type="range"
        className="mb-4 col-span-5 h-3 bg-gray-200 rounded-lg cursor-pointer range-lg dark:bg-gray-700 accent-blue-600"
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
