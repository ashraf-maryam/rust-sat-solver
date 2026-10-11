// SAT solver for propositional logic expressions in CNF / PoS form.
// Parses clauses, evaluates truth assignments, and reports a satisfying assignment when one exists.
//
// Example SAT:  (~P0 v P0)  satisfiable
// Example UNSAT: (P0).(~P0)
// Example: (P0 v P1) . (~P0 v P2)

const F: bool = false;
const T: bool = true;

#[derive(Clone, Copy)]
struct Preposition {
    pp: u32,   // variable number (P0 => 0)
    inv: bool, // true if it's negated (~)
}

impl Preposition {
    fn new() -> Preposition {
        Preposition {
            // "unset" value (we overwrite it when we read digits)
            pp: u32::pow(2, 31) - 1,
            inv: false,
        }
    }
}

struct Sum {
    sum: Vec<Preposition>,
}

impl Sum {
    fn new() -> Sum {
        Sum { sum: vec![] }
    }

    fn evaluate(&self, val: u32) -> bool {
        let mut x = F;

        for ppi in &self.sum {
            // Read the truth value for this variable from the assignment bit vector.
            let pp_eval: bool = ((val >> ppi.pp) & 1) != 0;

            // Invert if required (~P0 means flip the truth value).
            let pp_eval = pp_eval ^ ppi.inv;

            // Combine literals within the clause using OR.
            x = x || pp_eval;

            // OR short-circuit: if clause is already true, stop checking.
            if x == T {
                break;
            }
        }

        x
    }
}

struct POS {
    sums: Vec<Sum>,
}

impl POS {
    fn new() -> POS {
        POS { sums: vec![] }
    }

    // Parse expression like: "(P0 v P1) . (~P0 v P2)".
    fn parse(&mut self, bool_expr: &String) -> u32 {
        let bytes = bool_expr.as_bytes();
        let mut n: u8 = 0;

        let mut i: usize = 0;
        while i < bytes.len() {
            let item = bytes[i];

            if item.is_ascii_whitespace() || item == b'.' {
                i += 1;
                continue;
            }

            if item == b'(' {
                let mut sumi = Sum::new();
                let pp = Preposition::new();
                sumi.sum.push(pp);

                self.sums.push(sumi);
                i += 1;
            } else if item == b'~' {
                let sums_len = self.sums.len();
                let sum_len = self.sums[sums_len - 1].sum.len();
                self.sums[sums_len - 1].sum[sum_len - 1].inv = true;
                i += 1;
            } else if item == b'P' || item == b'p' {
                i += 1;
            } else if b'0' <= item && item <= b'9' {
                // Read a full number to support variables such as P10 and P11.
                let mut num: u32 = 0;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    num = num * 10 + (bytes[i] - b'0') as u32;
                    i += 1;
                }

                // Store the parsed variable index in the current literal.
                let sums_len = self.sums.len();
                let sum_len = self.sums[sums_len - 1].sum.len();
                self.sums[sums_len - 1].sum[sum_len - 1].pp = num;

                // Track the largest variable index so the solver knows how many assignments to test.
                if num <= u8::MAX as u32 {
                    n = std::cmp::max(n, num as u8);
                }
            } else if item == b'v' || item == b'V' {
                // Start the next literal in the current clause.
                let sums_len = self.sums.len();
                let new_pp = Preposition::new();
                self.sums[sums_len - 1].sum.push(new_pp);
                i += 1;
            } else if item == b')' {
                i += 1;
            } else {
                // Skip unexpected characters without failing silently.
                println!("Warning: skipping unexpected char: {}", item as char);
                i += 1;
            }
        }

        n as u32
    }

    // Evaluate the product of sums using AND across clauses.
    fn evaluate(&self, val: u32) -> bool {
        let mut x = T;

        for sumi in &self.sums {
            let clause_val = sumi.evaluate(val);
            x = x && clause_val;

            // AND short-circuit: if any clause is false, the full expression is false.
            if x == F {
                break;
            }
        }

        x
    }
}

fn main() {
    // Read the CNF / PoS expression from the command line.
    // Usage: cargo run -- "(P0 v P1) . (~P0 v P2)"
    let args: Vec<String> = std::env::args().collect();

    let s = if args.len() >= 2 {
        args[1..].join(" ")
    } else {
        String::from("(P0 v P1) . (~P0 v P2)")
    };

    let mut pos = POS::new();
    let n = pos.parse(&s);
    println!("\nMax variable index: P{}", n);

    // Evaluate assignments until a satisfying one is found.
    let mut t = F;
    let total = u32::pow(2, n + 1);

    for i in 0..total {
        t = pos.evaluate(i);
        if t == T {
            println!("\nSAT");
            println!("Assignment (from bits): {}", i);

            // Print variable values for the satisfying assignment.
            for var in 0..=n {
                let bit = ((i >> var) & 1) != 0;
                println!("P{} = {}", var, bit);
            }
            break;
        }
    }

    if t == T {
        println!("\nSatisfiable.");
    } else {
        println!("\nUNSAT (Not satisfiable).");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsat_simple_contradiction() {
        let s = String::from("(P0).(~P0)");
        let mut pos = POS::new();
        let n = pos.parse(&s);

        let mut found = false;
        let total = u32::pow(2, n + 1);
        for i in 0..total {
            if pos.evaluate(i) {
                found = true;
                break;
            }
        }

        assert!(!found, "Expected UNSAT, but solver found an assignment");
    }

    #[test]
    fn sat_basic_example() {
        let s = String::from("(P0 v P1).(~P0 v P2)");
        let mut pos = POS::new();
        let n = pos.parse(&s);

        let mut found = false;
        let total = u32::pow(2, n + 1);
        for i in 0..total {
            if pos.evaluate(i) {
                found = true;
                break;
            }
        }

        assert!(found, "Expected SAT, but solver did not find any assignment");
    }

    #[test]
    fn parse_handles_multi_digit_variables() {
        let s = String::from("(P10).(~P10)");
        let mut pos = POS::new();
        let n = pos.parse(&s);

        assert_eq!(n, 10, "Expected max variable index to be 10");
    }

    #[test]
    fn application_problem_1_is_sat() {
        // P0 = L, P1 = Q, P2 = B, P3 = N
        let s = String::from("(P0 v P1).(P0 v P2).(P0 v P3).(~P3 v ~P0).(P1 v P2).(~P2)");
        let mut pos = POS::new();
        let n = pos.parse(&s);

        let mut found = false;
        let total = u32::pow(2, n + 1);
        for i in 0..total {
            if pos.evaluate(i) {
                found = true;
                break;
            }
        }

        assert!(found, "Expected SAT for application problem 1");
    }
}
