
pub fn counting_sort(values: &mut [i32]) {
    if values.len() > 0 {
        let max: usize = (values.iter().max().unwrap() + 1) as usize;
        let mut counts = vec![0i32; max as usize];

        // Get count of occurrences of each value
        for val in values.iter() {
            counts[*val as usize] += 1;
        }

        let mut i: usize = 0;
        for (k, count) in counts.iter().enumerate() {
            let k = k as i32;
            let count: usize = *count as usize;
            for j in 0..count {
                values[j + i] = k;
            }
            i += count;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use crate::util::check_sorted;

    #[test]
    fn reverse_sorted() {
        let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        counting_sort(&mut arr);
        assert!(arr == expected)

    }

    #[test]
    fn same_element() {
        let mut arr = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ];
        let expected = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn zero_elements() {
        let mut arr: [i32; 0] = [];
        let expected: [i32; 0] = [];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn one_element() {
        let mut arr: [i32; 1] = [1];
        let expected: [i32; 1] = [1];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn two_elements_1() {
        let mut arr: [i32; 2] = [1, 2];
        let expected: [i32; 2] = [1, 2];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn two_elements_2() {
        let mut arr: [i32; 2] = [2, 1];
        let expected: [i32; 2] = [1, 2];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }


    #[test]
    fn rand_test() {
        let mut vec: Vec<i32> = (0..100i32).collect::<Vec::<i32>>();
        let mut rng = thread_rng();
        for _ in 0..2000 {
            vec.shuffle(&mut rng);
            counting_sort(&mut vec);
            assert!(check_sorted(&vec))
        }
    }
}
