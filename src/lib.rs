use std::collections::HashSet;
#[cfg(not(feature = "u64-turbocollatz"))]
type StepsType = u32;

#[cfg(feature = "u64-turbocollatz")]
type StepsType = u64;

pub fn collatz(seed: StepsType, print_steps: bool, known_good: &HashSet<StepsType>, return_steps: bool) -> Vec<StepsType> {
    let mut steps: Vec<StepsType> = vec!();
    let mut x: StepsType = seed;

    while !known_good.contains(&x) {
        if x.is_multiple_of(2) {
            x /= 2;
        }
        else {
            x = 3*x+1;
        }
        if return_steps {steps.push(x)};

        if print_steps {println!("{}", x);}
    }
    steps
}

pub fn collatz_ranged(start: StepsType, end: StepsType, use_slow: bool) -> u32 {
    let mut known_good:HashSet<StepsType> = HashSet::from([1]);
    let mut total_steps:u32 = 0;
    for i in start..=end {
        let steps:Vec<StepsType> = collatz(i, false, &known_good, true);
        total_steps += steps.len() as u32;
        if !use_slow {known_good.extend(steps);}
    }
    total_steps
}

pub fn easy_collatz(seed: StepsType, print_steps: bool) -> Vec<u32> {
    collatz(seed, print_steps, &HashSet::from([1]), true)
}
