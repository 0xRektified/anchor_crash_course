# Solana Tutorial - 60 Days of Solana

This repository is based on the [Rareskills Solana Tutorial](https://rareskills.io/solana-tutorial) for Anchor development.

## About

This Solana course is designed for engineers with a beginner or intermediate background in Ethereum or EVM development to get up to speed quickly with Solana program development.

The tutorial covers 60 days of comprehensive Solana development using the Anchor framework.

## Tutorial Source

- **Tutorial Website**: [https://rareskills.io/solana-tutorial](https://rareskills.io/solana-tutorial)
- **Focus**: Anchor framework for Solana program development

## Tutorial Progress

### Foundation & Basic Concepts

#### **[Day 1](day1/programs/day1/src/lib.rs)** - Hello World
- First Anchor program
- Basic project structure
- Simple message logging with `msg!`

#### **[Day 2](day2/programs/day2/src/lib.rs)** - Functions & Parameters
- Function parameters in Anchor
- Context usage
- Basic program interaction

#### **[Day 4](day4/programs/day4/src/lib.rs)** - Error Handling
- Custom error codes
- Error propagation
- Program error management

### Storage & Data Management

#### **[Basic Storage](basic_storage/programs/basic_storage/src/lib.rs)** - Data Storage Fundamentals
- Account initialization
- Data serialization/deserialization
- Account constraints

#### **[Day 10](day10/programs/day10/src/lib.rs)** - Modular Programming
- Rust modules in Anchor
- Code organization
- External function imports

#### **[Day 13](day13/programs/day13/src/lib.rs)** - Advanced Storage
- Complex data structures
- Account relationships
- Storage optimization

#### **[Day 14](day14/programs/day14/src/lib.rs)** - Account Validation
- Account constraints
- Security checks
- Data validation patterns

### Solana-Specific Features

#### **[Day 20](day20/programs/day20/src/lib.rs)** - System Variables (Sysvars)
- Clock, rent, and other system variables
- Blockchain state access
- Time-based operations

#### **[Day 23](day23/programs/day23/src/lib.rs)** - SOL Transfers & Remaining Accounts
- Native SOL transfers
- Multiple recipient transfers
- Remaining accounts pattern

#### **[Day 24](day24/programs/day24/src/lib.rs)** - Anchor Constraints
- Account constraints deep dive
- Security patterns
- Constraint combinations

#### **[Day 25](day25/programs/day25/src/lib.rs)** - Program Derived Addresses (PDAs)
- PDA creation and usage
- Seed-based account derivation
- Keypair vs PDA accounts

#### **[Day 26](day26/programs/day26/src/lib.rs)** - PDA Ownership & Transfers
- PDA ownership management
- Account ownership transfers
- Authority patterns

#### **[Day 27](day27/programs/day27/src/lib.rs)** - Advanced PDA Operations
- Complex PDA operations
- Multi-seed PDAs
- PDA security patterns

### Transaction & Account Management

#### **[Day 28](day28/programs/day28/src/lib.rs)** - Atomic Transactions
- Transaction atomicity
- Failure simulation
- Error propagation in transactions

#### **[Day 29](day29/programs/day29/src/lib.rs)** - Program Bytecode & Storage
- Program account structure
- Bytecode location analysis
- Program data vs program account

#### **[Day 30](day30/programs/day30/src/lib.rs)** - Account Closure
- Closing accounts with `close` constraint
- Lamport redistribution
- Account lifecycle management

#### **[Day 31](day31/programs/day31/src/lib.rs)** - Account Types & Reading
- Different account types (Account vs UncheckedAccount)
- Manual account deserialization
- Account data reading patterns

### Cross-Program Communication

#### **[Day 32](day32/programs/day32/src/lib.rs)** & **[Data Reader](day32/programs/data_reader/src/lib.rs)** - Cross-Program Data Reading
- Multiple programs in same workspace
- Cross-program data access
- Manual account deserialization
- Account discriminators

#### **[Day 33](day33/programs/day33/src/lib.rs)** & **[Alice Program](day33/programs/alice/src/lib.rs)** - Cross-Program Invocations (CPI)
- CPI implementation
- Program-to-program communication
- CPI contexts and account passing

### Utility & Advanced Topics

#### **[Anchor Function Tutorial](anchor-function-tutorial/programs/anchor-function-tutorial/src/lib.rs)** - Function Patterns
- Advanced function patterns
- Best practices
- Common function architectures

#### **[Deploy Tutorial](deploy_tutorial/programs/deploy_tutorial/src/lib.rs)** - Deployment Strategies
- Program deployment
- Environment management
- Production considerations

#### **[Example Map](example_map/programs/example_map/src/lib.rs)** - Data Structures
- HashMap-like structures
- Complex data organization
- Efficient lookups

#### **[Sysvar](sysvar/programs/sysvar/src/lib.rs)** - System Variables Deep Dive
- Comprehensive sysvar usage
- System state access
- Advanced blockchain interactions

#### **[TryRust](tryrust/programs/tryrust/src/lib.rs)** - Rust Fundamentals
- Rust language features in Solana
- Memory management
- Ownership patterns

#### **[Macro Demo](macro-demo/src/lib.rs)** - Rust Macros
- Custom macro development
- Code generation
- Metaprogramming in Rust

## Testing

To run tests with Anchor, you need to follow these steps:

### 1. Start the Solana Test Validator

In your first terminal, start the validator:

```bash
solana-test-validator
```

### 2. Monitor Program Logs (Optional)

In a second terminal, stream the transaction logs to see program output including `msg!` statements:

```bash
solana logs
```

Or to watch logs for a specific program:

```bash
solana logs <PROGRAM_ID>
```

### 3. Run Tests

In a third terminal, navigate to your project directory and run the tests:

```bash
anchor test --skip-local-validator
```

The `--skip-local-validator` flag tells Anchor to use the already running validator instead of starting a new one.

## Key Learning Concepts

- **Anchor Framework**: Modern Solana development framework
- **Account Model**: Solana's unique account-based architecture
- **Program Derived Addresses (PDAs)**: Deterministic addresses for accounts
- **Cross-Program Invocations (CPI)**: Inter-program communication
- **Account Constraints**: Security and validation patterns
- **Rent & Storage**: Solana's rent system and storage economics
- **Transactions**: Atomic operations and error handling