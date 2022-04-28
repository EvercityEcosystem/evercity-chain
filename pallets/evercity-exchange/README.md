# 1. Evercity Exchange Pallet

This repositary contains source code of blockchain node, which is a main part of Evercity's exchange. The pallet is created to provide carbon credits exchange to other tokens.

# 2. Evercity exchange project main entities

 ### CarbonCreditsPackageLot 
 Struct representing pack of carbon credits for sale.
 Can include target bearer (to sell only to them). Lot has deadline, after whitch selling is impossible.

 # 3. Extrinsics
 ```rust
 create_carbon_credit_lot
 ```
 Creates a lot with an expiration moment. Lot can be private - then only targer bearer can buy Carbon Credits from the lot. 
 ```
 buy_carbon_credit_lot_units
 ```
 Buys Carbon Credits from the lot. Can buy defined amount of it, or the whole lot.


