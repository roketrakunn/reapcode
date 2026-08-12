struct Solution;
impl Solution { 
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let(mut r, mut c) = (0 , 0); 

        for dir in commands.iter() { 
            match dir.as_str() {
                "RIGHT" => c +=1 , 
                "LEFT"  => c -=1, 
                "UP"    => r -=1,
                "DOWN"  => r +=1,
                
                _    => return 0
            }
        }
        r*n +c
    }
}

