# Use a Rust image with the desired nightly version
FROM rust:1.76.0 AS builder

# Set up your project directory
WORKDIR /usr/src/app

COPY . .

# Build dependencies to cache them
RUN cargo build --release

# Expose the port your application runs on
EXPOSE 8080

# Command to run the executable
CMD ["cargo", "run"]