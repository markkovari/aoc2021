const content = (await Deno.readTextFile("./input.data")).split("\n").filter((e: string) => e !== "") as string[];
const { length: lineCount } = content;

let mostOnes = [...content];
let builder = "";
do {
  const { length: numberOfFirstOnesatCharOne } = mostOnes.map((e: string) => e[0]).filter((e: string) => e === "1");
  const mostCommonFirstChar = mostOnes.length / 2 <= numberOfFirstOnesatCharOne ? "1" : "0";
  builder += mostCommonFirstChar;
  const withPrefix = mostOnes.filter((e: string) => e.startsWith(mostCommonFirstChar))
  mostOnes = withPrefix.map(([_, ...xs]) => xs.join(""))
} while (mostOnes.length != 1);

console.log({ builder, mostOnes, result: parseInt(builder.concat(mostOnes[0].length === 0 ? "" : mostOnes[0]), 2) })
let O2 = parseInt(builder.concat(mostOnes[0].length === 0 ? "" : mostOnes[0]), 2);


let mostZeros = [...content];
let secondBuilder = "";
do {
  const { length: numberOfFirstOnesatCharOne } = mostZeros.map((e: string) => e[0]).filter((e: string) => e === "1");
  const mostCommonFirstChar = mostZeros.length / 2 > numberOfFirstOnesatCharOne ? "1" : "0";
  secondBuilder += mostCommonFirstChar;
  const withPrefix = mostZeros.filter((e: string) => e.startsWith(mostCommonFirstChar))
  mostZeros = withPrefix.map(([_, ...xs]) => xs.join(""))
} while (mostZeros.length != 1);

let CO2 = parseInt(secondBuilder.concat(mostZeros[0].length === 0 ? "" : mostZeros[0]), 2);

console.log({ secondBuilder, mostZeros, result: parseInt(secondBuilder.concat(mostZeros[0].length === 0 ? "" : mostZeros[0]), 2) })

console.log({ result: O2 * CO2 })