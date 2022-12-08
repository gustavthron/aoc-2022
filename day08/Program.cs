var directions = new (int, int)[] { (0, 1), (0, -1), (-1, 0), (1, 0) };

int SolvePart1()
{
    var trees = new Dictionary<(int, int), int>();
    var input = File.ReadAllLines("input.txt");
    var height = input.Length;
    var width = input.First().Length;
    for (int y = 0; y < height; y++)
    {
        var row = input[y].ToCharArray();
        for (int x = 0; x < width; x++)
        {
            var tree = row[x];
            trees.Add((x, y), tree - '0');
        }
    }
    int visibleCount = 0;
    for (int y = 0; y < height; y++)
    {
        for (int x = 0; x < width; x++)
        {
            var visible = false;
            var tree = trees[(x, y)];
            foreach (var dir in directions)
            {
                var _visible = true;
                var _x = x + dir.Item1;
                var _y = y + dir.Item2;
                while (_visible && -1 < _x && _x < height && -1 < _y && _y < width)
                {
                    _visible = trees[(_x, _y)] < tree;
                    _x += dir.Item1;
                    _y += dir.Item2;
                }
                visible = _visible;
                if (visible)
                {
                    break;
                }   
            }
            visibleCount += visible ? 1 : 0;

        }
    }
    return visibleCount;
}

int SolvePart2()
{
    var trees = new Dictionary<(int, int), int>();
    var input = File.ReadAllLines("input.txt");
    var height = input.Length;
    var width = input.First().Length;
    for (int y = 0; y < height; y++)
    {
        var row = input[y].ToCharArray();
        for (int x = 0; x < width; x++)
        {
            var tree = row[x];
            trees.Add((x, y), tree - '0');
        }
    }
    int highScore = 0;
    for (int y = 0; y < height; y++)
    {
        for (int x = 0; x < width; x++)
        {
            var tree = trees[(x, y)];
            var score = 1;
            foreach (var dir in directions)
            {
                var lastTree = tree;
                var count = 0;
                var _x = x + dir.Item1;
                var _y = y + dir.Item2;
                while (-1 < _x && _x < height && -1 < _y && _y < width)
                {
                    count += 1;
                    var _tree = trees[(_x, _y)];
                    if (_tree >= tree)
                    {
                        break;
                    }
                    lastTree = _tree;
                    _x += dir.Item1;
                    _y += dir.Item2;
                }
                score *= count;
                if (score == 0)
                {
                    break;
                }
            }
            highScore = Math.Max(score, highScore);
        }
    }
    return highScore;
}

Console.WriteLine(Environment.GetEnvironmentVariable("part") == "part1" ? SolvePart1() : SolvePart2());