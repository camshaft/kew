import { Model } from "./model.ts";
import * as m from "./model.ts";

export class Sim {
  public steps: Step[];
  public queues: Queue[];

  constructor(steps: Step[] = [], queues: Queue[] = []) {
    this.steps = steps;
    this.queues = queues;
  }

  public pushStep(step: Step) {
    this.steps.push(step);
  }

  public pushQueue(queue: Queue): number {
    const id = this.queues.length;
    this.queues.push(queue);
    return id;
  }

  public states(): Model[] {
    let current = new Model();
    current.queues = this.queues.map(
      (queue, idx) => new m.Queue(idx, queue.name, queue instanceof Group)
    );
    const out = [];
    for (let i = 0; i < this.steps.length; i++) {
      const next = current.clone();
      this.apply(next);
      out.push(next);
      current = next;
    }
    return out;
  }

  public apply(model: Model) {
    const stepId = model.step + 1;
    if (stepId >= this.steps.length) {
      return;
    }
    this.steps[stepId].apply(model);
    model.step = stepId;
    model.lifetimes.finish();
    model.queues.forEach((q) => {
      q.items.finish(model.seconds);
    });
    return model;
  }
}

export class Queue {
  name: string;

  constructor(name: string) {
    this.name = name;
  }
}

export class Group extends Queue {
  constructor(name: string) {
    super(name);
  }
}

export class Step {
  seconds: number;
  events: Event[];

  constructor(seconds: number, events: Event[] = []) {
    this.seconds = seconds;
    this.events = events;
  }

  public pushEvent(event: Event) {
    this.events.push(event);
  }

  public apply(model: Model) {
    this.events.forEach((event) => {
      event.apply(model);
    });

    model.seconds = this.seconds;
  }
}

export interface Event {
  apply(model: Model): void;
}

export class PushFront implements Event {
  source: number | undefined;
  destination: number;
  value: number;

  constructor(source: number | undefined, destination: number, value: number) {
    this.source = source;
    this.destination = destination;
    this.value = value;
  }

  public apply(model: Model): void {
    let item;
    if (this.source !== undefined) {
      item = model.queues[this.source].items.pop(this.value, model.seconds);
    } else {
      item = new m.Item(this.value);
      item.entryTime = model.seconds;
    }
    model.queues[this.destination].items.pushFront(item, model.seconds);
  }
}

export class PushBack implements Event {
  source: number | undefined;
  destination: number;
  value: number;

  constructor(source: number | undefined, destination: number, value: number) {
    this.source = source;
    this.destination = destination;
    this.value = value;
  }

  public apply(model: Model): void {
    let item;
    if (this.source !== undefined) {
      item = model.queues[this.source].items.pop(this.value, model.seconds);
    } else {
      item = new m.Item(this.value);
      item.entryTime = model.seconds;
    }
    model.queues[this.destination].items.pushBack(item, model.seconds);
  }
}

export class Pop implements Event {
  queue: number;
  value: number;

  constructor(queue: number, value: number) {
    this.queue = queue;
    this.value = value;
  }

  public apply(model: Model): void {
    const item = model.queues[this.queue].items.pop(this.value, model.seconds);
    const lifetime = item.lifetime(model.seconds);
    model.lifetimes.record(new m.StatEntry(model.seconds, lifetime, item.id));
  }
}
