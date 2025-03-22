---
title: Introduction
---

<div class="hero">
  <h1>Kew</h1>
</div>

```js
const run_time = view(
  Inputs.range([1, 10], { step: 1, value: 1, label: "Run Time" })
);
const client_sleep_time = view(
  Inputs.range([1, 10], { step: 1, value: 1, label: "Client Latency" })
);
const server_sleep_time = view(
  Inputs.range([1, 10], { step: 1, value: 1, label: "Server Latency" })
);
const queue_depth = view(
  Inputs.range([10, 100], { step: 1, value: 10, label: "Queue Depth" })
);
const queue_type = view(
  Inputs.radio(["fifo", "lifo"], { value: "fifo", label: "Queue Type" })
);
```

```js id=fifo_1 sim
sim(() => ({
  run_time: run_time || 3,
  actors: [
    {
      id: "client",
      name: "Client",
      operations: [
        {
          loop: [
            {
              [queue_type == "fifo" ? "push_back" : "push_front"]: {
                queue: "queue",
              },
            },
            {
              sleep: (client_sleep_time || 1) * 0.001,
            },
          ],
        },
      ],
    },
    {
      id: "server",
      name: "Server",
      operations: [
        {
          loop: [
            {
              pop_front: { queue: "queue" },
            },
            {
              sleep: (server_sleep_time || 1) * 0.001,
            },
          ],
        },
      ],
    },
  ],
  queues: [
    {
      id: "queue",
      type: { prefer_old: queue_depth || 10 },
    },
  ],
}));
```

```sql id=queue_latency
SELECT
  epoch_ms (timestamp) as x,
  value / 1000000 as y
FROM
  fifo_1
WHERE
  name = 'sojourn_time'
```

```js
const queueLatencyPlot = (width) =>
  Plot.plot({
    title: "Queue Latency",
    width,
    x: { grid: true, label: "Timestamp (ms)" },
    y: { grid: true, label: "Sojourn Time (ms)" },
    marks: [
      Plot.ruleY([0]),
      Plot.lineY(queue_latency, { x: "x", y: "y", tip: true }),
    ],
  });
```

<div>${resize(queueLatencyPlot)}</div>

```sql id=error_count
SELECT
  TRUNC (epoch_ms (timestamp) / 100) * 100 as x,
  SUM(value) as y
FROM
  fifo_1
WHERE
  name = 'push'
  AND kind = 'count'
  AND attr_reason = 'reject'
GROUP BY
  x
```

```js
const errorPlot = (width) =>
  Plot.plot({
    title: "Errors",
    width,
    x: { grid: true, label: "Timestamp (ms)" },
    y: { grid: true, label: "Errors" },
    marks: [
      Plot.ruleY([0]),
      Plot.rectY(
        error_count,
        Plot.binX({ y: "y" }, { x: "x", interval: 1, zero: true })
      ),
    ],
  });
```

<div>${resize(errorPlot)}</div>

<style>

.hero {
  display: flex;
  flex-direction: column;
  align-items: center;
  font-family: var(--sans-serif);
  margin: 4rem 0 8rem;
  text-wrap: balance;
  text-align: center;
}

.hero h1 {
  margin: 1rem 0;
  padding: 1rem 0;
  max-width: none;
  font-size: 14vw;
  font-weight: 900;
  line-height: 1;
  background: linear-gradient(30deg, var(--theme-foreground-focus), currentColor);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.hero h2 {
  margin: 0;
  max-width: 34em;
  font-size: 20px;
  font-style: initial;
  font-weight: 500;
  line-height: 1.5;
  color: var(--theme-foreground-muted);
}

@media (min-width: 640px) {
  .hero h1 {
    font-size: 90px;
  }
}

</style>
