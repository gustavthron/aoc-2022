int SolvePuzzle(int knots)
{
    var lines = File.ReadAllLines("input.txt");
    var visited = new HashSet<(int x, int y)>();
    var rope = new List<(int x, int y)>();
    for (int i = 0; i < knots; i++)
    {
        rope.Add((x: 0, y: 0));
    }
    foreach (var line in lines)
    {
        var lineSplits = line.Split(' ');
        var dir = lineSplits[0];
        var steps = int.Parse(lineSplits[1]);
        var move = dir switch
        {
            "U" => (x: 0, y: 1),
            "D" => (x: 0, y: -1),
            "L" => (x: -1, y: 0),
            "R" => (x: 1, y: 0),
            _ => throw new ArgumentException("Direction not found")
        };
        for (var i = 0; i < steps; i++)
        {
            rope[0] = (x: rope[0].x + move.x, y: rope[0].y + move.y);
            for (var j = 1; j < knots; j++)
            {
                var delta = (x: rope[j-1].x - rope[j].x, y: rope[j-1].y - rope[j].y);
                if (Math.Abs(delta.x) + Math.Abs(delta.y) == 3)
                {
                    rope[j] = (x: rope[j].x + (delta.x + Math.Sign(delta.x)) / 2, y: rope[j].y + (delta.y + Math.Sign(delta.y)) / 2);
                }
                else if (Math.Abs(delta.x) + Math.Abs(delta.y) > 1)
                {
                    rope[j] = (x: rope[j].x + (delta.x) / 2, y: rope[j].y + (delta.y) / 2);
                }
            }
            visited.Add(rope[^1]);
        }
    }
    return visited.Count;
}

Console.WriteLine(SolvePuzzle(Environment.GetEnvironmentVariable("part") == "part1" ? 2 : 10));