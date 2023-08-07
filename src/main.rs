mod customer;
mod prng;
pub mod countsort;
pub mod util;

use customer::{
    Customer,
};
use prng::{
    make_random_vec,
    make_random_vec_customer,
};
use util::{
    get_i32,
    check_sorted,
    print_vec,
};
use countsort::{
    counting_sort,
};

fn test_sorting_integers(count: usize, max: usize)
{
    let mut values: Vec<usize> = make_random_vec(count, 0, max);
    print_vec::<usize>(&values, 20);
    counting_sort(&mut values);
    print_vec(&values, 20);
    println!("Sorted: {}", check_sorted(&values));
}

fn test_sorting_customers(count: usize, max: usize)
{
    let mut values: Vec<Customer> = make_random_vec_customer(count, 0, max);
    print_vec::<Customer>(&values, 20);
    counting_sort(&mut values);
    print_vec(&values, 20);
    println!("Sorted: {}", check_sorted(&values));
}

fn main() {
    let max = get_i32("Maximum value for array: ");
    let count = get_i32("Number of random values to sort: ");
    test_sorting_integers(count, max);
    test_sorting_customers(count, max);
}
