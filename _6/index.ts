const content = (await Deno.readTextFile("./input.data")).split(",").map(
  Number,
);

class Fish {
  constructor(private value: number) {}

  tick(): undefined | Fish {
    if (this.value === 0) {
      this.value = 6;
      return new Fish(8);
    }
    this.value--;
  }
  get val() {
    return this.value;
  }

  toString(): string {
    return `fish: ${this.value}`;
  }
}

class Fishes {
  constructor(private fishes: Fish[]) {}

  tick() {
    let newFishes: Fish[] = [];
    for (const fish of this.fishes) {
      const newSmallFish = fish.tick();
      if (newSmallFish !== undefined) {
        newFishes.push(newSmallFish);
      }
    }
    this.fishes.push(...newFishes);
  }
  get values(): number[] {
    return this.fishes.map(({ val }) => val);
  }
  get valuesSum(): number {
    return this.values.reduce((a, b) => a + b, 0);
  }

  get result(): number {
    return this.values.length;
  }
  toString(): string {
    return this.fishes.map((f: Fish) => f.toString()).join(", ");
  }
}
const fishes: Fishes = new Fishes(content.map((v: number) => new Fish(v)));

for (let i = 0; i < 256; i++) {
  fishes.tick();
  console.log(fishes.values.length);
  // console.log(fishes.valuesSum);
}
