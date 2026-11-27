impl Solution {
       pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut ans = vec![0; n];
        let mut right_max = -1;
        println!("n : {}",n);
        for i in (0..n).rev() {
            println!("i : {}, right_max: {}, arr[i]: {}, ans: {:#?} ",i,right_max,arr[i],ans);
            ans[i] = right_max;
            right_max = right_max.max(arr[i]);

        }

        ans
    }
}
