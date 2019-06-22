
fn main() {
    let n = 4;
    let p = 3;
    let mut arr = [3, 1, 9, 100];
    println!("Arr before sort: {:?}", arr);

    for i in 0..n {
        for j in 0..n - 1 {
            if arr[j] < arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    println!("Arr after sort: {:?}", arr);

    let mut target = 999;
    let mut need;
    let mut prev_need = 999;

    for j in 0..n {
        if j + p <= n {
            target = arr[j];
            need = 0;
            for i in j..n {
                need = need + target - arr[i];
            }
            if need <= prev_need {
                prev_need = need;
            }
        }
    }
    println!("Target: {} Need: {}", target, prev_need);
}


# Case 1 - Working
# Case 2 - Not Checked
# Case 3 - Not Checked