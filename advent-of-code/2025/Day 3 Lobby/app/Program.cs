// https://adventofcode.com/2025/day/3

Console.WriteLine("Starting...");

var input_lines = parse_input();

Console.WriteLine($"Input lines: {input_lines.Count}");

var result = get_jolts(input_lines);

Console.WriteLine($"Jolts: {result}");

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

static long get_jolts(List<string> lines)
{
    long jolts = 0;

    foreach (var line in lines)
    {
        jolts += get_jolts_from_line_part2(line);
    }

    return jolts;
}

static int get_jolts_from_line_part1(string line)
{
    char max_Tens = '0';
    char max_Ones = '0';

    // Get max tens place digit
    for (int i = 0; i < line.Length; i++)
    {
        char current = line[i];

        if (current > max_Ones)
        {
            max_Ones = current;
        }
        
        bool last_char = (i == line.Length - 1);
        if ( !last_char && current > max_Tens)
        {
            max_Tens = current;

            // reset ones when setting tens
            max_Ones = '0';
        }
        
        
    }

    string max_val_str = max_Tens.ToString() + max_Ones.ToString();
    return int.Parse(max_val_str);
}

static long get_jolts_from_line_part2(string line, int digit_limit = 12)
{
    int[] max_val_arr = Enumerable.Repeat(0, digit_limit).ToArray();

    int line_digits_remaining = line.Length;

    for (int i = 0; i < line.Length; i++)
    {
        int currentInt = int.Parse(line[i].ToString());

        int max_digits_current = line.Length - i;
        int max_digits_index = digit_limit - max_digits_current;

        int j_loop_start =  (max_digits_index > 0) ? max_digits_index : 0;
        for (int j = j_loop_start; j < max_val_arr.Length; j++)
        {
            if (currentInt > max_val_arr[j])
            {
                //update value
                max_val_arr[j] = currentInt;
                //zero rest of array
                if (j < (max_val_arr.Length -1)){
                    zero_int_array_from_index(max_val_arr, j+1);
                }
                //skip rest of j loop
                break;
            }
        }
    }

    var result = int_array_to_int(max_val_arr);
    return result;
}

static void zero_int_array_from_index(int[] arr, int index)
{
    for (int i = index; i < arr.Length; i++)
    {
        arr[i] = 0;
    }
}

static long int_array_to_int(int[] arr)
{
    return arr
        .Select((t, i) => t * Convert.ToInt64(Math.Pow(10, arr.Length - i - 1)))
        .Sum();   
}