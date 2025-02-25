# jqr

`jqr` is a very simple command-line tool for pretty-printing, querying, and converting JSON and YAML data.

## Features
- Pretty-print JSON data
- Extract values using JSONPath queries
- Convert JSON to YAML
- Convert YAML to JSON

## Installation

### Prerequisites
Ensure you have Rust installed. If not, install it using [Rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build the project
Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/your-repo/jqr.git
cd jqr
cargo build --release
```

To install `jqr` globally:

```sh
cargo install --path .
```

## Usage

### Pretty-print JSON

```sh
jqr file.json
```

or from stdin:

```sh
cat file.json | jqr
```

### Extract values using JSONPath

```sh
jqr file.json '$.user.name'
```

Example JSON:

```json
{
  "user": {
    "name": "Alice",
    "age": 30
  }
}
```

Command:

```sh
jqr file.json '$.user.name'
```

Output:

```json
"Alice"
```

### Convert JSON to YAML

```sh
jqr file.json --to-yaml
```

Example output:

```yaml
user:
  name: Alice
  age: 30
```

### Convert YAML to JSON

```sh
jqr file.yaml --to-json
```

Example YAML:

```yaml
user:
  name: Alice
  age: 30
```

Output:

```json
{
  "user": {
    "name": "Alice",
    "age": 30
  }
}
```

## Testing
Run the test suite with:

```sh
cargo test
```

## Contributing
1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Make your changes and commit (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature-branch`)
5. Create a pull request

## License
This project is licensed under the MIT License.

## Author
**Daniel Morlim**

