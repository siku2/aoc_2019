use crate::input::Input;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Write;
use std::ops;
use std::str::FromStr;

fn div_ceil(a: usize, b: usize) -> usize {
    (a - 1) / b + 1
}

#[derive(Clone, Debug)]
struct ChemicalQty {
    name: String,
    amount: usize,
}

impl ChemicalQty {
    fn new(name: String, amount: usize) -> Self {
        ChemicalQty { name, amount }
    }
}

impl ops::MulAssign<usize> for ChemicalQty {
    fn mul_assign(&mut self, rhs: usize) {
        self.amount *= rhs;
    }
}

impl fmt::Display for ChemicalQty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.name)
    }
}

impl FromStr for ChemicalQty {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();
        let qty = iter.next().unwrap_or_default().parse()?;
        let name = iter.next().ok_or_else(|| "chemical name missing")?;

        Ok(Self::new(String::from(name), qty))
    }
}

fn parse_chemicals(input: &str) -> Result<Vec<ChemicalQty>, Box<dyn Error>> {
    input.split(',').map(str::trim).map(str::parse).collect()
}

fn join_iter(
    it: &mut dyn Iterator<Item = impl fmt::Display>,
    sep: &str,
) -> Result<String, fmt::Error> {
    let mut s = String::new();
    for item in it {
        write!(&mut s, "{}{}", item, sep)?;
    }

    if !s.is_empty() {
        let new_len = s.len() - sep.len();
        s.truncate(new_len);
    }

    Ok(s)
}

#[derive(Clone, Debug)]
struct Reaction {
    input: Vec<ChemicalQty>,
    output: ChemicalQty,
}

impl Reaction {
    fn scale_to_produce(&mut self, amount: usize) {
        let current_amount = self.output.amount;
        if current_amount >= amount {
            return;
        }

        let factor = div_ceil(amount, current_amount);

        for c in self.input.iter_mut() {
            *c *= factor;
        }

        self.output *= factor;
    }
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inp = join_iter(&mut self.input.iter().map(|c| c.to_string()), ", ")?;
        write!(f, "{} => {}", inp, self.output)
    }
}

impl FromStr for Reaction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sides = s.split("=>").map(str::trim);
        let input = sides
            .next()
            .ok_or_else(|| "reaction input missing".into())
            .and_then(parse_chemicals)?;
        let output = sides
            .next()
            .ok_or_else(|| "reaction output missing".into())
            .and_then(str::parse)?;

        Ok(Reaction { input, output })
    }
}

type ChemicalMap = HashMap<String, usize>;

#[derive(Debug)]
struct Reactions {
    reactions: Vec<Reaction>,
}

impl Reactions {
    fn new(reactions: Vec<Reaction>) -> Self {
        Reactions { reactions }
    }

    fn from_input(i: &Input) -> Result<Self, Box<dyn Error>> {
        i.map_lines(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(Self::new)
    }

    fn reaction_for_chemical(&self, name: &str) -> Option<&Reaction> {
        self.reactions.iter().find(|r| r.output.name == name)
    }

    fn ore_required_for(
        &mut self,
        remaining_chemicals: &mut ChemicalMap,
        chemical: &ChemicalQty,
    ) -> Option<usize> {
        let name = &chemical.name;
        let mut needed = chemical.amount;

        if name == "ORE" {
            return Some(needed);
        }

        if let Some(already_have) = remaining_chemicals.get_mut(name) {
            if *already_have >= needed {
                *already_have -= needed;
                return Some(0);
            }

            needed -= *already_have;
            *already_have = 0;
        }

        let mut reaction = self.reaction_for_chemical(name)?.clone();
        reaction.scale_to_produce(needed);

        let mut total_ore = 0;
        for c in reaction.input.iter() {
            let req = self.ore_required_for(remaining_chemicals, c)?;
            total_ore += req;
        }

        let leftover = reaction.output.amount - needed;
        if leftover > 0 {
            remaining_chemicals.insert(reaction.output.name, leftover);
        }
        Some(total_ore)
    }

    fn ore_per_fuel(&mut self) -> Option<usize> {
        self.ore_required_for(
            &mut HashMap::new(),
            &ChemicalQty::new(String::from("FUEL"), 1),
        )
    }

    fn fuel_for_ore(&mut self, total_ore: usize) -> Option<usize> {
        let mut fuel = 1;
        let mut chem = ChemicalQty::new(String::from("FUEL"), fuel);

        loop {
            chem.amount = fuel + 1;
            let ore = self.ore_required_for(&mut HashMap::new(), &chem)?;
            if ore > total_ore {
                break;
            } else {
                let ratio = total_ore as f64 / ore as f64;
                fuel = ((fuel + 1) as f64 * ratio).floor() as usize;
            }
        }

        Some(fuel)
    }
}

impl fmt::Display for Reactions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reactions = join_iter(&mut self.reactions.iter().map(|r| r.to_string()), "\n")?;
        write!(f, "{}", reactions)
    }
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut reactions = Reactions::from_input(i)?;

    reactions
        .ore_per_fuel()
        .ok_or_else(|| "no way to produce fuel".into())
        .map(|ore| ore.to_string())
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut reactions = Reactions::from_input(i)?;

    reactions
        .fuel_for_ore(1_000_000_000_000)
        .ok_or_else(|| "no way to produce fuel".into())
        .map(|ore| ore.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            first(&Input::new(
                "
                9 ORE => 2 A
                8 ORE => 3 B
                7 ORE => 5 C
                3 A, 4 B => 1 AB
                5 B, 7 C => 1 BC
                4 C, 1 A => 1 CA
                2 AB, 3 BC, 4 CA => 1 FUEL
                ",
            ))?,
            "165",
        );

        assert_eq!(
            first(&Input::new(
                "
                157 ORE => 5 NZVS
                165 ORE => 6 DCFZ
                44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                179 ORE => 7 PSHF
                177 ORE => 5 HKGWZ
                7 DCFZ, 7 PSHF => 2 XJWVT
                165 ORE => 2 GPVTF
                3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
                ",
            ))?,
            "13312",
        );

        assert_eq!(
            first(&Input::new(
                "
                2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                17 NVRVD, 3 JNWZP => 8 VPVL
                53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                22 VJHF, 37 MNCFX => 5 FWMGM
                139 ORE => 4 NVRVD
                144 ORE => 7 JNWZP
                5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                145 ORE => 6 MNCFX
                1 NVRVD => 8 CXFTF
                1 VJHF, 6 MNCFX => 4 RFSQX
                176 ORE => 6 VJHF
                ",
            ))?,
            "180697",
        );

        assert_eq!(
            first(&Input::new(
                "
                171 ORE => 8 CNZTR
                7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                114 ORE => 4 BHXH
                14 VRPVC => 6 BMBT
                6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                5 BMBT => 4 WPTQ
                189 ORE => 9 KTJDG
                1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                12 VRPVC, 27 CNZTR => 2 XDBXC
                15 KTJDG, 12 BHXH => 5 XCVML
                3 BHXH, 2 VRPVC => 7 MZWV
                121 ORE => 7 VRPVC
                7 XCVML => 6 RJRHP
                5 BHXH, 4 VRPVC => 5 LTCX
                ",
            ))?,
            "2210736",
        );

        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            second(&Input::new(
                "
                157 ORE => 5 NZVS
                165 ORE => 6 DCFZ
                44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                179 ORE => 7 PSHF
                177 ORE => 5 HKGWZ
                7 DCFZ, 7 PSHF => 2 XJWVT
                165 ORE => 2 GPVTF
                3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
                ",
            ))?,
            "82892753",
        );

        assert_eq!(
            second(&Input::new(
                "
                171 ORE => 8 CNZTR
                7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                114 ORE => 4 BHXH
                14 VRPVC => 6 BMBT
                6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                5 BMBT => 4 WPTQ
                189 ORE => 9 KTJDG
                1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                12 VRPVC, 27 CNZTR => 2 XDBXC
                15 KTJDG, 12 BHXH => 5 XCVML
                3 BHXH, 2 VRPVC => 7 MZWV
                121 ORE => 7 VRPVC
                7 XCVML => 6 RJRHP
                5 BHXH, 4 VRPVC => 5 LTCX
                ",
            ))?,
            "460664",
        );

        Ok(())
    }
}
