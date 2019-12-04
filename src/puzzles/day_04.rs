use crate::input::Input;
use std::cmp::Ordering;
use std::error::Error;

fn get_range(i: &Input) -> Result<(usize, usize), Box<dyn Error>> {
    let parts = i
        .raw
        .trim()
        .split('-')
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()?;
    if parts.len() != 2 {
        return Err("requires start-end".into());
    }

    Ok((parts[0], parts[1]))
}

fn check_pw(pw: usize) -> bool {
    let mut last_char = '\0';
    let mut has_repeated = false;

    for c in pw.to_string().chars() {
        match c.cmp(&last_char) {
            Ordering::Less => return false,
            Ordering::Greater => (),
            Ordering::Equal => has_repeated = true,
        }

        last_char = c;
    }

    has_repeated
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let (start, end) = get_range(i)?;
    let total = (start..=end).filter(|pw| check_pw(*pw)).count();

    Ok(total.to_string())
}

fn check_pw_double_only(pw: usize) -> bool {
    let mut last_char = '\0';

    let mut has_double = false;
    let mut repeat_count = 0;

    for c in pw.to_string().chars() {
        match c.cmp(&last_char) {
            Ordering::Less => return false,
            Ordering::Greater => {
                if repeat_count == 2 {
                    has_double = true;
                }

                repeat_count = 1;
            }
            Ordering::Equal => {
                repeat_count += 1;
            }
        }

        last_char = c;
    }

    has_double || repeat_count == 2
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let (start, end) = get_range(i)?;
    let total = (start..=end).filter(|pw| check_pw_double_only(*pw)).count();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_pw() {
        assert_eq!(check_pw(111111), true);
        assert_eq!(check_pw(223450), false);
        assert_eq!(check_pw(123789), false);
    }

    #[test]
    fn test_check_pw_double_only() {
        assert_eq!(check_pw_double_only(112233), true);
        assert_eq!(check_pw_double_only(123444), false);
        assert_eq!(check_pw_double_only(111122), true);
    }
}
