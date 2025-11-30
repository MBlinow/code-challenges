public class Solution {
    public void MoveZeroes(int[] nums) {

        var zeroes = 0;

        for (int i = 0; i< nums.Length; i++){
            
            if (nums[i] == 0) zeroes++;

            else nums[i - zeroes] = nums[i];
        }


        //This block could possibly be done within the initial loop through the array, but logic would be a bit trickier.
        while(zeroes > 0){

            nums[nums.Length - zeroes] = 0;
            zeroes--;
        }
    }
}