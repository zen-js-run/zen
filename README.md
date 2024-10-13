# Zen

Zen is a high-performance JavaScript runtime built in Rust. It executes code faster than Node.js and Deno, making it an efficient choice for server-side JavaScript applications.

## Features

- Fast execution of JavaScript code
- Cross-platform support (Linux and Windows)
- Built using Rust for safety and performance

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Docker (for containerization)

### Installation

To build and run Zen locally, follow these steps:

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/Zen.git
   cd Zen
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Install the project:
    ```bash
    cargo install --path .
    ```

### Usage

You can run a JavaScript file using Zen with the following command:

```bash
zen input.js
```

### Docker Setup

Zen can also be built and run using Docker. The provided Dockerfile allows you to create an image that contains both the Linux and Windows binaries of Zen.

1. Build the Docker image:

   ```bash
   docker build -t zen .
   ```

2. Run the Zen application in a Docker container:

   ```bash
   docker run --rm zen <path_to_js_file>
   ```

### Dockerfile Explanation

The Dockerfile consists of two main stages:

1. **Builder Stage**: 
   - Uses the official Rust image to build the Zen application.
   - Copies the source code and required files.
   - Compiles the application for both Linux and Windows targets.

2. **Runtime Stage**: 
   - Uses a minimal Debian image to run the application.
   - Copies the built binaries from the builder stage.

### Example

To run a JavaScript file named `script.js`, execute:

```bash
zen script.js
```

Or, using Docker:

```bash
docker run --rm zen script.js
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have suggestions or improvements.

## License

This project is licensed under the Apache 2.0 License. See the LICENSE file for more details.