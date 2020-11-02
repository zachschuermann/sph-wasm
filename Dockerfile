FROM rust:1.46 as wasmbuilder
WORKDIR build
COPY src/ ./src/
COPY Cargo.* .
RUN cargo install wasm-pack
RUN wasm-pack build

FROM node:14 as builder
RUN mkdir /usr/src/client
COPY www/ /usr/src/client
COPY --from=wasmbuilder /build/pkg /usr/src/pkg
WORKDIR /usr/src/client
RUN npm install && npm run build

FROM nginx:stable-alpine
COPY --from=builder /usr/src/client/dist/* /usr/share/nginx/html/
COPY nginx.vh.default.conf /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
