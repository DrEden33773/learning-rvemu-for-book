# TODO

## GitHub Actions Improvements

- [ ] Dynamic update docker image -> run on latest image

您可以使用 Docker Hub 或其他容器注册表来存储您的 Docker 镜像，并使用 GitHub Actions Workflow 来检查您的 Docker 镜像是否需要更新。以下是一个可能的工作流程示例：

```yml
name: Rust with LLVM via Docker

on:
  push:
    branches: ["master"]
  pull_request:
    branches: ["master"]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Check if image needs to be updated
        id: check-image
        run: |
          IMAGE_NAME=my-rust-app
          IMAGE_TAG=latest
          DOCKER_REGISTRY=my-registry.example.com
          DOCKER_USERNAME=$DOCKER_USERNAME
          DOCKER_PASSWORD=$DOCKER_PASSWORD
          IMAGE_ID=$(docker inspect --format='{{.Id}}' $DOCKER_REGISTRY/$DOCKER_USERNAME/$IMAGE_NAME:$IMAGE_TAG || true)
          if [ -z "$IMAGE_ID" ]; then
            echo "Image not found in registry"
            exit 1
          fi
          LOCAL_IMAGE_ID=$(docker inspect --format='{{.Id}}' $IMAGE_NAME:$IMAGE_TAG || true)
          if [ "$IMAGE_ID" != "$LOCAL_IMAGE_ID" ]; then
            echo "Image needs to be updated"
            echo $DOCKER_PASSWORD | docker login -u $DOCKER_USERNAME --password-stdin $DOCKER_REGISTRY
            docker tag $IMAGE_NAME:$IMAGE_TAG $DOCKER_REGISTRY/$DOCKER_USERNAME/$IMAGE_NAME:$IMAGE_TAG
            docker push $DOCKER_REGISTRY/$DOCKER_USERNAME/$IMAGE_NAME:$IMAGE_TAG
          else
            echo "Image is up to date"
          fi
      - name: Run image
        if: steps.check-image.outcome == 'success'
        run: |
          IMAGE_NAME=my-rust-app
          IMAGE_TAG=latest
          DOCKER_REGISTRY=my-registry.example.com
          DOCKER_USERNAME=$DOCKER_USERNAME
          DOCKER_PASSWORD=$DOCKER_PASSWORD
          echo $DOCKER_PASSWORD | docker login -u $DOCKER_USERNAME --password-stdin $DOCKER_REGISTRY
          docker run $DOCKER_REGISTRY/$DOCKER_USERNAME/$IMAGE_NAME:$IMAGE_TAG
```

在这个工作流程中，我们首先检查 Docker 镜像是否需要更新。如果需要更新，我们会将本地 Docker 镜像打上标签并推送到 Docker 注册表。如果不需要更新，我们将跳过构建步骤并直接运行 Docker 镜像。

请注意，此示例假定您已经设置了 Docker 注册表和 GitHub Actions 的机密。您需要将 DOCKER_REGISTRY，DOCKER_USERNAME 和 DOCKER_PASSWORD 替换为您自己的值。

- [ ] Upload [Dockerfile](./Dockerfile) to `Docker Hub` for previous improvement

To set up a container registry to store your Docker images, you can use a service like Docker Hub, Google Container Registry, or Amazon Elastic Container Registry. Here are the general steps to set up Docker Hub:

Create a Docker Hub account at <https://hub.docker.com/signup>.
Log in to your Docker Hub account.
Create a new repository by clicking the "Create Repository" button on the Docker Hub dashboard.
Choose a name for your repository and select whether it will be public or private.
Build your Docker image using the docker build command.
Tag your Docker image with the name of your repository and the version number using the docker tag command.
Push your Docker image to Docker Hub using the docker push command.
Here's an example of how to push a Docker image to Docker Hub:

```bash
# Build the Docker image
docker build -t my-repo/my-image:1.0 .

# Tag the Docker image
docker tag my-repo/my-image:1.0 my-docker-username/my-image:1.0

# Log in to Docker Hub
docker login

# Push the Docker image to Docker Hub
docker push my-docker-username/my-image:1.0
```

Note that you'll need to replace my-repo, my-image, 1.0, and my-docker-username with your own values.
