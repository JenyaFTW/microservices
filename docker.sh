eval $(minikube docker-env)
docker build -t warehouse:1 -f services/warehouse/Dockerfile services/warehouse
docker build -t orders:1 -f services/orders/Dockerfile services/orders
docker build -t customers:1 -f services/customers/Dockerfile services/customers
docker build -t auth:1 -f services/auth/Dockerfile services/auth
docker build -t client:1 -f client/Dockerfile client