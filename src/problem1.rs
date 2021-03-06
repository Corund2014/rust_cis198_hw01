    /// Computes the sum of all elements in the input i32 slice named `slice`
    pub fn sum(slice: &[i32]) -> i32 {
        // TODO
        let mut sum_result: i32 = 0;
        for x in slice {
            sum_result += *x;
        }
        sum_result
    }

    /// Deduplicates items in the input vector `vs`. Produces a vector containing
    /// the first instance of each distinct element of `vs`, preserving the
    /// original order.
    pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
        let mut unique_vec = Vec::new();

        for x in vs {
            let mut duplicate: bool = false;
            for y in &unique_vec {
                if x == y {
                    duplicate = true;
                    break;
                }
            }
            if !duplicate {
                unique_vec.push(*x)
            }
        }
        unique_vec
    }

    /// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
    /// `bool`). Returns a new vector containing only elements that satisfy `pred`.
    pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for i in vs {

            if pred(*i) {
                result.push(*i);
            }

        }

        result
    }
