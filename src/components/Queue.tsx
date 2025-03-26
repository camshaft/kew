import { motion, AnimatePresence } from "motion/react";
import { Model } from "../data/model";
import * as m from "../data/model";
import clsx from "clsx";
import { useLaggingValue } from "./use-lagging-value";

export interface Props {
  states: Model[];
  idx: number;
}

export default function QueueVis({ states, idx }: Props) {
  idx = Math.max(idx, 0);
  idx = Math.min(idx, states.length - 1);

  const prevIdx = useLaggingValue(idx);

  const isForward = idx >= prevIdx;

  // TODO change the direction of animation
  //   console.log(isForward);

  const state = states[idx];

  const cols = state.queues.length;

  const children: any = [];

  state.queues.forEach((queue, idx) =>
    children.push(...Queue({ queue, column: idx + 1 }))
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

function Queue({ queue, column }: { queue: m.Queue; column: number }) {
  const children = [];

  children.push(
    <div
      key={`q${queue.id}`}
      className={clsx(
        " p-4 row-start-1 row-span-1 rounded-lg text-center select-none",
        {
          "bg-amber-600": queue.isGroup,
          "bg-blue-400": !queue.isGroup,
        }
      )}
      style={{ gridColumnStart: column, gridColumnEnd: column + 1 }}
    >
      {queue.name}
    </div>
  );

  queue.items.forEach((id, idx) =>
    children.push(Item({ id, column, row: idx + 2 }))
  );

  return children;
}

function Item({
  id,
  column,
  row,
}: {
  id: number;
  column: number;
  row: number;
}) {
  return (
    <motion.div
      key={`i${id}`}
      exit={{ transform: "translateX(100px)", opacity: 0, height: 0 }}
      initial={{ transform: "translateX(-100px)", opacity: 0, height: 0 }}
      animate={{ transform: "translateX(0px)", opacity: 1, height: "auto" }}
      transition={{ type: "spring", bounce: 0.2, duration: 0.5 }}
      layout
      style={{
        gridColumnStart: column,
        gridColumnEnd: column + 1,
        gridRowStart: row,
        gridRowEnd: row + 1,
      }}
    >
      <div className="bg-emerald-500 rounded-4xl p-4 text-center select-none">
        Item {id + 1}
      </div>
    </motion.div>
  );
}
