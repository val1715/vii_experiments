# Python:

Run with python3
```
python3 app.py
```
File `app_lambda.py` contains version prepared for AWS lambda usage.

# Rust:

Build for AWS lambda (worked on linux - Ubuntu 22):

```
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```
Alternatives:
https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html

Install: 
- create AWS lambda with custom runtime
- prepare package for AWS Lambda
```
cp ./target/math_chec ./target/bootstrap
zip lambda.zip ./target/bootstrap
```
- load zip package to AWS lambda function

More details for Rust with AWS Lambda is here:
https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html

# Benchmark compare results:

Python3 - count to 10_000_000, 10 loops - took 50914 ms - 810 mb memory used
Rust - count to 10_000_000, 20 loops - took 1556 ms - 156 mb memory used
