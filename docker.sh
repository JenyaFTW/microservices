eval $(minikube docker-env)
docker build -t warehouse:2 -f services/warehouse/Dockerfile.main services/warehouse
docker build -t warehouse-migrations:2 -f services/warehouse/Dockerfile.migration services/warehouse
docker build -t orders:2 -f services/orders/Dockerfile.main services/orders
docker build -t orders-migrations:2 -f services/orders/Dockerfile.migrations services/orders
docker build -t customers:1 -f services/customers/Dockerfile services/customers
docker build -t auth-service:2 -f services/auth/Dockerfile services/auth
docker build -t auth-migrations:2 -f services/auth/migrations/Dockerfile services/auth/migrations
docker build -t client:1 -f client/Drockerfile client