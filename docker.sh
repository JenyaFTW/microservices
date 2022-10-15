eval $(minikube docker-env)
docker build -t warehouse -f services/warehouse/Dockerfile services/warehouse
docker build -t orders -f services/orders/Dockerfile services/orders
docker build -t customers -f services/orders/Dockerfile services/customers
docker build -t auth -f services/auth/Dockerfile services/auth
