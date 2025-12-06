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
    let mut op = b' ';
    let mut total = 0;

    for x in 0..row_length {
        if ops[x] != b' ' {
            op = ops[x];
        }
        let mut found_digit = false;
        let mut number = 0;
        for y in 0..number_of_rows - 1 {
            let c = input[(row_length + 1) * y + x];

            if c.is_ascii_digit() {
                found_digit = true;
                number = number * 10 + u64::from(c - b'0');
            }
        }

        if found_digit {
            match op {
                b'+' => {
                    total += number;
                }
                b'*' => {
                    if total == 0 {
                        total = number;
                    } else {
                        total *= number;
                    }
                }
                _ => unreachable!(),
            }
        }

        if !found_digit {
            grand_total += total;
            total = 0;
        }
    }
    grand_total += total;
    grand_total
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_1() {
        let result = solve(b"123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ");
        assert_eq!(result, 3_263_827);
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
