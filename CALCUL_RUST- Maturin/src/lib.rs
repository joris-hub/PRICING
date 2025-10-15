// Price a call/put option using the Monte Carlo method.
// Monte Carlo = simulate many possible future paths of the stock, compute the payoff at maturity, and take the discounted average.
// The stock price at time t = drift (expected trend) + diffusion (random fluctuations driven by volatility).



use rand::thread_rng;
use rand_distr ::{Normal, Distribution}; // used to draw a number from a normal distribution

use statrs::distribution::{Normal as StatNormal}; // used for the Black-Scholes method to use the cdf. We call it StatNormal to avoid confusion with Normal 
use statrs::distribution::ContinuousCDF;

use pyo3::prelude::*;

use std::env;


#[pyfunction]
fn montecarloCallPrice(nSim :i32, S:f64, K:f64, r:f64, v:f64, T:f64)->PyResult<f64>{
    let drift: f64 = (r - 0.5*v*v)*T;
    let diffusion: f64 = v*((T).sqrt());
    let mut payoffSum: f64 = 0.0;

    let mut generator = thread_rng(); // must be mutable because its state changes with each draw
    let normal = Normal::new(0.0, 1.0).unwrap();

    for i   in 0..nSim{
        let Z: f64 = normal.sample(&mut generator);
        let SForward: f64 = S*((drift + diffusion*Z).exp());

        payoffSum += 0_f64.max(SForward - K ); // a and b must be the same type to use a.max(b)

    }
    Ok(payoffSum/(nSim as f64) * ((-r * T).exp())) // E(payoff) * exp(-rT) because we discount at the continuous rate
}

#[pyfunction]
fn montecarloPutPrice(nSim :i32, S:f64, K:f64, r:f64, v:f64, T:f64)->PyResult<f64>{
    let drift: f64 = (r - 0.5*v*v)*T;
    let diffusion: f64 = v*((T).sqrt());
    let mut payoffSum: f64 = 0.0;

    let mut generator = thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();

    for i   in 0..nSim{
        let Z: f64 = normal.sample(&mut generator);
        let SForward: f64 = S*((drift + diffusion*Z).exp());

        payoffSum += 0_f64.max(K - SForward ); // the only difference with monteCarloCallPrice

    }
    Ok(payoffSum/(nSim as f64) * ((-r * T).exp()))
}

#[pyfunction]
fn BSCallPrice(S:f64, K:f64, r:f64, v:f64, T:f64)->PyResult<f64>{
    let d1: f64 = ((S/K).ln() + (r + 0.5*v*v)*T) / (v*((T).sqrt()));
    let d2: f64 = d1 - v*T.sqrt();

    let normal = StatNormal::new(0.0, 1.0).unwrap();

    let C = (S)*normal.cdf(d1) - (K)*((-r*T).exp())*normal.cdf(d2); 
    Ok (C)
}

#[pyfunction]
fn BSPutPrice(S:f64, K:f64, r:f64, v:f64, T:f64)->PyResult<f64>{
    let d1: f64 = ((S/K).ln() + (r + 0.5*v*v)*T) / (v*((T).sqrt()));
    let d2: f64 = d1 - v*T.sqrt();

    let normal = StatNormal::new(0.0, 1.0).unwrap();

    let C = -(S)*normal.cdf(-d1) + (K)*((-r*T).exp())*normal.cdf(-d2); 
    Ok(C)
}

#[pymodule]
fn calcul_rust(_py : Python, m :&PyModule)-> PyResult<()> {
    m.add_function(wrap_pyfunction!(montecarloCallPrice,m)?)?;
    m.add_function(wrap_pyfunction!(montecarloPutPrice,m)?)?;
    m.add_function(wrap_pyfunction!(BSCallPrice,m)?)?;
    m.add_function(wrap_pyfunction!(BSPutPrice,m)?)?;
    Ok(())
}

// PyResult is a Result<T, PyErr>. We either return Ok(value) or Err(PyErr).

//`m` is the module we are "filling".  
//`_py` is just to allow access from Python.  
// Here, PyResult is empty because we only want to expose functions to Python; we don’t want the module itself to return a value.
//sss