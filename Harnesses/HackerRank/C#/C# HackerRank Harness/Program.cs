// See https://aka.ms/new-console-template for more information

using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace C__HackerRank_Harness
{
    public class Program
    {
        public static void Main(string[] args)
        {
            try
            {
                if (args.Length > 0 && args[0] == "--test")
                {
                    // Run embedded tests
                    TestHarness.RunTests(Solution.Solve);
                }
                else
                {
                    // Read all input lines from stdin
                    var lines = new List<string>();
                    string? line;
                    while ((line = Console.ReadLine()) is not null)
                    {
                        lines.Add(line);
                    }

                    // Measure execution time
                    var stopwatch = Stopwatch.StartNew();
                    var result = Solution.Solve(lines.ToArray());
                    stopwatch.Stop();

                    // Output result
                    Console.WriteLine(result);

                    // Log timing to stderr (not captured by HackerRank)
                    Console.Error.WriteLine($"Execution Time: {stopwatch.ElapsedMilliseconds} ms");
                }
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"Error: {ex.Message}");
            }
        }
    }
}
