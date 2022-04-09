use std::collections::HashMap;
use std::convert::TryInto;
use std::env;

use dotenvy::dotenv;

use reqwest::Url;

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
        let p = loan_amount.clone();
        let r = annual_interest_rate / 12.0;
        let n = periods * 12;
        let total_monthly_payment =
            (p * (r * ((1.0 + r).powf(n.into())))) / (((1.0 + r).powf(n.into())) - 1.0);
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

#[tokio::main]
async fn main() {
    //TODO: command line inputs

    // load environment variables
    dotenv().ok();

    println!("Monthly Expenses");

    let expenses = calculate_expenses();
    let income = MONTHLY_RENT.clone();
    
    let address = "137 Highland Ave, Jersey City, NJ 07306";

    // query rentometer for the rents for a given property
    //let market_rent = Rentometer::new(&address); 
    let http_client = reqwest::Client::new();
    const RENTOMETER_URL: &str = "https://www.rentometer.com/api/v1/summary";
    // bedrooms is required by the rentometer api
    // TODO: query the bedrooms from some public listing so
    // we can just enter the address alone
    let bedrooms = "2";

    let search_params = [
        ("api_key", env::var("RENTOMETER_API_KEY").ok().unwrap()), // <-- ?
        ("address", address.to_string()), 
        ("bedrooms", bedrooms.to_string())
    ];

    let body = 
        http_client
            .get(RENTOMETER_URL.clone())
            .query(&search_params)
            .send()
            .await
            // await? <-- why doesn't this work????
            .ok()
            .unwrap()
            .text()
            .await
            // await? < -- and this!!
            .ok()
            .unwrap();

    println!("Your rentometer response: {}", body);
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
