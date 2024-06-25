# Currency Converter

## Table of Contents
- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
- [Tests](#tests)
- [Contributing](#contributing)
- [License](#license)

## Introduction
Currency Converter is a Rust library that provides functionality for converting currency amounts between different currencies. It offers a simple and straightforward interface for performing currency conversions based on the latest exchange rates.

This documentation will guide you through the installation process, explain how to use the library in your code, provide information about running tests, and outline how you can contribute to the project.

## Installation
To use Currency Converter, follow these steps:

1. Ensure you have Rust installed. If not, you can install it by following the official [Rust installation guide](https://www.rust-lang.org/tools/install).

2. Run the following command to fetch and build the dependency:

   ```bash
   cargo update
   ```

## Usage

```
exchange <amount> <from_currency> to <to_currency>

./exchange 100 usd to dkk
>> Converted amount: 689.00 DKK

```

## Release & Run

```
bash release.sh

```

## Tests
Currency Converter includes a comprehensive suite of unit tests to ensure the correctness of the codebase. To run the tests, use the following command:

```bash
cargo test
```
