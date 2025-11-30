public class Solution {
    public int MaxArea(int[] heights) {
        int maxValue = 0;

        int i = 0;
        int j = heights.Length - 1;
                    
        while (i < j){
                int tallest = GetShortest(heights[i], heights[j]);
                //Log($"Tallest: {tallest.ToString()}");
                int currentWater = tallest * (j - i);
                //Log($"CurrentWater: {currentWater.ToString()}.");
                if (currentWater > maxValue) maxValue = currentWater;

                if (heights[i] > heights[j]) j--;
                else i++;
        }
        return maxValue;
    }

    private int GetShortest(int first, int second){
        Log($"Values: first: {first.ToString()} second: {second.ToString()}");
        if (first > second) return second;
        return first;
    }

    private void Log(string val) => Console.WriteLine(val);
}