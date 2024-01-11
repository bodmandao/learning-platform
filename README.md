# ICP Lottery App

This is a simple lottery application running on the Internet Computer (ICP). It allows users to buy lottery tickets, conduct lottery draws, participate in draws, and query information about tickets and draws.

## Features

### 1. Buy Lottery Ticket

#### Function Signature
```rust
#[ic_cdk::update]
fn buy_lottery_ticket(owner: String, numbers: Vec<u32>) -> Result<LotteryTicket, LotteryError>
```
This function allows users can purchase a lottery ticket.

```rust
#[ic_cdk::query]
fn check_lottery_ticket(id: u64) -> Result<LotteryTicket, LotteryError>
```
This function allows users to check the details of a purchased lottery ticket by providing its ID.
If the ticket with the specified ID is found, its details are returned. Otherwise, a NotFound error is returned.

```rust
#[ic_cdk::update]
fn conduct_lottery_draw(winning_numbers: Vec<u32>) -> Result<LotteryDraw, LotteryError>
```
This function conducts a lottery draw by providing the winning numbers.

```rust
#[ic_cdk::update]
fn participate_in_lottery_draw(ticket_id: u64, draw_id: u64) -> Result<LotteryDraw, LotteryError>
```
This function allows a user to participate in a specific lottery draw by providing the IDs of their purchased ticket and the target draw.

```rust
#[ic_cdk::query]
fn get_all_lottery_tickets() -> Result<Vec<LotteryTicket>, LotteryError>
```
This function retrieves a list of all purchased lottery tickets.

```rust
#[ic_cdk::query]
fn get_all_lottery_draws() -> Result<Vec<LotteryDraw>, LotteryError>
```
This function retrieves a list of all conducted lottery draws.

You can run the following commands to start working on it :

```bash
cd icp-lottery/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
