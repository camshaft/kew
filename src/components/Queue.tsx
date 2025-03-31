import { motion, AnimatePresence as _AnimatePresence } from "motion/react";
import { Model } from "../data/model";
import * as m from "../data/model";
import clsx from "clsx";
import { useLaggingValue } from "./use-lagging-value";

export interface Props {
  states: Model[];
  idx: number;
  showAge?: boolean;
}

const AnimatePresence = import.meta.env.SSR
  ? ({ children }: any) => children
  : _AnimatePresence;

const AnimatedDiv = import.meta.env.SSR
  ? ({ ...props }) => <div {...props} />
  : motion.div;

export default function QueueVis({ states, idx, showAge }: Props) {
  idx = Math.max(idx, 0);
  idx = Math.min(idx, states.length - 1);
  showAge = !!showAge;

  const prevIdx = useLaggingValue(idx);

  const isForward = idx >= prevIdx;

  // TODO change the direction of animation
  //   console.log(isForward);

  if (!states.length) return false;

  const state = states[idx];

  const cols = state.queues.length;

  const children: any = [];

  state.queues.forEach((queue, idx) =>
    children.push(...Queue({ queue, column: idx + 1, state, showAge }))
  );

  return (
    <>
      <div
        className="grid gap-4"
        style={{ gridTemplateColumns: `repeat(${cols}, minmax(0, 1fr))` }}
      >
        <AnimatePresence>{children}</AnimatePresence>
      </div>
    </>
  );
}

function Queue({
  queue,
  column,
  state,
  showAge,
}: {
  queue: m.Queue;
  column: number;
  state: Model;
  showAge: boolean;
}) {
  const children = [];

  children.push(
    <div
      key={`q${queue.id}`}
      className="z-10"
      style={{ gridColumnStart: column, gridColumnEnd: column + 1 }}
    >
      <QueueLabel queue={queue} showAge={showAge} />
    </div>
  );

  queue.items.forEach((item, idx) =>
    children.push(Item({ item, column, row: idx + 2, state, showAge }))
  );

  return children;
}

function QueueLabel({ queue, showAge }: { queue: m.Queue; showAge: boolean }) {
  return (
    <div
      className={clsx(
        "p-4 row-start-1 row-span-1 rounded text-center select-none",
        {
          "bg-amber-600": queue.isGroup,
          "bg-blue-400": !queue.isGroup,
          "grid grid-cols-2 items-center pl-8 pr-8": showAge,
        }
      )}
    >
      <div className={showAge ? "text-left" : "text-center"}>{queue.name}</div>
      {showAge && (
        <div className="text-right text-xs">
          Avg Sojourn: {formatNumber(queue.items.sojournTimes.average)}
        </div>
      )}
    </div>
  );
}

function Item({
  item,
  column,
  row,
  state,
  showAge,
}: {
  item: m.Item;
  column: number;
  row: number;
  state: Model;
  showAge: boolean;
}) {
  return (
    <AnimatedDiv
      key={`i${item.id}`}
      exit={{ transform: "translateY(-100px)", opacity: 0, height: 0 }}
      initial={{ transform: "translateY(-100px)", opacity: 0, height: 0 }}
      animate={{ transform: "translateY(0px)", opacity: 1, height: "auto" }}
      transition={{ type: "tween", duration: 0.2 }}
      layout
      style={{
        gridColumnStart: column,
        gridColumnEnd: column + 1,
        gridRowStart: row,
        gridRowEnd: row + 1,
      }}
    >
      <ItemLabel item={item} state={state} showAge={showAge} />
    </AnimatedDiv>
  );
}

function ItemLabel({
  item,
  state,
  showAge,
}: {
  item: m.Item;
  state: Model;
  showAge: boolean;
}) {
  return (
    <div
      className={clsx(
        "ml-4 mr-4 px-3 py-3 select-none rounded-lg bg-emerald-500 text-gray-900 dark:text-white",
        {
          "grid grid-cols-2 items-center": showAge,
        }
      )}
    >
      <div className={showAge ? "text-left" : "text-center"}>
        Item {item.id + 1}
      </div>
      {showAge && (
        <div className="text-right text-xs">
          Age: {formatNumber(item.lifetime(state.seconds))}
        </div>
      )}
    </div>
  );
}

function formatNumber(v: number): string {
  return v.toLocaleString(undefined, {
    style: "decimal",
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  });
}
