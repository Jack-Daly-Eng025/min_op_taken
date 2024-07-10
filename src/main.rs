pub struct Solution;


impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter().fold(0, |path, dir| match dir.as_str(){
            "../" => 0.max(path-1),
            "./" => path,
            _ => path+1
        })
    }
}


fn main() {
    println!("Hello, world!");

    let dir_ops: Vec<String> = vec![ "d1/".to_string(),"d2/".to_string(),"../".to_string(),"d32/".to_string(),"./".to_string()];

    let path_taken = Solution::min_operations(dir_ops);

    println!("The path travelled through your local directory has a depth of {}", path_taken);


}
