export class Model {
  public queues: Queue[] = [];
  public seconds: number = 0;
  public step: number = -1;

  public clone(): Model {
    const clone = new Model();
    clone.queues = this.queues.map((q) => q.clone());
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
  items: number[] = [];

  public clone(): Items {
    const clone = new Items();
    clone.items = [...this.items];
    return clone;
  }

  public pushBack(item: number) {
    this.items.push(item);
  }

  public pushFront(item: number) {
    this.items.unshift(item);
  }

  public pop(value: number) {
    this.items = this.items.filter((v) => v !== value);
  }

  public forEach<T>(f: (item: number, index: number) => T) {
    this.items.forEach(f);
  }

  public map<T>(f: (item: number) => T): T[] {
    return this.items.map(f);
  }

  public _values(): number[] {
    return this.items;
  }
}
