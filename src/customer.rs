use std::fmt;
use std::cmp::Ordering;

#[derive(Clone, Copy, Eq)]
pub struct Customer {
    // id: String,
    id: usize,
    num_purchases: usize,
}

impl Customer {
    // Another associated function, taking two arguments:
    pub fn new(customer_num: usize, num_purchases: usize) -> Customer {
        // let id = format!("C{customer_num}");
        // Customer { id, num_purchases }
        Customer { id: customer_num, num_purchases }
    }
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}

impl Into<usize> for Customer {
    fn into(self: Self) -> usize {
        self.num_purchases
    }
}

impl PartialOrd for Customer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Customer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num_purchases.cmp(&other.num_purchases)
    }
}

impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.num_purchases == other.num_purchases
    }
}
