#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_a_vector_is_sorted() {
        assert_eq!(utils::is_sorted(vec![2, 3]), true);
        assert_eq!(utils::is_sorted(vec![2]), true);
        assert_eq!(utils::is_sorted(vec![5, 3]), false);
        assert_eq!(utils::is_sorted(vec![-2, 3]), true);
        assert_eq!(utils::is_sorted(vec![-2, -3]), false);
        assert_eq!(utils::is_sorted(vec![2, -3]), false);
        assert_eq!(utils::is_sorted(vec![2, 3, 4, 5, 99, 0]), false);
        assert_eq!(utils::is_sorted(vec![2, 3, -4, 5, 99, 100]), false);
        assert_eq!(utils::is_sorted(vec![]), true);
        assert_eq!(utils::is_sorted(vec![4, 1, 5, 2, 0]), false);
        assert_eq!(utils::is_sorted(vec![4, -2, 0, 3, 4, 4, 0, -7]), false)
    }
}


pub mod utils {
    pub fn is_sorted(vec: Vec<i32>) -> bool {
        if vec.len() == 0  {
            return true
        }
        
        for index in 0..vec.len() - 1 {
            if vec[index] > vec[index + 1] {
                return false
            }
        }
        true
    }    
}
