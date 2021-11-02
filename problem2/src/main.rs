fn sum(arr: &[u32]) -> Option<u32> {
    let mut flag: Option<u32> = Some(0);
    for i in arr {
        flag = flag.unwrap().checked_add(*i);
        if flag.is_none() {
            return flag;
        }
    }

    flag
}

fn main() {
    let data: [u32;8] = [11,22,33,44,55,66,77,88];
    let flag = sum(&data);
    println!("The result is {:?}", flag.unwrap());
}
