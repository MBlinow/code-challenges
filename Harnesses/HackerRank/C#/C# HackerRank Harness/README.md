# C# HackerRank Harness

This is a harness project for solving HackerRank coding puzzles in C#. It provides an easy way to debug and test solutions with different inputs locally before submitting to HackerRank.

## Features

- **Stdin/Stdout Simulation**: Mimics HackerRank's input/output environment.
- **Embedded Test Cases**: Run predefined test cases with pass/fail verification.
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

3. Run embedded tests:
   ```
   dotnet run -- --test
   ```

4. Test with input files:
   ```
   dotnet run < Inputs/input1.txt
   ```
   Or redirect output:
   ```
   dotnet run < Inputs/input1.txt > output.txt
   ```

### Structure

- `Program.cs`: Main entry point, handles I/O and mode selection.
- `Solution.cs`: Contains the `Solve` method for problem logic. Modify this for each puzzle.
- `TestHarness.cs`: Manages embedded test cases.
- `Inputs/`: Folder for sample input files.

### Customizing for a Problem

1. Update the `Solve` method in `Solution.cs` with your algorithm.
2. Add test cases to `TestHarness.cs` as needed.
3. Create input files in `Inputs/` for file-based testing.

### Debugging

- Use Visual Studio or VS Code debugger.
- For stdin mode, set breakpoints after input reading.
- Check stderr for timing and error messages.

## Notes

- Execution time is logged to stderr (not stdout) to avoid interfering with HackerRank submissions.
- Keep dependencies minimal; no external packages required.