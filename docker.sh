eval $(minikube docker-env)
docker build -t warehouse-service:2 -f services/warehouse/Dockerfile.main services/warehouse
docker build -t warehouse-migrations:2 -f services/warehouse/Dockerfile.migration services/warehouse
docker build -t orders-service:2 -f services/orders/Dockerfile.main services/orders
docker build -t orders-migrations:2 -f services/orders/Dockerfile.migrations services/orders
docker build -t customer-service:2 -f services/customers/Dockerfile.main services/customers
docker build -t customer-migrations:2 -f services/customers/Dockerfile.migrations services/customers
docker build -t auth-service:2 -f services/auth/Dockerfile services/auth
docker build -t auth-migrations:2 -f services/auth/migrations/Dockerfile services/auth/migrations
docker build -t shipments-service:4 -f services/shipments/Dockerfile services/shipments
docker build -t shipment-migrations:4 -f services/shipments/Dockerfile.migrations services/shipments
docker build -t client:1 -f client/Dockerfile client
