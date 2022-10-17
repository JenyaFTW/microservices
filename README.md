# Lab 1

## Team (#4):
- Яковлєв Євген ([Auth Service](https://github.com/JenyaFTW/microservices-1/tree/main/services/auth), [k8s](https://github.com/JenyaFTW/microservices-1/tree/main/k8s/auth), [Docker Image](https://hub.docker.com/layers/neura/auth-service/1/images/sha256-768b75b9ba44314871159216115c1a4808c99e5a8f927bc0dcc6b013f41a91a6?context=repo))
- Коваль Максим ([Warehouse Service](https://github.com/JenyaFTW/microservices-1/tree/main/services/warehouse), [k8s](https://github.com/JenyaFTW/microservices-1/tree/main/k8s/warehouse), [Docker Image]())
- Самохатня Міліна ([Orders Service](https://github.com/JenyaFTW/microservices-1/tree/main/services/orders), [k8s](https://github.com/JenyaFTW/microservices-1/tree/main/k8s/orders), [Docker Image](https://hub.docker.com/layers/milinass/order-service/latest/images/sha256-d98af88d629c362063968674d2d936f3132b037dfa77315ee2b93e04bae04ae0?context=repo))
- Помазан Нікіта ([Customers Service](https://github.com/JenyaFTW/microservices-1/tree/main/services/customers), [k8s](https://github.com/JenyaFTW/microservices-1/tree/main/k8s/customers), [Docker Image]())

[Client](https://github.com/JenyaFTW/microservices-1/tree/main/client), [Client k8s](https://github.com/JenyaFTW/microservices-1/tree/main/k8s/client)

## How to install (Minikube):
1) Make sure Minikube is installed: https://minikube.sigs.k8s.io/docs/start/
2) Start Kubernetes with `minikube start`
3) Enable Ingress addon with `minikube addons enable ingress`
4) Prebuild Docker images with `chmod +x docker.sh && ./docker.sh`
5) Apply k8s configurations with `kubectl apply -R -f k8s`

## How to run (Minikube):
1) Start tunnel using `minikube tunnel`
2) Access frontend on http://localhost

## API Requests

### Auth
`GET /api/auth/me - Get authenticated user`

`POST /api/auth/login - Login user`

`POST /api/auth/signup - Signup user`

### Orders
`GET /api/orders - Get all orders`

`GET /api/orders/{id} - Get orders by id`

### Customers
`GET /api/customers - Get all customers`

`GET /api/customers/{id} - Get customers by id`

### Warehouse
`GET /api/warehouse - Get all items in warehouse`

`GET /api/warehouse/{id} - Get item in warehouse by id`
