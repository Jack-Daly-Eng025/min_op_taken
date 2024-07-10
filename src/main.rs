pub struct Solution;


impl Solution {
    pub fn min_operations(logs: Vec<&str>) -> i32 {
        logs.iter().fold(0, |path, dir| match dir{
            &"../" => 0.max(path-1),
            &"./" => path,
            _ => path+1
        })
    }
}


fn main() {
    println!("Hello, world!");

    let dir_ops: Vec<&str> = vec![ "d1/","d2/","../","d32/","./"];

    let path_taken = Solution::min_operations(dir_ops);

    println!("The path travelled through your local directory has a depth of {}", path_taken);
}
