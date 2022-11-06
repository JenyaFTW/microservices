eval $(minikube docker-env)
docker build -t warehouse:1 -f services/warehouse/Dockerfile services/warehouse
docker build -t orders:1 -f services/orders/Dockerfile services/orders
docker build -t customers:1 -f services/customers/Dockerfile services/customers
docker build -t auth-service:2 -f services/auth/Dockerfile services/auth
docker build -t auth-migrations:2 -f services/auth/migrations/Dockerfile services/auth/migrations
docker build -t client:1 -f client/Dockerfile client