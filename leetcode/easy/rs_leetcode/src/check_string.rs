struct Solution;

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        
        let mut sr : String = String::new();

        for word in words.iter(){ 
            sr.push_str(word);
            if sr == s {
                return true;
            }
        }
        false
    }
}
