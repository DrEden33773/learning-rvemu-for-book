import subprocess

# Vars
USER_NAME = "edenwang33773"
IMAGE_NAME = "learning-rvemu-for-book-env"

# Run docker login
subprocess.run(["docker", "login"])

# Build the Docker image
subprocess.run(["docker", "build", "-t", IMAGE_NAME, "."])

# Tag the Docker image
subprocess.run(
    [
        "docker",
        "tag",
        IMAGE_NAME,
        f"{USER_NAME}/{IMAGE_NAME}",
    ]
)

# Push the Docker image
subprocess.run(["docker", "push", f"{USER_NAME}/{IMAGE_NAME}"])

# Clean
subprocess.run(["docker", "builder", "prune", "-a", "-f"])
