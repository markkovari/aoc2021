const content = (await Deno.readTextFile("./input.test")).split("\n");

type Value = "." | number;

class Point {
  horizontal = 0;
  vertical = 0;
  value: Value = ".";

  constructor() {}

  static fromString(from: string) {
    const [h, v] = from.split(",");
    return this.fromValues(+h, +v, ".");
  }
  static fromValues(horizontal: number, vertical: number, val: Value): Point {
    const point: Point = new Point();
    point.vertical = vertical;
    point.horizontal = horizontal;
    point.value = val;
    return point;
  }
}

class Line {
  start: Point;
  end: Point;

  constructor(from: string) {
    const [start, end] = from.split(" -> ");
    this.start = Point.fromString(start);
    this.end = Point.fromString(end);
  }

  get isHorizontal() {
    return this.start.horizontal === this.end.horizontal;
  }
  get isVertical() {
    return this.start.vertical === this.end.vertical;
  }

  normalise() {
    if (this.isHorizontal) {
      if (this.end.vertical < this.start.vertical) {
        const [start, end] = [this.end, this.start];
        this.start = start;
        this.end = end;
      }
    }
    if (this.isVertical) {
      if (this.end.horizontal < this.start.horizontal) {
        const [start, end] = [this.end, this.start];
        this.start = start;
        this.end = end;
      }
    }
    return this;
  }
}

class PointMap {
  points: Point[][] = [];
  lines: Line[];
  constructor(lines: Line[]) {
    this.lines = lines;
    const maxHorizontal: number = Math.max(
      ...lines.map(({ end }) => end.horizontal),
    );
    const maxVertical: number = Math.max(
      ...lines.map(({ end }) => end.vertical),
    );
    this.points = [...Array(maxHorizontal).keys()]
      .map((i: number) =>
        [...Array(maxVertical).keys()].map((j: number) =>
          Point.fromValues(i, j, ".")
        )
      );
    for (let i = 0; i < maxHorizontal; i++) {
      for (let j = 0; j < maxVertical; j++) {
        this.points[i][j] = Point.fromValues(i, j, ".");
      }
    }
  }

  get verticalLines() {
    return this.lines.filter((line: Line) => line.isHorizontal).map((
      line: Line,
    ) => line.normalise());
  }

  get horizontalLines() {
    return this.lines.filter((line: Line) => line.isVertical).map((
      line: Line,
    ) => line.normalise());
  }

  get straightLines() {
    return [...this.horizontalLines, ...this.verticalLines];
  }

  addLineValues() {
    console.log("beforeVLinesapply", this.points);
    for (const { start, end } of this.verticalLines) {
      for (let i = start.vertical; i <= end.vertical; i++) {
        const point = this.points[i][start.horizontal];
        console.log("vLine:", this.points[i]);
        point.value = point.value === "." ? 1 : point.value + 1;
      }
    }
    console.log("afterVLinesapply", this.points);
    for (const { start, end } of this.horizontalLines) {
      for (let i = start.horizontal; i <= end.horizontal; i++) {
        console.log({ x: start.horizontal, i, a: start.vertical });
        console.log("line", this.points[start.vertical]);
        const point = this.points[start.vertical][i];
        console.log({ point, x: start.horizontal, y: i });
        point.value = point.value === "." ? 1 : point.value + 1;
      }
    }
    console.log("afterHLinesapply", this.points);
  }

  toString(): string {
    return this.points.map((line: Point[]) =>
      line.map(({ value }) => value).join(" ")
    ).join(" \n");
  }
}

const lines = content.map((line: string) => new Line(line));

const pointMap = new PointMap(lines);

// console.log({ content, lines, pointMap: pointMap.toString() });
console.log("vertical lines", pointMap.verticalLines);
// console.log(pointMap.toString());
pointMap.addLineValues();
console.log(pointMap.toString());
