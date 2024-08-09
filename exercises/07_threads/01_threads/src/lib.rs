// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    if v.is_empty() {
        return 0;
    }

    let block_size: usize = usize::try_from(v.len().ilog2()).unwrap().max(1);

    let extra_values = v.len() % block_size;

    let (mut sum_pool, trimmed_size) = if extra_values == 0 {
        (Vec::with_capacity(v.len() / block_size), v.len())
    } else {
        let mut sum_pool = Vec::with_capacity((v.len() / block_size) + 1);

        let trimmed_size = v.len() - extra_values;
        let block_slice = Vec::from(&v[trimmed_size..v.len()]);
        sum_pool.push(thread::spawn(move || -> i32 { block_slice.iter().sum() }));

        (sum_pool, trimmed_size)
    };

    println!();
    println!("len:             [{}]", v.len());
    println!("block_size:      [{}]", block_size);
    println!("block_num:       [{}]", sum_pool.capacity());

    for i in (0..trimmed_size).step_by(block_size) {
        let block_slice_start = i;
        let block_slice_end = i + block_size;
        let block_slice = Vec::from(&v[block_slice_start..block_slice_end]);
        sum_pool.push(thread::spawn(move || -> i32 { block_slice.iter().sum() }));
    }

    sum_pool.into_iter()
        .map(|jh| jh.join())
        .map(|r| r.unwrap())
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    /*
        len:             [1]
        block_size:      [1]
        block_num:       [1]
    */
    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    /*
        len:             [5]
        block_size:      [2]
        block_num:       [3]
    */
    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    /*
        len:             [9]
        block_size:      [3]
        block_num:       [3]
    */
    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    /*
        len:             [10]
        block_size:      [3]
        block_num:       [4]
    */
    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }

    /*
        len:             [100]
        block_size:      [6]
        block_num:       [17]
    */
    #[test]
    fn one_hundred() {
        let vec = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 2,
            3, 4, 5, 6, 7, 8, 9, 10, 5, 2,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 2,
            3, 4, 5, 6, 7, 8, 9, 10, 5, 2,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
        ];
        assert_eq!(sum(vec.clone()), vec.iter().sum());
    }

    /*
        len:             [1000]
        block_size:      [9]
        block_num:       [112]
    */
    #[test]
    fn one_thousand() {
        let vec = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2,
            3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2,
            3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2,
            3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2,
            3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2,
            3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2,
            3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1,
            2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2,
            3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2, 3, 4, 5, 6, 7, 8, 9, 10, 5, 2,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];
        assert_eq!(sum(vec.clone()), vec.iter().sum());
    }
}
