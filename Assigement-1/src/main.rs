fn main() {
    println!("Hello, world!");
    let arr = [0, 1, 2, 4, 4, 5, 1, 2, 3];
    let sub_arr = [1, 2, 3];
    println!("{}", arr.len());
    let distance = arr.len() - sub_arr.len() +1;
    let sub_arr_len = sub_arr.len();
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut bool_val = false;
    while (i<arr.len()){
        if arr[i] == sub_arr[j] {
            i +=1;
            j +=1;
            if j == sub_arr.len() {
                bool_val = true;
                break;
            }
        }
        else
        {
            i = i - j + 1;
            j = 0;
        }
    }
    println!("{}", bool_val);
}
