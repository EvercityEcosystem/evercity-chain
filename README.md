![Compilation and Testing Suite](https://github.com/EvercityEcosystem/evercity-substrate/workflows/Compilation%20and%20Testing%20Suite/badge.svg?branch=master)
# 1. Evercity Chain Substrate Node

This repositary contains source code of blockchain node, which is a main part of Evercity's Smart Sustainable Bond project.

![Web3 Foundation Badge](https://raw.githubusercontent.com/EvercityEcosystem/evercity-substrate/master/docs/web3_foundation_badge.jpg)

# 2. Introduction

Sustainable finance is on the rise. The market of green and sustainability-linked bonds exceeded half a trillion USD (517.4bn) in 2021, and the carbon credit market reached a new height of $272 billion. 

However, there are still challenges preventing growth of these markets: low accuracy and transparency of impact measurement due to lack of on-site data; fraud risks; lack of tools to distribute financial and impact results between projects and investors; high back office costs; relatively low liquidity and profitability; double accounting; lack of incentives for issuers. 

One way to add more incentives for issuers identified by UNFCCC and BIS is integration of green bonds and carbon credits. The key idea is that if buyers of the green bond can also receive tradable carbon credits generated by the financed project, they will have additional incentive to pay the premium for the green bond, which will create more incentives for companies to issue green bonds. Also, integration of carbon credit and green bond markets will make carbon credits more attractive for investors and boost the carbon market by adding more liquidity. 

Other challenges related to transparency, traceability, double accounting, impact allocation and high costs can be solved by digital technologies (such as blockchain, IoT, AI, satellite monitoring), which is confirmed in the UN and EU documents. At the same time, a challenge preventing rapid blockchain adoption in climate finance still lies in the high carbon footprint of main PoW blockchains including Ethereum. Parity Substrate blockchain has many advantages having a low carbon footprint, as well as enabling interoperability and scalability.

Evercity aims to realize this potential by building an open-source Sustainable Finance protocol based on Parity Substrate blockchain - digital infrastructure which enables end to end lifecycle of traditional green bonds, sustainability-linked bonds with adjustable floating impact-linked coupon rate, as well as green bonds linked to carbon credits. The protocol is supported by Web3 Foundation which nurtures and stewards technologies and applications in the fields of decentralized web software protocols.


# 3. Overview

Powered by Parity Substrate blockchain engine, Sustainable Finance Protocol is an open-source software which allows participants to issue and monitor traditional green bonds, sustainability-linked bonds with adjustable floating impact-linked coupon rate, as well as green bonds linked to carbon credits. The main idea of the project is to increase accuracy of impact monitoring and reporting eliminating the risk of greenwashing, as well as to enable fair and transparent impact allocation between different stakeholders engaged in sustainability-related projects. The main operations performed are confirmed by blockchain digital signatures and can be traced publicly. The platform stablecoin EVERUSD cannot be used outside the platform, which eliminates the risks of money laundering.

Evercity Carbon Credits pallet allows issuing carbon credits according to any standard (or even creating own standard using customizable frameworks) as a result of interaction between various stakeholders: project owners, standard representatives, auditors and registries. We are replicating the globally accepted life cycle of carbon credits on blockchain making it more transparent, efficient and accessible. Key target audiences of our product are project owners who issue carbon credits, companies who want to offset their emissions as well as blockchain projects who want to offset the carbon footprint of their transactions. Evercity Exchange pallet allows trading EVERUS, which originates in the bond pallet, to carbon credits and vice versa.


# 4. Evercity project main entities

Evercity pallet implements actions for three types of entities: 
    1) accounts and roles, 
    2) token balances,
    3) operations with bonds and carbon credits.

### 4.1 Accounts and roles

Each Evercity account can accommodate one or more roles from the bond and carbon credit pallets. The account only has access to the functions available to its role(s). The approximate functions of each role in the project are as follows:

Bond roles

- MASTER: the administrative role that can create new accounts and assign roles to them. This role also regulates the launch of bonds to the market, making the final decision on whether the bond meets the requirements.
- CUSTODIAN: the role which can mint and burn the main platform token. This role is assigned to the public account of the partner bank, which exchanges USD --> EVERUSD and EVERUSD --> USD.
- ISSUER: the role which can create bonds. An account with the ISSUER role issues a bond to fund a sustainability-aligned project. After receiving funds from the sale of Bond Units, the ISSUER undertakes to provide data on the impact of the project, which influences the coupon rate that should be paid to the investor. The ISSUER is obliged to replenish the bond balance with the amount necessary to cover its financial obligations.
- INVESTOR: accounts with the INVESTOR role use the EVERUSD token to buy Bond Units and sell them on the secondary market. Each billing period Investor receives a coupon income proportional to its balances of various Bond Units
- AUDITOR: these accounts check and confirm the environmental impact data sent by Issuer, as well as certify the documents uploaded to the platform
MANAGER: the task of accounts with this role is to help Issuers work with projects, verify data and prepare documents

Carbon credit roles
- CC_PROJECT_OWNER: the role which can create carbon projects, annual reports and issue carbon credits.
- CC_STANDARD; CC_AUDITOR; CC_REGISTRY: the roles which sign project documentation and annual reports (the order of signatures is determined by Carbon Standard entity).


### 4.2 Account balances and operations 

The EVERUSD platform token is a stablecoin strictly linked 1 to 1 to one of the fiat currencies (USD). The platform token is a reflection of the financial obligations of the participants and is not a means of payment. EVERUSD token cannot be freely sent from one address to another. Any operation that changes EVERUSD balances must have a justification transaction on the platform. It can be: buying Bond Units, receiving a coupon income, selling Bond Units on the secondary market, etc.

The mechanics of EVERUSD are such that it always has verified fiat collateral. Initially, there are 0 EVERUSD-s on the platform. The creation (mint) of new EVERUSD is possible only upon request from accounts that have passed KYC and provided the Bank-Custodian with documents confirming the payment. The same scheme is used to destroy (burn) EVERUSD tokens on users' balances - strictly at their request and with confirmation of the payment of the corresponding amount in fiat currency. Here is an example of how this “mirror” scheme works:

 - Investor creates a request for the purchase of 1000 EVERUSD and sends 1000 USD from his account to the bank
 - The bank verifies the payment, verifies the Investor's identity, and confirms the application by adding +1000 EVERUSD to the Investor’s balance
 - Investor performs transactions on the platform, buying Bond Units, receiving coupon income, organizing transactions in the secondary market, etc., while a part of his EVERUSD goes to the Issuer balance (e.g. 800 EVERUSD)
 - Issuer creates a request for withdrawal of 800 USD with burning 800 EVERUSD on its balance
 - The bank sees a request for burning of EVERUSD from the Issuer, receives proof of the legality of receiving EVERUSD from observing the contracts on the platform, and confirms the application. Tokens are burned, and 800 USD are sent to Issuer.

With this scheme on the platform, any transaction with the EVERUSD token has a strict confirmation on the blockchain, which can be presented as proof of transparency of all the money flows on the platform, and each EVERUSD has a guaranteed collateral.

### 4.3 Operations with bonds and carbon credits

Bonds 

Bonds are the one of main essences of the project. The logic of the work of Evercity bonds copies the logic of the traditional issuance of bonds in the financial markets, but links the impact data on the use of proceeds with the financial parameters of the bond. The data on the environmental impact of the project uploaded to the blockchain changes the coupon rate. The parameters for these changes are configured at the stage of bond structuring. This mechanism is described in more detail in the platform scenario [here](https://github.com/EvercityEcosystem/smart-sustainable-bond).

Carbon credits

Carbon credits pallet has several main entities:

- Project - Entity for signing carbon credits project documentation and creating annual reports
- Carbon Standard - Algebraic type which determines the order of signature among three roles: CC_AUDITOR, CC_STANDARD, CC_REGISTRY
- Annual Report - Entity for confirming annual volume of carbon credit issuance
- Carbon Credit Passport - Entity for registering carbon credits as assets
- Carbon Offset Certificate - Entity for granting certificates for carbon emissions offsetting using carbon credits

You can find more info about carbon credits flow in this [documentation](https://github.com/EvercityEcosystem/evercity-chain/blob/master/pallets/evercity-carbon-credits/README.md)

# 5. Evercity project scenario

1. Issuer starts structuring a green or sustainability-linked bond (flow description [here](https://github.com/EvercityEcosystem/smart-sustainable-bond)). In case the issuer also wants to issue carbon credits based on the project performance, there is an additional optional field, where he can describe allocation of future carbon credits to different roles and accounts: 
- Issuer’s account
- Evercity account (commission fee), 
- Project developer’s account,
- Investors’ accounts (exact addresses are not known yet).
2. The bond undergoes its standard flow as described [here](https://github.com/EvercityEcosystem/smart-sustainable-bond). 
3. After the bond reaches FINISHED state, carbon project can be structured based on the flow described [here](https://github.com/EvercityEcosystem/carbon-credits). During the structuring, a corresponding bond ticker is also indicated. 
4. Carbon project is approved by CC_AUDITOR and CC_REGISTRY. 
5. The issuer creates a report where he indicates a number of issued carbon credits (based on the bond project impact), as well as their unique ticker. The report is also approved by CC_AUDITOR and CC_REGISTRY. 
6. Approved report enables issuance of Carbon Credit units. 
7. Issued Carbon Credit units are automatically distributed among the accounts and roles indicated at step 1. 
8. Distributed Carbon Credit units can be transferred, retired and sold for EVERUSD in [pallet-evercity-exchange](https://github.com/EvercityEcosystem/evercity-chain/tree/master/pallets/evercity-exchange).


# 6. Evercity documentation

### 6.1 Runtime methods

Methods of pallet-evercity are described in Rust documentation which can be generated by the `cargo doc` command

### 6.2 Build

```bash
git clone https://github.com/EvercityEcosystem/evercity-chain.git
cd evercity-chain
cargo build --release
```

### 6.3 Run

#### 6.3.1 Single Node Development Chain

Purge any existing dev chain state:

```bash
./target/release/evercity-node purge-chain --dev
```

#### 6.3.2 Remove chains with all data

[WARNING] All chains data is usually located in ```$HOME/.local/share/evercity-node/chains/*```
Removing of all chains: "dev", "local-testnet", and any others to launch all chains from block "0" can be made by:
```
rm -rf $HOME/.local/share/evercity-node/chains/*
```

#### 6.3.3 Start a development chain:

```bash
./target/release/evercity-node --dev
```
### 6.4 Build docker image
```bash
cargo build --release
docker build --tag evercity-node:1.0
```

### 6.5 Configure Node as a service
 Build
```bash
cargo build --release
```
 Copy ./target/release/evercity-node into /usr/bin/evercity/.
 Create /var/run/evercity directory.

 Put evercity.service in /etc/systemd/system/
```
 [Unit]
 Description=Evercity substrate test service
 After=network.target
 StartLimitIntervalSec=0
 [Service]
 Type=simple
 Restart=always
 RestartSec=1
 User=evercity
 ExecStart=/usr/bin/evercity/evercity-node  --dev --rpc-external --ws-external  --ws-port 9940  --rpc-port 9930 --port 30300 --base-path /var/run/evercity
```
```bash
systemctl enable evercity
```
start service
```bash
systemctl start evercity
```
### 6.6 Running tests

```bash
cargo test
```
```bash
./target/release/evercity-node --dev
```

### 6.7 Generate documentation

```bash
cargo doc
```

### 6.8 Run evercity-chain node via makefile 

### 6.8.1 Using in-memory storage

```bash
make run
```
### 6.8.2 Using persistent storage

```bash
make run-local
```
