use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("猜字游戏升级版!");

	//随机生成1-100的数字
    let secret_number = rand::thread_rng().gen_range(1, 101);
 
	loop {
	
		println!("请输入你猜测的数字.");

		//新建可变变量guess
		let mut guess = String::new();
		
		//读取用户输入的内容
		io::stdin().read_line(&mut guess)
			.expect("读取输入失败!");
		
		//新建变量guess,Rust允许覆盖上个名字的变量，这种通常用来改变变量的类型。u32表示无符号的32位整数
		let guess: u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => continue,
		};
		
		println!("你的猜测是: {}", guess);
		
		match guess.cmp(&secret_number){
			Ordering::Less => println!("太小了!"),
			Ordering::Greater => println!("太大了!"),
			Ordering::Equal => {
				println!("猜对了!");
				break;
			}
		}
	}
}