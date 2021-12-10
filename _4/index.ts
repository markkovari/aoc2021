const [numbers, ...rawTables] = (await Deno.readTextFile("./input.data"))
  .split(
    "\n\n",
  );

const choosenNumbers = numbers.split(",").map(Number);

type Point = {
  isFound: boolean;
  value: number;
};

class Table {
  private rows: Point[][];

  constructor(numbers: Point[][]) {
    this.rows = numbers;
  }
  static fromStringAsNumbers(input: string): Table {
    let data = input
      .split("\n")
      .filter((line: string) => line.length !== 0)
      .map((line: string) => line.trim())
      .map((line: string) =>
        line.split(/\s+/).map((e: string) => ({ value: +e, isFound: false }))
      );
    return new Table(data);
  }

  hasFullRow(): boolean {
    return this.rows.some((row: Point[]) => row.every((p: Point) => p.isFound));
  }

  hasFullColumn(): boolean {
    for (let i: number = 0; i < this.rows[0].length; i++) {
      if (
        this.rows.map((values: Point[]) => values[i]).every((p: Point) =>
          p.isFound
        )
      ) {
        return true;
      }
    }
    return false;
  }

  getNotUsed(): number[] {
    return this.rows
      .map((row: Point[]) => row.filter(({ isFound }) => !isFound)).flat()
      .map((p: Point) => p.value);
  }

  won(): boolean {
    return this.hasFullColumn() || this.hasFullRow();
  }

  found(num: number): void {
    for (const row of this.rows) {
      for (const element of row) {
        if (element.value === num) {
          element.isFound = true;
        }
      }
    }
  }
  toString(): string {
    return this.rows.map((row: Point[]) =>
      row.map(({ value, isFound }) => `${value}:${isFound}`).join("\n")
    ).join("\n");
  }
}

const tables = rawTables.map((line: string) =>
  Table.fromStringAsNumbers(line)
) as Table[];

function getFirstWonTable(
  nums: number[],
  tables: Table[],
): { table: Table; at_: number } | undefined {
  for (const num of nums) {
    for (const table of tables) {
      table.found(num);
      if (table.won()) {
        return { table, at_: num };
      }
    }
  }
}
function onlyTableNotWon(
  nums: number[],
  tables: Table[],
): { table: Table; at_: number } | undefined {
  let numberindex = 0;
  do {
    for (const table of tables) {
      table.found(nums[numberindex]);
    }
    tables = tables.filter((t: Table) => !t.won());
    console.log({
      tables,
      l: tables.length,
      num: nums[numberindex],
      numberindex,
    });
    numberindex++;
  } while (tables.length !== 1);
  console.log(tables[0]);
  return { table: tables[0], at_: nums[numberindex] };
}
const { table, at_ } = getFirstWonTable(choosenNumbers, tables) as {
  table: Table;
  at_: number;
};
const missed = table.getNotUsed();
const missedSum = missed.reduce((a, b) => a + b, 0);
const { table: onlyNotFound, at_: notFoundLastNumber } = onlyTableNotWon(
  choosenNumbers,
  tables,
) as {
  table: Table;
  at_: number;
};
const missedOfLastTable = onlyNotFound.getNotUsed();
const missedOfLastTableSum = missedOfLastTable.reduce((a, b) => a + b, 0);
console.log("yolo", {
  missedOfLastTable: notFoundLastNumber,
  score: (missedOfLastTableSum - notFoundLastNumber) * notFoundLastNumber,
});
