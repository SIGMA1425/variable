//static変数：グローバル変数，mutを付けられる．
static mut X: i32 = 90;

fn main() {
    //letで変数に値を束縛
    let x = 10;
    println!("x:{}", x);
    //変更可能な変数はmutをつける
    let mut y = 100;
    println!("y:{}", y);
    y = 1000;
    println!("y:{}", y);
    //定数はconstを用いる，letと違い常に不変な値が束縛される
    const NUM: i32 = 40; //型を明示する必要がある
    println!("NUM:{}", NUM);
    
    //static変数のアクセスにはunsafeブロックでくくる
    unsafe {
        X += 1;
        println!("X:{}", X);
    }
}
