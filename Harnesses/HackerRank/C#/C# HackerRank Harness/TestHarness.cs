using System;
using System.Collections.Generic;

namespace C__HackerRank_Harness
{
    public static class TestHarness
    {
        public static void RunTests(Func<string[], string> solver)
        {
            var testCases = new List<(string[] Input, string Expected)>
            {
                (new[] { "Hello" }, "Hello"),
                (new[] { "World" }, "World"),
                (new[] { "Test", "Input" }, "Test")  // Example with multiple lines
            };

            foreach (var (input, expected) in testCases)
            {
                var result = solver(input);
                var status = result == expected ? "PASS" : "FAIL";
                Console.WriteLine($"Input: [{string.Join(", ", input)}] | Expected: {expected} | Result: {result} | Status: {status}");
            }
        }
    }
}