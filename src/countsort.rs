// Way too many copies/clones of the original values
pub fn counting_sort<T>(values: &mut [T])
    where T: Into<usize> + Clone
{
    let value_count = values.len();
    if value_count > 1 {
        // Find maximum key (a usize) in values
        let max: usize = 
            values.iter()
            .map(|val: &T| Into::<usize>::into((*val).clone()))
            .max().unwrap() + 1;

        // Make count storage for maximum number of keys
        let mut counts: Vec<usize> = vec![0; max];

        // Get count of occurrences of each key in values
        for val in values.into_iter() {
            // Why does getting the key for the value require a clone?
            let key: usize = Into::<usize>::into(val.clone());
            counts[key] += 1;
        }

        // Convert counts-in-bucket to running-count
        for i in 1..max {
            counts[i] += counts[i - 1];
        }

        // Make a vector of indexes where each value belongs:
        // the values in this vector are the positions that the
        // values belong in the final vector.
        let mut indexes = vec![0; value_count];
        for (i, val) in values.iter().enumerate().rev() {
            let key: usize = Into::<usize>::into((*val).clone());
            counts[key] -= 1;
            indexes[counts[key]] = i;
        }

        // Place the values in order in a temporary vector
        let mut tmp: Vec<T> = Vec::with_capacity(value_count);
        for i in indexes.into_iter() {
            tmp.push(values[i].clone());
        }

        // Copy the temporary array into the mutable input array.
        for (i, val) in tmp.into_iter().enumerate() {
            values[i] = val;
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
        let mut arr: [usize; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let expected: [usize; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        counting_sort(&mut arr);
        assert!(arr == expected)

    }

    #[test]
    fn same_element() {
        let mut arr: [usize; 10] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ];
        let expected: [usize; 10] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, ];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn zero_elements() {
        let mut arr: [usize; 0] = [];
        let expected: [usize; 0] = [];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn one_element() {
        let mut arr: [usize; 1] = [1];
        let expected: [usize; 1] = [1];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn two_elements_1() {
        let mut arr: [usize; 2] = [1, 2];
        let expected: [usize; 2] = [1, 2];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }

    #[test]
    fn two_elements_2() {
        let mut arr: [usize; 2] = [2, 1];
        let expected: [usize; 2] = [1, 2];
        counting_sort(&mut arr);
        assert!(arr == expected)
    }


    #[test]
    fn rand_test() {
        let mut vec: Vec<usize> = (0..100).collect::<Vec::<usize>>();
        let mut rng = thread_rng();
        for _ in 0..2000 {
            vec.shuffle(&mut rng);
            counting_sort(&mut vec);
            assert!(check_sorted(&vec))
        }
    }
}
