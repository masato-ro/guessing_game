use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("請猜測第一個數字！");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("祕密數字為：{secret_number}");
    
    loop {
        println!("請輸入你的猜測數字。");
    
        let mut guess = String::new(); // 可變的
        
        io::stdin()
            .read_line(&mut guess)
            .expect("讀取該行失敗！");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你的猜測數字︰{guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                print!("獲勝！");
                break;
            }
        }   
    }    
}
