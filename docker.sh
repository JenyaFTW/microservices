eval $(minikube docker-env)
docker build -t warehouse -f services/warehouse/Dockerfile .