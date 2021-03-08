use std::collections::HashMap;


fn main() {

    let mut int_list = vec![4, 6, 23, 5, 4, 10];
    
    println!("Mean: {}", mean(&int_list));
    println!("Median: {}", median(&mut int_list));
    println!("Mode: {}", mode(&int_list));

}


fn mean(list: &Vec<i32>) -> f32 { 
    list.iter().sum::<i32>() as f32 / list.len() as f32
}

fn median(list: &mut Vec<i32>) -> f32 {
    list.sort();
    let length = list.len();
    let result;

    if length % 2 == 0 {
        result = (list[length / 2] + list[(length / 2) - 1]) as f32 / 2.0;

    } else {
        result = list[length / 2] as f32; 
    }

    result
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut collection = HashMap::new();

    for i in list.iter(){
        let count = collection.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;

    for (i, v) in collection.iter(){
        if *v > mode {
            mode = *v;
        }
    }

    mode
}
