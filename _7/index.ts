let a = { alma: "fa" };

const { alma } = a;

function stupi(int: number): [number, number] {
  return [int, int * 2];
}

const [originalValue, doubledValue] = stupi(1);
