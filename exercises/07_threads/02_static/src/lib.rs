use std::ops::Range;
// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;
use std::thread::JoinHandle;

pub fn sum(slice: &'static [i32]) -> i32 {
    if slice.is_empty() {
        return 0;
    }

    let block_size = (slice.len().ilog2() as usize).max(1);

    // I got a bit carried away...
    (0..slice.len())
        .step_by(block_size)
        .map(|block_start| block_range(block_start, block_size, slice.len()))
        .map(|block_range| &slice[block_range])
        .map(|block| || block.iter().sum::<i32>())
        .map(thread::spawn)
        .map(JoinHandle::join)
        .map(Result::unwrap)
        .sum::<i32>()
}

fn block_range(
    block_start: usize,
    block_size: usize,
    true_size: usize
) -> Range<usize> {
    let block_end = (block_start + block_size).min(true_size);
    block_start..block_end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    /*
        len:             [1]
        block_size:      [1]
        block_num:       [1]
    */
    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    /*
        len:             [5]
        block_size:      [2]
        block_num:       [3]
    */
    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    /*
        len:             [9]
        block_size:      [3]
        block_num:       [3]
    */
    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    /*
        len:             [10]
        block_size:      [3]
        block_num:       [4]
    */
    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }

    /*
        len:             [100]
        block_size:      [6]
        block_num:       [17]
    */
    #[test]
    fn one_hundred() {
        static ARRAY: [i32; 100] = [
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
        assert_eq!(sum(&ARRAY), ARRAY.iter().sum());
    }

    /*
        len:             [1000]
        block_size:      [9]
        block_num:       [112]
    */
    #[test]
    fn one_thousand() {
        static ARRAY: [i32; 1000] = [
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
        assert_eq!(sum(&ARRAY), ARRAY.iter().sum());
    }
}
