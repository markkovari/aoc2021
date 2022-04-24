if (args.Length == 0)
{
    Console.Error.WriteLine("Add a parameter input or test");
    return 1;
}
List<string> options = new List<string> { "test", "input" };
if (!options.Contains(args[0]))
{
    Console.Error.WriteLine("input is not test or data");
    return 1;
}
var path = $"{args[0]}.data";
Int64[] values = Array.ConvertAll(System.IO.File.ReadAllLines(path), Int64.Parse);

var count = 0;
var countOfThree = 0;
for (var i = 0; i < values.Length - 3; i++)
{
    if (values[i + 3] > values[i])
    {
        Console.WriteLine($"{values[i + 3]} > {values[i]}");
        countOfThree++;
    }
    if (values[i] < values[i + 1])
    {
        count++;
    }
}

for (var i = values.Length - 2; i > values.Length - 4; i--)
{
    if (values[i + 1] > values[i])
    {
        count++;
    }
}

Console.Out.WriteLine($"count: {count} and countOfThree: {countOfThree}");
return 0;
