FROM docker.io/bitnami/minideb:bullseye AS build-base
RUN apt update
RUN apt install -y \
    build-essential \
    curl \
    bash \
    clang \
    nodejs \
    wget \
    npm
RUN curl https://sh.rustup.rs -sSf | bash -s -- --profile minimal -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown
RUN wget -qO- https://github.com/thedodd/trunk/releases/latest/download/${VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
RUN cp trunk /bin
RUN trunk --version
RUN rm -rf /var/lib/{apt,dpkg,cache,log}/ /var/cache

FROM build-base AS builder
WORKDIR /usr/src
COPY . .
RUN cargo fetch --target wasm32-unknown-unknown
RUN ls
RUN trunk build --release

FROM docker.io/nginx:alpine
COPY --from=builder /usr/src/dist/ /usr/share/nginx/html
COPY --from=builder /usr/src/nginx.conf /etc/nginx/nginx.conf
CMD ["nginx", "-g", "daemon off;"]