# Bitcoin Keys Tweaker
**Bitcoin Keys Tweaker** is a web application built with [Yew](https://yew.rs) that allows users to tweak Bitcoin secret keys.
By providing two valid Wallet Import Format (WIF) secret keys, the app calculates the sum of the keys and generates a new WIF secret key for the result.

Before you get started, make sure you have the following installed:
1. Rust (latest stable version): Install from rustup.rs.
2. Trunk: The build tool for Yew projects. Install it with:
cargo install trunk

## How It Works

1. Input two valid WIF secret keys:
    - Your Secret Key: The first key for the operation.
    - Calculated Secret Key: The second key for the operation.
2. The app validates both keys and computes their sum.
3. The resulting secret key is displayed in WIF format.

This tool is useful for developers and Bitcoin enthusiasts working with custom key derivations or advanced cryptographic operations.

## Getting Started

### Clone the Repository
Clone the repository to your local machine:

```
git clone https://github.com/your-username/bitcoin-keys-tweaker.git
cd bitcoin-keys-tweaker
```
### Install Dependencies
Ensure you have all required dependencies installed. You may also need to add wasm32-unknown-unknown as a target:

```
rustup target add wasm32-unknown-unknown
```

## Run the Application
To launch the app locally:

1. Build and serve the app using trunk:
```
trunk serve
```

2. Open your browser and navigate to http://127.0.0.1:8080.
The application will automatically rebuild and refresh the browser when you make changes to the source code.


## Build for Production

To create an optimized build for production, run:

```
trunk build --release
```

The output will be placed in the dist directory.


License

This project is licensed under the MIT License.
