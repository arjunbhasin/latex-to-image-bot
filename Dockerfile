# Use a Python base image
FROM python:3.9

# Set the working directory in the container
WORKDIR /usr/src/app

# Install Matplotlib, Python development headers, and system packages for LaTeX
RUN pip install matplotlib \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        # LaTeX packages required for Matplotlib LaTeX rendering
        #texlive-latex-base \
        #texlive-fonts-recommended \
        #texlive-fonts-extra \
        #texlive-latex-extra \
    && rm -rf /var/lib/apt/lists/*

# Install Rust compiler and cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the current directory (which includes your Rust project and Python script) into the Docker image
COPY . /usr/src/latex-to-image-bot

# Set the working directory to your Rust crate
WORKDIR /usr/src/latex-to-image-bot

# Build your Rust crate
RUN cargo build --release

# Set the command to run your Rust application
CMD ["./target/release/latex-to-image-bot"]