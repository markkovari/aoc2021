const content = (await Deno.readTextFile("./input.data")).split("\n").filter((
  line: string,
) => line.length !== 0);

console.log({ content });
// for()

function lineSplitter(from: string): [number[], number[]] {
  const [start, end] = from.split(" -> ");
  const [startX, startY] = start.split(",").map(Number);
  const [endX, endY] = end.split(",").map(Number);
  return [[startX, startY], [endX, endY]];
}

function horizontalOrVertical(of: [number[], number[]]): boolean {
  return of[0][0] === of[1][0] || of[0][1] === of[1][1];
}

const neededLines = content.map(lineSplitter).filter(horizontalOrVertical);
console.log(neededLines);

const maxX = Math.max(...neededLines.map((e) => Math.max(e[0][0], e[1][0])));
const maxY = Math.max(...neededLines.map((e) => Math.max(e[0][1], e[1][1])));
console.log({ maxX, maxY });

const pointMap: number[][] = Array.from(
  Array(maxX + 1),
  (_) => Array(maxY + 1).fill(0),
);

for (const [start, end] of neededLines) {
  if (start[0] === end[0]) {
    const [startPoint, endPoint] = start[1] < end[1]
      ? [start[1], end[1]]
      : [end[1], start[1]];
    console.log("horizontal:", { startPoint, endPoint, x: start[0] });
    for (let i = startPoint; i <= endPoint; i++) {
      pointMap[i][start[0]] += 1;
    }
  }

  if (start[1] === end[1]) {
    const [startPoint, endPoint] = start[0] < end[0]
      ? [start[0], end[0]]
      : [end[0], start[0]];
    console.log("horizontal:", { startPoint, endPoint, y: start[0] });

    for (let i = startPoint; i <= endPoint; i++) {
      pointMap[start[1]][i] += 1;
    }
  }
}
console.table(pointMap);
const countOfAboveOne = pointMap.map((v) => v.filter((e) => e > 1).length)
  .reduce((a, b) => a + b, 0);

console.log(countOfAboveOne);
