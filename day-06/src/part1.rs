const INPUT: &[u8] = include_bytes!("./input.txt");

pub fn main() {
    let result = solve(INPUT);
    println!("{result}");
}

fn solve(input: &[u8]) -> u64 {
    let row_length = input.iter().position(|c| *c == b'\n').unwrap();
    let number_of_rows = input.len() / row_length;

    let operator_index = (number_of_rows - 1) * (row_length + 1);
    let ops = &input[operator_index..operator_index + row_length];

    let mut grand_total = 0;

    let mut numbers = [0; 4];
    let mut op = b' ';

    for x in 0..row_length {
        let mut found_digit = false;
        for y in 0..number_of_rows - 1 {
            let c = input[(row_length + 1) * y + x];

            if c.is_ascii_digit() {
                found_digit = true;
                numbers[y] = numbers[y] * 10 + u64::from(c - b'0');
            }
        }

        if !found_digit {
            match op {
                b'+' => {
                    let mut total = 0;
                    for number in numbers.iter_mut().take(number_of_rows - 1) {
                        total += *number;
                        *number = 0;
                    }
                    grand_total += total;
                }
                b'*' => {
                    let mut total = 1;
                    for number in numbers.iter_mut().take(number_of_rows - 1) {
                        total *= *number;
                        *number = 0;
                    }
                    grand_total += total;
                }
                _ => unreachable!(),
            }
        }

        if ops[x] != b' ' {
            op = ops[x];
        }
    }

    match op {
        b'+' => {
            let mut total = 0;
            for number in numbers.iter_mut().take(number_of_rows - 1) {
                total += *number;
                *number = 0;
            }
            grand_total += total;
        }
        b'*' => {
            let mut total = 1;
            for number in numbers.iter_mut().take(number_of_rows - 1) {
                total *= *number;
                *number = 0;
            }
            grand_total += total;
        }
        _ => unreachable!(),
    }

    grand_total
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(b"123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ");
        assert_eq!(result, 4_277_556);
    }
}

#[cfg(feature = "bench")]
pub mod benchmarks {
    use super::INPUT;

    pub fn main() {}

    #[divan::bench()]
    fn bench_solve() {
        super::solve(INPUT);
    }
}
