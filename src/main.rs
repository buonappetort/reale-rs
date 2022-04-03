use std::collections::HashMap;
use std::convert::TryInto;

const MONTHLY_RENT: i32 = 2500;

struct AmmortizationPeriod {
    balance_start: f64,
    interest: f64,
    balance_end: f64,
    principal: f64,
}
struct Ammortization {
    annual_interest_rate: f64,
    periods: i32,
    total_monthly_payment: f64,
    loan_amount: f64, // Should this be stored? Or just live
                      // as a method...
                      //schedule: HashMap<i32, AmmortizationPeriod>
}

impl Ammortization {
    fn new(annual_interest_rate: f64, periods: i32, loan_amount: f64) -> Ammortization {
        // TODO: OH BOY TIME TO IMPLEMENT SOME MATH!!!
        // How to calculate a monthly mortgage payment (TMP)
        // TMP = ( P * ( r * ( ( 1 + r )^n ) ) ) / ( ( (1 + r)^n ) - 1 );
        let P = loan_amount.clone();
        let r = annual_interest_rate / 12.0;
        let n = periods * 12;
        let total_monthly_payment =
            (P * (r * ((1.0 + r).powf(n.into())))) / (((1.0 + r).powf(n.into())) - 1.0);
        Ammortization {
            annual_interest_rate,
            periods,
            total_monthly_payment,
            loan_amount,
        }
    }

    fn generate_schedule(
        &mut self,
        _principal: &f64,
        _annual_interest_rate: &f64,
        periods: &i32,
    ) -> HashMap<i32, AmmortizationPeriod> {
        // hashmap of length = periods
        HashMap::with_capacity(periods.clone().try_into().unwrap())

        // TODO: OH BOY TIME TO IMPLEMENT SOME MATH!!!
        // How to calculate a monthly mortgage payment (TMP)
        // TMP = ( P * ( r * ( ( 1 + r )^n ) ) ) / ( ( (1 + r)^n ) - 1 );
        // let P = principal.clone();
        // let r = annual_interest_rate / 12.0;
        // let n = 30 * 12;
        // let monthly_payment = ( P * ( r * ( ( 1 + r )^n ) ) ) / ( ( (1 + r)^n ) - 1 );

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

fn calculate_expenses() -> () {
    // inputs
    let loan_amount: f64 = 200000.00;
    let annual_interest_rate: f64 = 0.05;

    let ammortization = Ammortization::new(annual_interest_rate, 30, loan_amount);

    print!(
        "Your total monthly payment is: {:?}",
        ammortization.total_monthly_payment
    );
}

fn main() {
    // println!("Mortage Information");

    println!("Monthly Expenses");

    let expenses = calculate_expenses();
    let income = MONTHLY_RENT.clone();

    println!("Monthly Gross Income: {}", &income);
}

#[test]
fn test1() {
        // inputs
        let loan_amount: f64 = 200000.00;
        let annual_interest_rate: f64 = 0.05;
    
        let ammortization = Ammortization::new(annual_interest_rate, 30, loan_amount);
    
        assert_eq!(ammortization.total_monthly_payment.ceil(), 1074.00)
}
