int SolvePuzzle(int distinctCount)
{
    var input = File.ReadAllText("input.txt").ToCharArray();
    var received = new List<char>();
    for (int i = 0; i < input.Length; i++)
    {
        received.Add(input[i]);
        if (received.TakeLast(distinctCount).Distinct().Count() == distinctCount)
        {
            return i + 1;
        }
    }
    return 0;
}

Console.WriteLine(SolvePuzzle(Environment.GetEnvironmentVariable("part") == "part1" ? 4 : 14));