fn main() {
    println!("{}", evolve_n_times(123, 10));
}

fn evolve(n: usize) -> usize {
    let n1 = ((n * 64) ^ n) % 16777216;
    let n2 = ((n1 / 32) ^ n1) % 16777216;
    let n3 = ((n2 * 2048) ^ n2) % 16777216;
    n3
}

fn evolve_bitwise(n: usize) -> usize {
    let n1 = ((n << 6) ^ n) & 16777215;
    let n2 = ((n1 >> 5) ^ n1) & 16777215;
    let n3 = ((n2 << 11) ^ n2) & 16777215;
    n3
}

fn evolve_n_times(mut secret: usize, n: usize) -> usize {
    for _ in 0..n {
        secret = evolve_bitwise(secret);
    }
    secret
}
