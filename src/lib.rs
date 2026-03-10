use std::collections::HashSet;
#[cfg(not(feature = "u64-turbocollatz"))]
pub type StepsType = u32;

#[cfg(feature = "u64-turbocollatz")]
pub type StepsType = u64;

/// Given a starting point and a list of stopping points, verify the Collatz conjecture and return the steps if requested using return_steps.
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

/// Verify a range of integers and then return the total number of steps that it took to complete. If use_slow is set to true, then memory-intensive caching will be disabled at the cost of performance.
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

/// A simplified interface for collatz().
pub fn easy_collatz(seed: StepsType, print_steps: bool) -> Vec<StepsType> {
    collatz(seed, print_steps, &HashSet::from([1]), true)
}
