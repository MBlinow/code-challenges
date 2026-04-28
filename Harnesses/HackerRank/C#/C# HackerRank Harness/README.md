# C# HackerRank Harness

This is a harness project for solving HackerRank coding puzzles in C#. It provides an easy way to debug and test solutions with different inputs locally before submitting to HackerRank.

## Features

- **Stdin/Stdout Simulation**: Mimics HackerRank's input/output environment.
- **Unit Testing with NUnit**: Run predefined test cases with detailed pass/fail feedback.
- **Timing**: Measures execution time for performance checks.
- **Error Handling**: Catches and logs exceptions.
- **File-Based Testing**: Use input files for testing via redirection.

## Usage

### Building and Running

1. Build the project:
   ```
   dotnet build
   ```

2. Run in stdin mode (for submission or manual input):
   ```
   dotnet run
   ```
   Enter input lines and press Ctrl+D (or Ctrl+Z on Windows) to end.

3. Run unit tests with NUnit:
   ```
   dotnet test
   ```
   This provides detailed pass/fail feedback for test cases.

4. Test with input files:
   ```
   dotnet run < Inputs/input1.txt
   ```
   Or redirect output:
   ```
   dotnet run < Inputs/input1.txt > output.txt
   ```

### Structure

- `Program.cs`: Main entry point, handles I/O.
- `Solution.cs`: Contains the `Solve` method for problem logic. Modify this for each puzzle.
- `SolutionTests.cs`: NUnit test class with test cases for verification.
- `Inputs/`: Folder for sample input files.

### Customizing for a Problem

1. Update the `Solve` method in `Solution.cs` with your algorithm.
2. Add or modify test cases in `SolutionTests.cs` as needed.
3. Run tests: `dotnet test`
4. Create input files in `Inputs/` for file-based testing.

### Debugging

- Use Visual Studio or VS Code debugger.
- For stdin mode, set breakpoints after input reading.
- Check stderr for timing and error messages.

## Notes

- Execution time is logged to stderr (not stdout) to avoid interfering with HackerRank submissions.
- Uses NUnit for unit testing; run `dotnet test` for automated test execution and feedback.
- Keep dependencies minimal; only NUnit packages added for testing.