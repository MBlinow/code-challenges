using NUnit.Framework;

namespace C__HackerRank_Harness
{
    [TestFixture]
    public class SolutionTests
    {
        [Test]
        public void Test_SingleLineInput()
        {
            var input = new[] { "Hello" };
            var expected = "Hello";
            var result = Solution.Solve(input);
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Test_AnotherSingleLineInput()
        {
            var input = new[] { "World" };
            var expected = "World";
            var result = Solution.Solve(input);
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Test_MultipleLinesInput()
        {
            var input = new[] { "Test", "Input" };
            var expected = "Test";  // Since placeholder echoes first line
            var result = Solution.Solve(input);
            Assert.AreEqual(expected, result);
        }

        [Test]
        public void Test_EmptyInput()
        {
            var input = new string[] { };
            var expected = "No input";
            var result = Solution.Solve(input);
            Assert.AreEqual(expected, result);
        }
    }
}