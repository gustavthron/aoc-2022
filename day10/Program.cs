int SolvePart1()
{
    var lines = File.ReadAllLines("input.txt");
    var cycle = 0;
    var x = 1;
    var sum = 0;
    foreach (var line in lines)
    {
        if (cycle > 220)
        {
            break;
        }
        cycle += 1;
        if ((cycle - 20) % 40 == 0)
        {
            sum += cycle * x;
        }
        if (line == "noop")
        {
            continue;
        }
        var lineSplits = line.Split(' ');
        cycle += 1;
        if ((cycle - 20) % 40 == 0)
        {
            sum += cycle * x;
        }
        x += int.Parse(lineSplits[1]);
    }
    return sum;
}

void RenderCycle(int cycle, int x)
{
    var pos = (cycle - 1) % 40;
    if (pos == x - 1 || pos == x || pos == x + 1)
    {
        Console.Write("#");
    }
    else
    {
        Console.Write(".");
    }

    if (cycle % 40 == 0)
    {
        Console.Write("\n");
    }

}

void SolvePart2()
{
    var lines = File.ReadAllLines("input.txt");
    var cycle = 0;
    var x = 1;
    foreach (var line in lines)
    {
        cycle += 1;
        RenderCycle(cycle, x);
        if (line == "noop")
        {
            continue;
        }
        var lineSplits = line.Split(' ');
        cycle += 1;
        RenderCycle(cycle, x);
        x += int.Parse(lineSplits[1]);
    }
}

if (Environment.GetEnvironmentVariable("part") == "part1")
{
    Console.WriteLine(SolvePart1());
}
else
{
    SolvePart2();
}