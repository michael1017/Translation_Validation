
fn f_gold(n: usize) -> i32 {
    let mut bell = vec![vec![0; n + 1]; n + 1];
    bell[0][0] = 1;
    for i in 1..=n {
        bell[i][0] = bell[i - 1][i - 1];
        for j in 1..=i {
            bell[i][j] = bell[i - 1][j - 1] + bell[i][j - 1];
        }
    }
    bell[n][0]
}


#[cfg(kani)]
#[kani::proof]
fn main_rs() {
    f_gold(0);
}