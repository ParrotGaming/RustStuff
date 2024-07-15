use rand::Rng;

fn main() {
    let mut data_set: Vec<i32> = gen_numbers(100000);
    
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
        dst[0] = merge(dst[0].clone(), dst[1].clone());
        dst.remove(1);
    }
    data_set = dst[0].clone();

    for i in 0..data_set.len() {
        print!("{} ", data_set[i]);
    }
}

fn gen_numbers(num: i32) -> Vec<i32>{
    let mut tmp_vec: Vec<i32> = Vec::new();
    for _i in 1..num {
        tmp_vec.push(rand::thread_rng().gen_range(0..num));
    }
    return  tmp_vec;
}

fn merge(x: Vec<i32>, y: Vec<i32>) -> Vec<i32> {
    let mut small_x: i32 = 0;
    let mut small_y: i32 = 0;

    let mut merged: Vec<i32> = Vec::new();

    for i in 0..(x.len() + y.len()) {
        if small_x < i32::try_from(x.len()).unwrap() && small_y < i32::try_from(y.len()).unwrap() {
            if x[usize::try_from(small_x).unwrap()] < y[usize::try_from(small_y).unwrap()] {
                merged.insert(i, x[usize::try_from(small_x).unwrap()]);
                small_x +=1;
            } else {
                merged.insert(i, y[usize::try_from(small_y).unwrap()]);
                small_y +=1;
            }
        } else if small_x < i32::try_from(x.len()).unwrap() {
            merged.insert(i, x[usize::try_from(small_x).unwrap()]);
            small_x +=1;
        } else {
            merged.insert(i, y[usize::try_from(small_y).unwrap()]);
            small_y += 1;
        }
    }
    return  merged;
}