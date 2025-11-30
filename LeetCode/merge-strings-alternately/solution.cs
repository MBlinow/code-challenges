
public class Solution {
    public string MergeAlternately(string word1, string word2) {
        var maxLength = word1.Length;
        if (word2.Length > maxLength ) maxLength = word2.Length;

        var outString = "";

        for(int i = 0; i < maxLength; i++){
            if (word1.Length > i){
                outString += word1[i];
            }
            if (word2.Length > i){
                outString += word2[i];
            }
        }

        return outString;
    }
}