# microservices

The following microservices exist:

- Account

They are build using Rust.

## Requirements

- Rust
- Docker

## Running

To run the project you can use `cargo watch -x 'run --bin {service}'`.

## Environment

The environment variables are located inside `example.env`, and should be copied to a `.env`.

Export all .env variables through `export $(cat .env | xargs)`.

## Session store

`docker run -p 6379:6379 --name leprecon-valkey valkey/valkey:7.2.5-alpine3.19`

## Kubernetes

To start container with kubernetes
`minikube start`

To start dashboard
`minikube dashboard`

To start tunnel to access gateway (only for local use)
`minikube tunnel`

Apply all objects
`kubectl apply -R -f kubernetes/`

## Azure

# Debug

Get valid image `kubectl get cm -n aks-istio-system -o yaml | grep "mcr.microsoft.com\/oss\/istio\/proxyv2"`

Go to agent pool `Node pools -> agent pool -> Nodes -> both`, and click on the `gateway-istio` image. In yaml replace container image of proxy to the one from command above.

https://learn.microsoft.com/en-us/azure/aks/istio-upgrade
