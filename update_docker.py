import os
import subprocess

if os.environ.get("GITHUB_ACTIONS") == "true":
    print("`GitHub Actions Server` detected, skip `Building Docker image`...")
    exit()

user_name = "edenwang33773"
image_name = "learning-rvemu-for-book-env"

# Run docker login
subprocess.run(["docker", "login"], check=True)

# Build the Docker image
subprocess.run(["docker", "build", "-t", image_name, "."], check=True)

# Tag the Docker image
subprocess.run(["docker", "tag", image_name, f"{user_name}/{image_name}"], check=True)

# Push the Docker image
subprocess.run(["docker", "push", f"{user_name}/{image_name}"], check=True)
