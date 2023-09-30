# Simple CV generator

[![GitHub license](https://img.shields.io/github/license/alk-neq-me/react-native-expo-using-native-with-rust.svg?style=for-the-badge)](https://github.com/alk-neq-me/simple_cv_generator/blob/master/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/alk-neq-me/react-native-expo-using-native-with-rust.svg?style=for-the-badge)](https://github.com/alk-neq-me/simple_cv_generator/stargazers)

---

## Table of Contents

- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Help](#help)
- [Usage](#usage)
- [License](#license)

---

## Getting Started

To get this project up and running on your local machine, follow the instructions below.

### Prerequisites

Make sure you have the following tools and dependencies installed on your system:

- [Rust](https://)

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/alk-neq-me/simple_cv_generator
   ```

2. Change into the project directory:
   ```sh
   cd simple_cv_generator/
   ```

---

## Help
```sh
cargo run --release --help
```

---

## Usage

### Create config file
```sh
mkdir config/ output/
```

### Choose a template
```sh
# basic template
cp ./templates/basic/basic_example.toml my_basic.toml

# edit your toml file with code editor such as `Vim` or `VSCode`
# for vim
vim ./config/my_basic.toml
```

### Compile toml to your resume html file
```sh
# basic template
cargo run --release -- --config-file ./config/my_basic.toml --template basic --output-file ./output/my_basic.html
```

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
