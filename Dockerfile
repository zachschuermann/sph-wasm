FROM rust:1.46 as builder
WORKDIR build
COPY . .
RUN cargo install wasm-pack
RUN wasm-pack build

FROM node:15
COPY www/ .
COPY --from=builder /build/pkg ./pkg
EXPOSE 8080
# RUN npm install
ENTRYPOINT ["npm", "run", "start"]
