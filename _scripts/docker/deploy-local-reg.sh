#!/usr/bin/env bash
eval $(minikube docker-env)

echo "Building the image..."
sh ./_scripts/build.sh

echo "Image built so lets tag it..."

docker tag rustic-users localhost:5000/rustic-user:latest

echo "Image is tagged so push it to registry ..."
docker push localhost:5000/rustic-user

echo "Image has been pushed to the local registry"
