import ast

import requests
import random
from datetime import datetime

import calcul_rust


api_key = "demo"
symbol = "IBM"  # we will price IBM options

url_options = f"https://www.alphavantage.co/query?function=HISTORICAL_OPTIONS&symbol={symbol}&apikey={api_key}"
url_actions = f"https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={symbol}&apikey={api_key}"


# get the options
response = requests.get(url_options)
dico = response.json()

dico_options = dico["data"]
option = dico_options[random.randint(0, len(dico_options)-1)] # we choose a random option

# get the stocks
respone_actions = requests.get(url_actions)
dico_actions = respone_actions.json()

S = float((dico_actions["Global Quote"]["05. price"]))
K = float(option["strike"])
v = float(option["implied_volatility"])
option_type = option["type"]

print("C'est une option de type : ", option_type)
print("Le prix de l'action est de : ", S)
print("Le strike est de :", K)
print("L'expiration est le :", option["expiration"])
print("La volatilité est: ", v)

#----------------
# get the maturity in days, that's how we calculate it with Monte Carlo
expiration_str = option["expiration"]
expiration_date = datetime.strptime(expiration_str, "%Y-%m-%d") # expiration_date is a datetime object, not a number

diff_days = (expiration_date - datetime.today()).days

T = diff_days/365 

#-----------------

r = 0.05

if option_type == "call":
    print("1 pour Monte-Carlo, 2 pour Black Scholes")
    choix = input()
    if choix == 1 : 
        print(calcul_rust.montecarloCallPrice(1000, S, K, r, v, T))
    else:
        print(calcul_rust.BSCallPrice(S, K, r, v, T))
elif option_type == "put":
    print("1 pour Monte-Carlo, 2 pour Black Scholes")
    choix = input()
    if choix == 1 : 
        print(calcul_rust.montecarloPutPrice(1000, S, K, r, v, T))
    else:
        print(calcul_rust.BSPutPrice(S, K, r, v, T))