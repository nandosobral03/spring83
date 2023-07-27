# Stage 1: Building
FROM rust:1.67 as builder
WORKDIR /usr/src

# Install necessary tools and dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    file \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update
RUN apt install -y pkg-config libssl-dev

# Set the working directory inside the container
WORKDIR /server

# Copy the source files into the container
COPY /server /server

# # The compiled executable will be available at /app/target/release/
WORKDIR /server
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /server
COPY --from=builder /server/target/release/server ./
COPY --from=builder /server/.env ./
COPY /server/src/assets /server/src/assets
RUN apt-get update && apt-get install -y 

RUN apt install -y pkg-config libssl-dev
RUN rm -rf /var/lib/apt/lists/*

EXPOSE 3000

CMD ["./server"]