import Content from "./content.mdx";
import { useAcceptQueue } from "$/kew";
import Page from "@/Page.tsx";
import { useState } from "react";
import Slider from "@/Slider";
import Toggle from "@/Toggle";
import { useLocation } from "preact-iso";
import Plot from "@/Plot";
import { axisFy, axisY, bin, binX, crosshairX, dot, lineY, marks, rect } from "@observablehq/plot";

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
    getInt("queue-capacity", 100)
  );
  const [service_time, setServiceTime] = useState(
    getInt("service-time", 0.20)
  );
  const [baseline_duration, setBaselineDuration] = useState(
    getFloat("baseline-duration", 10.0)
  );
  const [burst_duration, setBurstDuration] = useState(
    getFloat("burst-duration", 10.0)
  );
  const [recovery_duration, setRecoveryDuration] = useState(
    getFloat("recovery-duration", 10.0)
  );
  const [baseline_client_count, setBaselineClientCount] = useState(
    getInt("baseline-client-count", 1)
  );
  const [baseline_client_tps, setBaselineClientTps] = useState(
    getInt("baseline-client-tps", 1)
  );
  const [burst_client_count, setBurstClientCount] = useState(
    getInt("burst-client-count", 1)
  );
  const [burst_client_tps, setBurstClientTps] = useState(
    getInt("burst-client-tps", 1)
  );
  const [worker_count, setWorkerCount] = useState(
    getInt("worker-count", 10)
  );
  const [codel_multiplier, setCodelMultiplier] = useState(getInt("codel-mult", 0));
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

  const state = useAcceptQueue(
    {
      queue_capacity,
      baseline_duration,
      baseline_client_count,
      baseline_client_tps,
      burst_client_count,
      burst_client_tps,
      worker_count,
      service_time,
      burst_duration,
      recovery_duration,
      codel_multiplier,
      lifo: mode == "LIFO",
      prefer_recent,
    },
    (sim) => {
      const states = sim?.states() || [];
      return states[states.length - 1];
    }
  );

  if (!state) return;

  // const time = states[idx]?.seconds || 0;

  const channels = {
    timestamp: 'timestamp',
    value: 'value',
  };

  const tip = {
    format: {
      timestamp: false,
      x: false
    }
  };

  const queueDepth = state.queues[1].itemCounts.entries;
  const queueDepthBin = binX({
    y: 'max',
  }, {
    x: 'timestamp',
    y: 'value',
    stroke: 'blue',
    channels,
    tip,
  });

  const failedRequests = state.queues[0].items.pops.entries.concat(state.queues[1].items.pops.entries);
  const failedRequestsBin = binX({
    y: 'sum'
  }, {
    x: 'timestamp',
    y: 'value',
    stroke: 'red',
    channels,
    tip,
  });

  const successfulRequests = state.queues[2].items.pops.entries;
  const successfulRequestsBin = binX({
    y: 'sum'
  }, {
    x: 'timestamp',
    y: 'value',
    stroke: 'green',
    channels,
    tip,
  });

  const maxLifetime = state.lifetimes.max;
  const y2Domain = [];
  for (let i = 0; i < maxLifetime * 10; i += maxLifetime) {
    y2Domain.push({
      x: 0,
      y: i,
    })
  }

  const plot = (
    <Plot
      width={1200}
      height={700}
      className="mt-20 min-w-full h-auto"
      x={{
        label: "Time",
      }}
      y={{
        label: "Count",
        axis: "left",
      }}
      color={{
        legend: true,
        domain: [
          "Queue Depth", "Failed Requests", "Successful Requests",
        ],
        range: [
          "blue", "red", "green",
        ],
      }}
      marks={[
        // queue depth
        lineY(queueDepth, queueDepthBin),
        crosshairX(queueDepth, queueDepthBin),
        // failed requests
        lineY(failedRequests, failedRequestsBin),
        crosshairX(failedRequests, failedRequestsBin),
        // successful requests
        lineY(successfulRequests, successfulRequestsBin),
        crosshairX(successfulRequests, successfulRequestsBin),
      ]}
    />
  );

  const latencyPlot = (
    <Plot
      width={1200}
      height={700}
      color={{
        scheme: "Turbo",
      }}
      className="mt-10 min-w-full h-auto absolute -z-10"
      x={{
        axis: false,
      }}
      y={{
        label: "Request Latency (s)",
        axis: "right",
      }}
      marginLeft={50}
      marginBottom={50}
      marks={[
        // latency
        dot(state.lifetimes.entries, {
          x: 'timestamp',
          y: 'value',
        }),
        // rect(state.lifetimes.entries, bin({
        //   fill: 'sum',
        // }, {
        //   x: 'timestamp',
        //   y: 'value',
        // }))
      ]}
    />
  );

  return (
    <Page>
      {({ components }) => (
        <>
          {/* <Content fifo={states} components={components} /> */}

          <div className="grid grid-cols-2 gap-8 mt-10">
            <Slider
              min={1}
              max={500}
              value={queue_capacity}
              onChange={setQueueCapacity}
            >
              Queue Capacity
            </Slider>
            <Slider
              min={0.1}
              step={0.1}
              max={2}
              value={service_time}
              onChange={setServiceTime}
            >
              Service Time
            </Slider>
            <Slider
              min={1}
              max={20}
              value={worker_count}
              onChange={setWorkerCount}
            >
              Worker Count
            </Slider>
            {/* <Slider
              min={0}
              max={5}
              value={codel_multiplier}
              onChange={setCodelMultiplier}
            >
              CoDeL Multiplier
            </Slider> */}
            <span></span>
            <Slider
              min={1}
              max={20}
              value={baseline_client_count}
              onChange={setBaselineClientCount}
            >
              Baseline Client Count
            </Slider>
            <Slider
              min={1}
              max={100}
              value={baseline_client_tps}
              onChange={setBaselineClientTps}
            >
              Baseline TPS
            </Slider>
            <Slider
              min={1}
              max={20}
              value={burst_client_count}
              onChange={setBurstClientCount}
            >
              Burst Client Count
            </Slider>
            <Slider
              min={1}
              max={100}
              value={burst_client_tps}
              onChange={setBurstClientTps}
            >
              Burst TPS
            </Slider>
            <Slider
              min={1}
              max={120}
              value={baseline_duration}
              onChange={setBaselineDuration}
            >
              Baseline Duration
            </Slider>
            <Slider
              min={1}
              max={120}
              value={burst_duration}
              onChange={setBurstDuration}
            >
              Burst Duration
            </Slider>
            <Slider
              min={1}
              max={120}
              value={recovery_duration}
              onChange={setRecoveryDuration}
            >
              Recovery Duration
            </Slider>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <Toggle
              offLabel="FIFO"
              value={mode === "LIFO"}
              onChange={(v) => setMode(v ? "LIFO" : "FIFO")}
            >
              LIFO
            </Toggle>
            <Toggle
              offLabel="Oldest"
              value={prefer_recent}
              onChange={setPreferNew}
            >
              Newest
            </Toggle>
          </div>

          <div className="relative">
            {latencyPlot} {plot}
          </div>

          {/* <Player
            max={states.length - 1}
            value={idx}
            onChange={setIdx}
            playing={playing}
            setPlaying={setPlaying}
            time={time}
          />

          <div className="">
            <Queue states={states} idx={idx} showAge={showAge}></Queue>
          </div> */}
        </>
      )}
    </Page>
  );
}
