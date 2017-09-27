fn main() {
    let mut a = true;
    loop {
        while a {
            if true {
                a = false;
                continue;
            }
            return;
        }
        println!("Looping");
    }
}
