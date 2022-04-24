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
DirWithLength[] directions = Array.ConvertAll(System.IO.File.ReadAllLines(path), e => new DirWithLength(e));
var position = new Position();

foreach (DirWithLength d in directions)
{
    position.Add(d);
}

Console.WriteLine($"x:{position.X}, y:{position.Y} times -> {position.Distance}");


return 0;

enum Dir
{
    forward,
    down,
    up
}

class DirWithLength
{
    public Dir direction { get; private set; }
    public Int32 Length { get; private set; }

    public DirWithLength(String fromLine)
    {
        string[] parts = fromLine.Split(" ");
        this.Length = Int32.Parse(parts[1]);
        Dir dir;
        Enum.TryParse<Dir>(parts[0], out dir);
        this.direction = dir;
    }
}

class Position
{
    public int X { get; set; } = 0;
    public int Y { get; set; } = 0;

    public int Distance {
      get => this.X * this.Y;
    }

    public void Add(DirWithLength dir)
    {
        var direction = dir.direction;
        if (direction.Equals(Dir.up))
        {
            this.Y -= dir.Length;
        }
        else if (direction.Equals(Dir.down))
        {
            this.Y += dir.Length;
        }
        else if (direction.Equals(Dir.forward))
        {
            this.X += dir.Length;
        }
    }

}
