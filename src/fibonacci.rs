pub fn up_to(n: u64) {
    let (mut a, mut b) = (0, 1);

    while a <= n {
        print!("{} ", a);
        (a, b) = (b, a + b);
    }
}
pub fn of_length(n: u64) {
    let (mut a, mut b) = (0, 1);

    for _ in 0..n {
        print!("{} ", a);
        (a, b) = (b, a + b);
    }
}
