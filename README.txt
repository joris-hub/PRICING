OPTION PRICING USING RUST AND PYTHON

This project is composed of a Python file and a Rust folder. 

Python file :
	- calls a demo API from alphavantage to extract the options data
	- uses the "calcul_rust" module to price a random option 
	- asks the user whether to use a Monte-Carlo method or the Black-Scholes method.

Rust folder : 
	- Cargo.toml : defines the Rust module and dependencies (PyO3)
	- lib.rs : the main part of the project, it's where I create the functions of the module


How to run :

To use the Rust module in your Python environment, you need to install it using `maturin`:  
	- Open the anaconda prompt
	- Navigate to the Rust folder
	- maturin develop


Note : I used Maturin and PyO3 to create this project. PyO3 is used in lib.rs to create the functions of the module. Maturin is only used to install the module in the Python environnement.
