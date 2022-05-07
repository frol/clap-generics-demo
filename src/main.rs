use clap::{Args, Subcommand, Parser};

#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(subcommand)]
    top_level: TopLevel,
}

#[derive(Subcommand, Debug)]
enum TopLevel {
    Account(Account),
    Contract(Contract),
}

#[derive(Args, Debug)]
struct Account {
    account_id: String,
    #[clap(subcommand)]
    network: NetworkArg<ViewAtBlock>
}

#[derive(Args, Debug)]
struct Contract {
    contract_id: String,
    #[clap(subcommand)]
    network: NetworkArg<SignWith>
}

#[derive(Subcommand, Debug)]
enum NetworkArg<Next: Subcommand> {
    Network(Network<Next>),
}

#[derive(Args, Debug)]
struct Network<Next: Subcommand> {
    network_name: String,
    #[clap(subcommand)]
    next: Next
}

#[derive(Subcommand, Debug)]
enum ViewAtBlock {
    Now,
    AtBlockHeight(AtBlockHeight)
}

#[derive(Args, Debug)]
struct AtBlockHeight {
    block_height: u64,
}

#[derive(Subcommand, Debug)]
enum SignWith {
    PlaintextPrivateKey,
}

fn main() {
    let cmd = CliArgs::parse();
    println!("{:#?}", cmd);
}
