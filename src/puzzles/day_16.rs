use crate::input::Input;
use std::error::Error;
use std::fmt::Write;
use std::iter;

type NumberList = Vec<isize>;

fn signal_from_input(i: &Input) -> Result<NumberList, Box<dyn Error>> {
    i.raw
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|v| v as isize)
                .ok_or_else(|| "couldn't parse digit".into())
        })
        .collect::<Result<_, _>>()
}

fn signal_to_string(signal: &[isize]) -> String {
    let mut s = String::with_capacity(signal.len());
    signal
        .iter()
        .for_each(|v| s.write_str(&v.to_string()).unwrap());

    s
}

const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

fn get_pattern(n: usize) -> impl Iterator<Item = isize> {
    BASE_PATTERN
        .iter()
        .flat_map(move |&v| iter::repeat(v).take(n))
        .cycle()
        .skip(1)
}

fn calc_element(signal: impl Iterator<Item = isize>, i: usize) -> isize {
    let sum: isize = signal.zip(get_pattern(i + 1)).map(|(v, p)| v * p).sum();
    (sum % 10).abs()
}

fn run_phase(signal: &mut NumberList) {
    for i in 0..signal.len() {
        signal[i] = calc_element(signal.iter().copied(), i);
    }
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut signal = signal_from_input(i)?;
    for _ in 0..100 {
        run_phase(&mut signal);
    }

    Ok(signal_to_string(&signal[..8]))
}

fn get_offset(signal: impl Iterator<Item = isize>) -> usize {
    signal.take(7).fold(0, |n, d| 10 * n + d) as usize
}

fn run_phase_offset(signal: &mut [isize]) {
    let mut partial_sum: isize = signal.iter().sum();
    for v in signal.iter_mut() {
        let d = (partial_sum % 10).abs();
        partial_sum -= *v;
        *v = d;
    }
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    const REP: usize = 10_000;

    let once_signal = signal_from_input(i)?;
    let offset = get_offset(once_signal.iter().copied());

    let signal_len = REP * once_signal.len();
    if 2 * offset <= signal_len {
        // We're relying on the fact that when i >= len(signal) / 2, get_pattern(i)
        // yields 0 for i < len(signal) and 1 for every i after that.
        // This means that for i >= len(signal) the value is just sum(signal[i:])!
        return Err("expected bigger offset!".into());
    }
    let offset_start = offset % once_signal.len();
    let mut signal: NumberList = once_signal
        .iter()
        .copied()
        .cycle()
        .skip(offset_start)
        .take(signal_len - offset)
        .collect();

    for _ in 0..100 {
        run_phase_offset(&mut signal);
    }

    Ok(signal_to_string(&signal[..8]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pattern() {
        assert_eq!(
            get_pattern(1).take(8).collect::<Vec<_>>(),
            vec![1, 0, -1, 0, 1, 0, -1, 0]
        );
        assert_eq!(
            get_pattern(2).take(15).collect::<Vec<_>>(),
            vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1]
        );
        assert_eq!(
            get_pattern(3).take(8).collect::<Vec<_>>(),
            vec![0, 0, 1, 1, 1, 0, 0, 0]
        );
    }

    #[test]
    fn test_run_phase() {
        let mut signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
        run_phase(&mut signal);
        assert_eq!(signal_to_string(&signal), "48226158");
    }

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            &first(&Input::new("80871224585914546619083218645595"))?,
            "24176176"
        );
        assert_eq!(
            &first(&Input::new("19617804207202209144916044189917"))?,
            "73745418"
        );
        assert_eq!(
            &first(&Input::new("69317163492948606335995924319873"))?,
            "52432133"
        );

        Ok(())
    }

    #[test]
    fn test_get_offset() {
        assert_eq!(
            get_offset(vec![1, 2, 3, 4, 5, 6, 7].iter().copied()),
            1234567
        );
    }
}
