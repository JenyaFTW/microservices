eval $(minikube docker-env)
docker build -t auth -f services/auth/Dockerfile .
