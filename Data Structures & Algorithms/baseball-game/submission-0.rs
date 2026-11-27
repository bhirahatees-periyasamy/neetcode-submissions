impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for opr in operations {
            let len = stack.len();
            match opr.as_str() {
                "+" => {
                    let score = stack[len - 2] + stack[len - 1];
                    stack.push(score);
                }
                "D" => {
                    let score = stack[len - 1];
                    stack.push(score * 2);
                }
                "C" => {
                    stack.pop();
                }

                _ => {
                    let score = opr.parse::<i32>().unwrap();
                    stack.push(score);
                }
            }
        }
        stack.iter().sum()
    }
}
