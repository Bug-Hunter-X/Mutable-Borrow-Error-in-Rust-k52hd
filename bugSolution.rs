fn main() {
    let mut x = 5;
    { // create a new scope
        let y = &mut x;
        *y = 10;
    }
    let z = &mut x;
    *z = 100;
} 