use num_traits::Unsigned;

pub fn lcm<T: Unsigned + Copy + Ord>(a: T, b: T) -> T {
    a * b / gcd(a, b)
}

pub fn gcd<T: Unsigned + Copy + Ord>(a: T, b: T) -> T {
    let (mut min, mut max) = if a < b { (a, b) } else { (b, a) };

    loop {
        let res = max % min;
        if res.is_zero() {
            return min;
        }

        max = min;
        min = res;
    }
}
