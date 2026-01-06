// https://adventofcode.com/2025/day/3

Console.WriteLine("Hello, World!");

var input_lines = parse_input();

Console.WriteLine($"Input lines: {input_lines.Count}");

Console.WriteLine($"Jolts: {get_jolts(input_lines)}");

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
        jolts +=get_jolts_from_line(line);
    }

    return jolts;
}

static int get_jolts_from_line(string line)
{
    char max_Tens = '0';
    int max_tens_index = -1;

    char max_Ones = '0';
    int max_ones_index = -1;

    // Get max tens place digit
    for (int i = 0; i < line.Length; i++)
    {
        char current = line[i];

        if (current > max_Ones)
        {
            max_Ones = current;
            max_ones_index = i;
        }
        
        bool last_char = (i == line.Length - 1);
        if ( !last_char && current > max_Tens)
        {
            max_Tens = current;
            max_tens_index = i;

            // reset ones when setting tens
            max_Ones = '0';
            max_ones_index = -1;
        }
        
        
    }

    string max_val_str = max_Tens.ToString() + max_Ones.ToString();
    return int.Parse(max_val_str);
}