use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

pub fn rayon_tests() {
    let current = std::time::Instant::now();
    let mut v = [10 as i32; 100];
    let sum: i32 = v.par_iter().sum();
    println!("{:?}", sum);
    println!("Time taken par: {:?}", current.elapsed());
}

pub fn normal_tests() {
    let current = std::time::Instant::now();
    let mut v = [10 as i32; 100];
    let sum: i32 = v.iter().sum();
    println!("{:?}", sum);
    println!("Time taken norm: {:?}", current.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rayon_tests() {
        rayon_tests();
        normal_tests();
    }
}
