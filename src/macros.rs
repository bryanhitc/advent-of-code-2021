#[cfg(test)]
macro_rules! part_test {
    ($name:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let mut test_name_parts = stringify!($name).split('_');

            let part_name = [
                test_name_parts.next().unwrap(),
                test_name_parts.next().unwrap(),
            ]
            .join("_");

            let file_name = test_name_parts.next().unwrap();
            let part_func = match part_name.as_str() {
                "part_one" => part_one,
                "part_two" => part_two,
                _ => panic!("invalid part name"),
            };

            let input = get_input(file_name);
            let actual = part_func(&input);

            assert_eq!($expected, actual);
        }
    };
}

#[macro_export]
macro_rules! part_impl {
    ($($name:ident: $values:expr,)*) => {
        fn get_day_and_raw_input(file_name: &str) -> (&'static str, String) {
            let day = module_path!()
                .split("::")
                .find(|&module_name| module_name.starts_with("day"))
                .unwrap();
            let file_path = format!("inputs/{}/{}.txt", day, file_name);
            let raw_input = std::fs::read_to_string(&file_path).unwrap();

            (day, raw_input)
        }

        pub fn exec() {
            let (day, raw_input) = get_day_and_raw_input("problem");
            let begin = std::time::Instant::now();

            let input = parse_input(&raw_input);
            let input_elapsed = begin.elapsed();

            let p1 = part_one(&input);
            let p1_elapsed = begin.elapsed() - input_elapsed;

            let p2 = part_two(&input);
            let p2_elapsed = begin.elapsed() - p1_elapsed;

            println!("Finished {} in {:?}: P1={} ({:?}) P2={} ({:?})",
                day, begin.elapsed(), p1, p1_elapsed, p2, p2_elapsed);
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            lazy_static! {
                static ref TEST01_INPUT: Input = {
                    let (_, raw_input) = get_day_and_raw_input("test01");
                    parse_input(&raw_input)
                };

                static ref PROBLEM_INPUT: Input = {
                    let (_, raw_input) = get_day_and_raw_input("problem");
                    parse_input(&raw_input)
                };
            }

            fn get_input(file_name: &str) -> &Input {
                match file_name {
                    "test01" => &TEST01_INPUT,
                    "problem" => &PROBLEM_INPUT,
                    _ => panic!("unexpected"),
                }
            }

            $(
                part_test!($name, $values);
            )*
        }
    };
}
