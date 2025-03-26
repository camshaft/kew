export class Model {
  public queues: Queue[] = [];
  public lifetimes: Stats = new Stats();
  public seconds: number = 0;
  public step: number = -1;

  public clone(): Model {
    const clone = new Model();
    clone.queues = this.queues.map((q) => q.clone());
    clone.lifetimes = this.lifetimes.clone();
    clone.seconds = this.seconds;
    clone.step = this.step;
    return clone;
  }
}

export class Queue {
  public id: number;
  public name: string;
  public isGroup: boolean = false;
  public items: Items = new Items();

  constructor(id: number, name: string, isGroup: boolean) {
    this.id = id;
    this.name = name;
    this.isGroup = isGroup;
  }

  public clone(): Queue {
    const clone = new Queue(this.id, this.name, this.isGroup);
    clone.items = this.items.clone();
    return clone;
  }
}

export class Items {
  items: Item[] = [];
  public sojournTimes: Stats = new Stats();

  public clone(): Items {
    const clone = new Items();
    clone.items = this.items.map((i) => i.clone());
    clone.sojournTimes = this.sojournTimes.clone();
    return clone;
  }

  public pushBack(item: Item, now: number) {
    item.entryTimeInQueue = now;
    this.items.push(item);
  }

  public pushFront(item: Item, now: number) {
    item.entryTimeInQueue = now;
    this.items.unshift(item);
  }

  public pop(id: number, now: number): Item {
    let item = null;
    this.items = this.items.filter((v) => {
      if (v.id !== id) return true;
      item = v;
      return false;
    });

    if (!item) throw new Error(`item ${id} not in queue`);
    item = item as Item;

    const sojournTime = item.sojournTime(now);

    this.sojournTimes.record(new StatEntry(now, sojournTime, item.id));

    return item;
  }

  public finish(now: number) {
    const sojournTimes = this.sojournTimes;

    sojournTimes.finish();

    // record the items that we have as well
    let min = Number.MAX_SAFE_INTEGER;
    let max = 0;
    let length = sojournTimes.entries.length;

    for (let index = 0; index < this.items.length; index++) {
      const item = this.items[index];
      const value = item.sojournTime(now);

      // skip items that just entered
      if (!value) continue;

      length += 1;
      sojournTimes.total += value;
      min = Math.min(min, value);
      max = Math.max(max, value);
    }

    sojournTimes.average = length ? sojournTimes.total / length : 0;
    sojournTimes.min = Math.min(sojournTimes.min, min);
    sojournTimes.max = Math.max(sojournTimes.max, max);
  }

  public forEach<T>(f: (item: Item, index: number) => T) {
    this.items.forEach(f);
  }

  public map<T>(f: (item: Item, index: number) => T): T[] {
    return this.items.map(f);
  }

  public _values(): Item[] {
    return this.items;
  }
}

export class Item {
  public id: number;
  public entryTime: number = 0;
  public entryTimeInQueue: number = 0;

  constructor(id: number) {
    this.id = id;
  }

  public clone(): Item {
    const clone = new Item(this.id);
    clone.entryTime = this.entryTime;
    clone.entryTimeInQueue = this.entryTimeInQueue;
    return clone;
  }

  public sojournTime(now: number): number {
    return now - this.entryTimeInQueue;
  }

  public lifetime(now: number): number {
    return now - this.entryTime;
  }
}

export class Stats {
  public entries: StatEntry[] = [];
  public min: number = 0;
  public max: number = 0;
  public average: number = 0;
  public total: number = 0;

  public clone(): Stats {
    const clone = new Stats();
    // just do a shallow copy - we don't modify records
    clone.entries = [...this.entries];
    return clone;
  }

  public record(entry: StatEntry) {
    this.entries.push(entry);
  }

  public finish() {
    if (!this.entries.length) return;
    let total = 0;
    let min = Number.MAX_SAFE_INTEGER;
    let max = 0;
    for (let index = 0; index < this.entries.length; index++) {
      const entry = this.entries[index];
      total += entry.value;
      min = Math.min(min, entry.value);
      max = Math.max(max, entry.value);
    }
    this.total = total;
    this.average = total / this.entries.length;
    this.min = min;
    this.max = max;
  }
}

export class StatEntry {
  public timestamp: number;
  public value: number;
  public item_id: number;

  constructor(timestamp: number, value: number, item_id: number) {
    this.timestamp = timestamp;
    this.value = value;
    this.item_id = item_id;
  }
}
