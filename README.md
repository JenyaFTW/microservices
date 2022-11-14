## Team (#4):
- Яковлєв Євген ([Auth Service](https://github.com/JenyaFTW/microservices-1/tree/main/services/auth), [k8s](https://github.com/JenyaFTW/microservices-1/tree/main/k8s/auth), [Docker Image](https://hub.docker.com/layers/neura/auth-service/1/images/sha256-768b75b9ba44314871159216115c1a4808c99e5a8f927bc0dcc6b013f41a91a6?context=repo))
- Коваль Максим ([Warehouse Service](https://github.com/JenyaFTW/microservices-1/tree/main/services/warehouse), [k8s](https://github.com/JenyaFTW/microservices-1/tree/main/k8s/warehouse), [Docker Image](https://hub.docker.com/layers/maksolo27/warehouse-service/1/images/sha256-7b4ae6150b5888108de495146b651be7b9f29f2b0c8e3e376849bd959cfb254a?context=repo))
- Самохатня Міліна ([Orders Service](https://github.com/JenyaFTW/microservices-1/tree/main/services/orders), [k8s](https://github.com/JenyaFTW/microservices-1/tree/main/k8s/orders), [Docker Image](https://hub.docker.com/layers/milinass/order-service/1/images/sha256-d98af88d629c362063968674d2d936f3132b037dfa77315ee2b93e04bae04ae0?context=repo))
- Помазан Нікіта ([Customers Service](https://github.com/JenyaFTW/microservices-1/tree/main/services/customers), [k8s](https://github.com/JenyaFTW/microservices-1/tree/main/k8s/customers), [Docker Image](https://hub.docker.com/layers/pomazzanus/customer-docker/latest/images/sha256-7ef44f070c78a5263a031dc46a773ada0741dff85e05fd98954e9b87f8e9b8e2?context=repo))

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

`GET /api/orders/get/{orderId} - Get order by id`

`POST /api/orders/create - Create order`

`PUT /api/orders/update/{orderId} - Update order by id`

`DELETE /api/orders/delete/{orderId} - Delete order by id`

### Customers
`GET /api/customer - Get all customers`

`GET /api/customer/{id} - Get customer by id`

`POST /api/customer - Create customer`

`PUT /api/customer/{id} - Update customer by id`

`DELETE /api/customer/{id} - Delete customer by id`

### Warehouse
`GET /api/warehouse - Get all items in warehouse`

`GET /api/warehouse/get/{id} - Get item in warehouse by id`

`POST /api/warehouse/create - Create item in warehouse`

`POST /api/warehouse/update/{id} - Update item in warehouse by id`

`DELETE /api/warehouse/delete/{id} - Delete item in warehouse by id`