// https://adventofcode.com/2025/day/3

Console.WriteLine("Hello, World!");

var input_lines = parse_input();

Console.WriteLine($"Input lines: {input_lines.Count}");

static List<string> parse_input()
{
    StreamReader reader = new StreamReader("E:\\DEV\\code-challenges\\advent-of-code\\2025\\Day 3 Lobby\\app\\input.txt");

    List<string> lines = new List<string>();

    string? line;
    while ((line = reader.ReadLine()) != null)
    {
        lines.Add(line);
    }

    return lines;
}

static int get_jolts(List<string> lines)
{
    int jolts = 0;

    foreach (var line in lines)
    {
        // Process each line to calculate jolts
    }

    return jolts;
}