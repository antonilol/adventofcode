pub mod dir;
pub mod ext;
pub mod sequence;

// TODO get $y and $d from binary name (proc macro? (proc_macro_span?))
#[macro_export]
macro_rules! day {
    ($y:literal $d:literal) => {
        const INPUT: &str = include_str!(concat!("../../input/", $y, "/", $d, ".txt"));

        fn main() {
            let (answer_1, answer_2) = solve(INPUT);
            println!("answers:          {answer_1} {answer_2}\nno expected answers");
        }

        #[cfg(test)]
        mod tests {
            #[test]
            fn test_expected_answers() {
                panic!(concat!("no expected answers for ", $y, " ", $d, "!"));
            }
        }
    };
    ($y:literal $d:literal, $a1:literal, $a2:literal) => {
        const INPUT: &str = include_str!(concat!("../../input/", $y, "/", $d, ".txt"));

        fn main() {
            let (answer_1, answer_2) = solve(INPUT);
            println!(
                concat!("answers:          {} {}\nexpected answers: ", $a1, " ", $a2),
                answer_1, answer_2
            );
        }

        #[cfg(test)]
        mod tests {
            use super::{solve, INPUT};

            #[test]
            fn test_expected_answers() {
                let (answer_1, answer_2) = solve(INPUT);
                assert_eq!(answer_1, $a1);
                assert_eq!(answer_2, $a2);
            }
        }
    };
}
