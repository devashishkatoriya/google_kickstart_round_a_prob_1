
fn main() {
    let n = 5;
    let p = 5;
    let mut arr = [7, 7, 1, 7, 7];
    println!("Arr before sort: {:?}", arr);

    for _ in 0..n {
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
            for i in j..j + p {
                need = need + target - arr[i];
            }
            if need <= prev_need {
                prev_need = need;
            }
        }
    }
    println!("Target: {} Need: {}", target, prev_need);
}


// Case 1 - Working
// Case 2 - Working
// Case 3 - Working