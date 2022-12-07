use std::fs;

pub fn soln01() -> i32 {
    let input = fs::read_to_string("inputs/day01/input.txt").expect("Unable to read file");
    let mut sums: Vec<i32> = Vec::new();
    let mut sum = 0;
    for line in input.lines() {
        if line == "" {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    sums.push(sum);

    // get largest sum
    let mut largest = 0;
    for sum in sums {
        if sum > largest {
            largest = sum;
        }
    }
    println!("Answer: {}", largest);
    largest
}

pub fn soln02() -> i32 {
    let input = fs::read_to_string("inputs/day01/input.txt").expect("Unable to read file");
    let mut sums: Vec<i32> = Vec::new();
    let mut sum = 0;
    for line in input.lines() {
        if line == "" {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    sums.push(sum);

    // get the sum of the largest 3 sums
    let mut answer = 0;
    for _ in 0..3 {
        let mut largest: &i32 = &sums[0];
        let mut largest_index: usize = 0;
        for (index, sum) in sums.iter().enumerate() {
            if sum > largest {
                largest = &sum;
                largest_index = index;
            }
        }
        answer += largest;
        sums[largest_index] = 0;
    }
    println!("Answer: {}", answer);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soln01() {
        assert_eq!(soln01(), 74394);
    }

    #[test]
    fn test_soln02() {
        assert_eq!(soln02(), 212836);
    }
}
