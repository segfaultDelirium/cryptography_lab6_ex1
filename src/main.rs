fn add_modulo2(x: u8, y: u8) -> u8 {
    let res = x + y;
    if res == 2 {0} else {res}
}


fn modulo_n_vectors(x: &Vec<u8>, y: &Vec<u8>) -> Vec<u8>{
    x.iter().zip(y.into_iter()).map(|(x_el, y_el)| add_modulo2(*x_el, *y_el) ).collect()
}


fn convert_from_flatten(vec: &Vec<u8>) -> Vec<Vec<u8>>{

    let to_push1 = vec.clone().into_iter().take(4).collect();
    let to_push2 = vec.clone().into_iter().skip(4).take(4).collect();
    let to_push3 = vec.clone().into_iter().skip(8).take(4).collect();
    let to_push4 = vec.clone().into_iter().skip(12).take(4).collect();
    vec![to_push1, to_push2, to_push3, to_push4]
}


fn main() {
    

    let x1 = vec![0,0,0,0, 0,0,0,0, 1,1,1,1, 1,1,1,1];
    let x2 = vec![0,0,0,0, 1,1,1,1, 0,0,0,0, 1,1,1,1];
    let x3 = vec![0,0,1,1, 0,0,1,1, 0,0,1,1, 0,0,1,1,];
    let x4 = vec![0,1,0,1, 0,1,0,1, 0,1,0,1, 0,1,0,1,];

    let y1 = vec![1,0,1,0, 0,1,1,1, 0,1,0,1, 0,1,0,0];
    let y2 = vec![1,1,1,0, 0,1,0,0, 0,0,1,1, 1,0,0,1];
    let y3 = vec![1,0,0,0, 1,1,1,0, 1,1,1,0, 0,0,0,1];
    let y4 = vec![0,0,1,1, 0,1,1,0, 1,0,0,0, 1,1,0,1];


    let sum1  = modulo_n_vectors(&modulo_n_vectors(&x1, &x4), &y2);

    println!("X1 xor X4 xor Y2 = {:?}", sum1);
    println!();
    let count1 = sum1.into_iter().reduce(|acc, x| {
        if x == 0 {
            acc + 1
        }else {acc}
    }).unwrap();
    let pr_1 = count1 as f32 / 16.0;
    println!("Pr[X1 xor X4 xor Y2 = 0] = {}", pr_1);
    println!("Pr[X1 xor X4 xor Y2 = 1] = {}", 1.0 - pr_1);
    println!("eps[X1 xor X4 xor Y2 = 0] = {}", (count1 - 8) as f32 / 16.0);
    println!("eps[X1 xor X4 xor Y2 = 1] = {}", (count1 - 8) as f32 / 16.0);

    println!();
    let sum2  = modulo_n_vectors(&modulo_n_vectors(&modulo_n_vectors(&x3, &x4), &y1), &y4);

    println!("X3 xor X4 xor Y1 xor Y4 = {:?}", sum2);
    println!();
    let count2 = sum2.into_iter().reduce(|acc, x| {
        if x == 0 {
            acc + 1
        }else {acc}
    }).unwrap();
    let count2_ones = 16 - count2;
    let pr_2 = count2 as f32 / 16.0;
    println!("Pr[X1 xor X4 xor Y2 = 0] = {}", pr_2);
    println!("Pr[X1 xor X4 xor Y2 = 1] = {}", 1.0 - pr_2);
    println!("eps[X1 xor X4 xor Y2 = 0] = {}", (count2 as i32 - 8) as f32 / 16.0);
    println!("eps[X1 xor X4 xor Y2 = 1] = {}", (count2_ones as i32 - 8) as f32 / 16.0);



}
