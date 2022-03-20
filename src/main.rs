use std::collections::{HashMap};
use std::convert::{TryInto};

const MONTHLY_RENT: i32 = 2500;

struct AmmortizationPeriod {
    balance_start: f64,
    interest: f64,
    balance_end: f64,
    principal: f64
}
struct Ammortization {
    annual_interest_rate: f64,
    periods: i32,
    schedule: HashMap<i32, AmmortizationPeriod>
}

impl Ammortization {
    fn new(&mut self, annual_interest_rate: f64, periods: i32, principal: f64) -> Ammortization {
        Ammortization {
            annual_interest_rate,
            periods,
            schedule: self.generate_schedule(&principal, &annual_interest_rate, &periods)
        }
    }

    fn generate_schedule(&mut self, annual_interest_rate: &f64, periods: &i32, principal: &i32) -> HashMap<i32, AmmortizationPeriod> {
        
        // hashmap of length = periods
        let schedule = HashMap::with_capacity(periods.clone().try_into().unwrap());

        // TODO: OH BOY TIME TO IMPLEMENT SOME MATH!!!
        // How to calculate a monthly mortgage payment (TMP)
        // TMP = ( P * ( r * ( ( 1 + r )^n ) ) ) / ( ( (1 + r)^n ) - 1 );
        let P = principal.clone();
        let r = annual_interest_rate / 12.0;
        let n = 30 * 12;
        let monthly_payment = ( P * ( r * ( ( 1 + r )^n ) ) ) / ( ( (1 + r)^n ) - 1 );
        

        // assumming annualized
        // n            balance_start                  interest                principal                           balance_end        
        // 1            Bs = principal                 r * balance_start       monthly_payment - interest          Be[1] = balance_start - principal 
        // 2            Bs = B[n-1]                                                                                    Be[2] =
        // 3                                                                                                Be[3]
        // 4
        // 5
        // ...
        // periods                                                                                          Be[n]

    }
}

fn calculate_expenses() -> f64 {
    // inputs
    let principal: f64 = 500.0;
    let interest_rate: f64 = 0.05;  



    principal

}

fn main() {
    // println!("Mortage Information");

    println!("Monthly Expenses");

    let expenses = calculate_expenses();
    let income = MONTHLY_RENT.clone();

    println!("Monthly Gross Income: {}", &income);
}
