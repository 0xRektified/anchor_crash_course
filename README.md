# Solana Tutorial - 60 Days of Solana

This repository is based on the [Rareskills Solana Tutorial](https://rareskills.io/solana-tutorial) for Anchor development.

## About

This Solana course is designed for engineers with a beginner or intermediate background in Ethereum or EVM development to get up to speed quickly with Solana program development.

The tutorial covers 60 days of comprehensive Solana development using the Anchor framework.

## Tutorial Source

- **Tutorial Website**: [https://rareskills.io/solana-tutorial](https://rareskills.io/solana-tutorial)
- **Focus**: Anchor framework for Solana program development

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