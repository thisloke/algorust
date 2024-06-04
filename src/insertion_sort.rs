#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_insert_position() {
        assert_eq!(insertion_sort::find_insert_position(vec![2], 2), Ok(0));
        assert_eq!(insertion_sort::find_insert_position(vec![], 1), Err("vec Vector cannot be empty"));
        assert_eq!(insertion_sort::find_insert_position(vec![3, 1], 3), Ok(0));
        assert_eq!(insertion_sort::find_insert_position(vec![1, 2, 3], 1), Ok(0));
        assert_eq!(insertion_sort::find_insert_position(vec![1, 2, 3], 4), Err("val Value out of bound"));
        assert_eq!(insertion_sort::find_insert_position(vec![5, 2 ,6], 4), Ok(0));
        assert_eq!(insertion_sort::find_insert_position(vec![1, 2, 3, 7, 8], 5), Ok(3));
        assert_eq!(insertion_sort::find_insert_position(vec![1, 2, 3, 7, 8], 9), Err("val Value out of bound"));
        assert_eq!(insertion_sort::find_insert_position(vec![1, 2, 3, 7, 8], 4), Ok(3));
        assert_eq!(insertion_sort::find_insert_position(vec![1, 2, 3, 7, 8], 7), Ok(3));
        assert_eq!(insertion_sort::find_insert_position(vec![1, 2, 3, 7, 8], 8), Ok(4));
        assert_eq!(insertion_sort::find_insert_position(vec![1, 2, 3, 7, 8], 1), Ok(0));
        assert_eq!(insertion_sort::find_insert_position(vec![1, 2, 3, 7, 8], 2), Ok(1));
    }

    #[test]
    fn should_return_empty_vector_if_input_is_an_empty_vector() {
        assert_eq!(insertion_sort::insertion_sort(vec![]), vec![])
    }

    #[test]
    fn should_return_same_vector_if_input_is_a_vector_of_one_item() {
        assert_eq!(insertion_sort::insertion_sort(vec![2]), vec![2])
    }

    #[test]
    fn should_return_ordered_vector() {
        assert_eq!(insertion_sort::insertion_sort(vec![2, 3]), vec![2, 3]);
        assert_eq!(insertion_sort::insertion_sort(vec![3, 2, 1]), vec![1, 2, 3]);
        assert_eq!(insertion_sort::insertion_sort(vec![4, 1, 5, 2, 0]), vec![0, 1, 2, 4, 5]);
        assert_eq!(insertion_sort::insertion_sort(vec![4, -2, 0, 3, 4, 4, 0, -7]), vec![-7, -2, 0, 0, 3, 4, 4, 4]);
        assert_eq!(insertion_sort::insertion_sort(vec![]), vec![]);
        assert_eq!(insertion_sort::insertion_sort(vec![-1]), vec![-1]);
        assert_eq!(insertion_sort::insertion_sort(vec![1,2,3,4,5,7,8,9]), vec![1,2,3,4,5,7,8,9]);
        assert_eq!(insertion_sort::insertion_sort(vec![1,2,3,4,5,7,9,8]), vec![1,2,3,4,5,7,8,9]);
    }
}

pub mod insertion_sort {
    use core::panic;

    pub fn find_insert_position(vec: Vec<i32>, val: i32) -> Result<usize, &'static str> {

        let mut val_exist_in_vec: bool = false;

        if vec.len() == 0 {
            return Err("vec Vector cannot be empty")
        }

        if val <= vec[0] {
            return Ok(0)
        }

        for index in 0..vec.len() {
            if vec[index] == val {
                val_exist_in_vec = true
            }

            if index == vec.len() - 1 && !val_exist_in_vec {
                return Err("val Value out of bound")
            }

            if vec[index] < val && vec[index + 1] >= val {
                return Ok(index + 1)
            }
        }

        Ok(vec.len())
    }

    pub fn insertion_sort(mut vec: Vec<i32>) -> Vec<i32> {

        if vec.len() <= 1 {
            return vec;
        }

        for v in 0..vec.len() {
            let result = find_insert_position(vec.clone(), vec[v]);
            match result {
                Ok(pos) => {
                    println!("{}, position: {}", vec[v], pos);
                    vec.insert(pos, vec[v]);
                    vec.remove(v + 1);
                }
                Err(e) => panic!("{}", e)
            }
        }
        return vec;
    }
}
