use rand::Rng;

fn main() {
    let mut data_set: Vec<i32> = gen_numbers(1000000);
    
    //Bubble Sort Implementation
    
    // let mut swapped: bool = true;
    // while swapped {
    //     let mut prev: i32 = data_set[0];
    //     swapped = false;
    //     for i in 0..data_set.len() {
    //         let n: i32 = data_set[i];
    //         if n < prev {
    //             let tmp: i32 = data_set[usize::try_from(i).unwrap()];
    //             data_set[usize::try_from(i).unwrap()] = data_set[usize::try_from(i).unwrap()-1];
    //             data_set[usize::try_from(i).unwrap()-1] = tmp;
    //             swapped = true;
    //         }
    //         prev = data_set[i];
    //     }
    // }

    //Merge Sort Implementation

    let mut dst: Vec<Vec<i32>> = data_set.chunks(2).map(|s| s.into()).collect();
    for x in &mut dst {
        if x.len() > 1 {
            if x[0] > x[1] {
                let tmp: i32 = x[0];
                x[0] = x[1];
                x[1] = tmp;
            }
        }
    }

    while dst.len() > 1 {
        let mut dst2: Vec<Vec<i32>> = Vec::new();
        let mut i: usize = 0;
        while i < dst.len() {
            if i+2 > dst.len() {
                dst2.push(merge(&dst[i], &Vec::new()));
                break;
            } else {
                dst2.push(merge(&dst[i], &dst[i+1]));
                i+=2;
            }
        }
        dst = dst2;
    }
    data_set = dst[0].clone();

    for i in data_set {
        print!("{} ", i);
    }
}

fn gen_numbers(num: i32) -> Vec<i32>{
    let mut tmp_vec: Vec<i32> = Vec::new();
    for _i in 0..num {
        tmp_vec.push(rand::thread_rng().gen_range(0..10000));
    }
    return  tmp_vec;
}

fn merge(x: &Vec<i32>, y: &Vec<i32>) -> Vec<i32> {
    let mut small_x: usize = 0;
    let mut small_y: usize = 0;

    let mut merged: Vec<i32> = Vec::new();

    for i in 0..(x.len() + y.len()) {
        if small_x < x.len() && small_y < y.len() {
            if x[small_x] < y[small_y] {
                merged.insert(i, x[small_x]);
                small_x +=1;
            } else {
                merged.insert(i, y[small_y]);
                small_y +=1;
            }
        } else if small_x < x.len() {
            merged.insert(i, x[small_x]);
            small_x +=1;
        } else {
            merged.insert(i, y[small_y]);
            small_y += 1;
        }
    }
    return  merged;
}