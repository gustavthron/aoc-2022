void HandleChangeDirectory(string[] lineParts, ref List<string> workdir)
{
    if (lineParts[1] != "cd")
    {
        return;
    }
    switch (lineParts[2])
    {
        case "/":
            workdir = new List<string> { lineParts[2] };
            break;
        case "..":
            workdir.RemoveAt(workdir.Count - 1);
            break;
        default:
            workdir.Add(lineParts[2]);
            break;
    }
}

void HandleFileSize(int size, List<string> workdir, ref Dictionary<string, int> directories)
{
    for (int count = 1; count <= workdir.Count; count++)
    {
        var dirPath = workdir.Take(count).Aggregate((acc, dir) => acc + dir);
        directories[dirPath] = directories.GetValueOrDefault(dirPath) + size;
    }
}


Dictionary<string, int> GetDirectories()
{
    var input = File.ReadAllLines("input.txt");
    var directories = new Dictionary<string, int>();
    var workdir = new List<string> { "/" };
    foreach (var line in input)
    {
        var lineParts = line.Split(' ');
        switch (lineParts[0])
        {
            case "$":
                HandleChangeDirectory(lineParts, ref workdir);
                break;
            case "dir":
                break;
            default:
                HandleFileSize(int.Parse(lineParts[0]), workdir, ref directories);
                break;
        }
    }
    return directories;
}

int SolvePart1()
{
    var directories = GetDirectories();
    return directories.Where((pair) => pair.Value <= 100000).Aggregate(0, (acc, pair) => acc + pair.Value);
}

int SolvePart2()
{
    var directories = GetDirectories();
    var sizeToDelete = directories["/"] - 40000000;
    return directories.Where((pair) => pair.Value >= sizeToDelete).MinBy((pair) => pair.Value).Value;
}

Console.WriteLine(Environment.GetEnvironmentVariable("part") == "part1" ? SolvePart1() : SolvePart2());