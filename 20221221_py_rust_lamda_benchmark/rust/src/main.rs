use rand::Rng;
use std::env;

const SIZE: u64 = 10_000_000;
// const iters: i32 = 5;

fn repeat_test(iter_count: i32) {
    for test_loop in 0..iter_count {
        let num = rand::thread_rng().gen_range(SIZE..SIZE+5);
        let box1: Vec<u64> = (0..num as u64).collect();
        let box2: Vec<u64> = box1.iter().map( |x| (x + (x+1) + (x+2)) / 3 ).collect();
        let sum_all = box2.iter().fold(0u64, |sum, i| sum + (*i as u64));
        println!("Loop: [{}], len: [{}], 10 elements: {:?}, results sum: [{}]",
                test_loop, box2.len(), &box2[0..10], sum_all);
    }
}

fn main() {
    // let iters = env::var("ITER_COUNT").is_ok();
    let iters = match env::var_os("ITER_COUNT") {
        Some(val) => val.into_string().unwrap().parse::<i32>().unwrap(),//val.into_string().unwrap(),
        None => 5
    };
    repeat_test(iters);
}

// fn main() {
//     let size = 10_000_000; 
//     let num = rand::thread_rng().gen_range(size..size+2);
    
//     let box1: Vec<u128> = (0..num).collect();
//     let box2: Vec<u128> = box1.iter().map( |x| (x + (x+1) + (x+2)) / 3 ).collect();
//     let sum_all = box2.iter().fold(0u64, |sum, i| sum + (*i as u64));
//     // let box1: [usize; num] = core::array::from_fn(|i| i + 1);g
//     // let array: [usize; 5] = 
    
//     // let first_elems: Vec<usize> = box2.iter().skip(5);
    
//     println!("First 10 elements : {:?}", &box2[0..10]);
//     println!("Len of results array : [{}]", box2.len() );
//     println!("Sum of all results : [{}]", sum_all);

//     println!("Hello, digit {}", num);
    

// }


