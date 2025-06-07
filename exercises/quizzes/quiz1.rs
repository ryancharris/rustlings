 fn calculate_price_of_apples(num_of_apples: i32) -> i32 {
     if num_of_apples > 40 {
         num_of_apples // Each apple costs 1 rustbuck
     } else {
         num_of_apples * 2 // Each apple costs 2 rustbucks
     }

 }

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
