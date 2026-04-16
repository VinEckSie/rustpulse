# RustPulse New Environment Deployment Runbook

This document explains how to deploy **RustPulse** to a new environment (staging or production) from zero.

Goal:any engineer should be able to reproduce the deployment without additional explanations.

# Architecture
Target stack:
- Linux VM (Ubuntu 22.04 recommended)
- Docker containers
- Docker Compose orchestration
- systemd manages lifecycle
- GitHub Actions performs CI/CD
- GHCR stores container images

Deployment flow:
Deployment flow:
1. Staging VM prepared with Docker and systemd
2. Push to main or manual trigger starts deploy-staging workflow
3. GitHub Actions builds the RustPulse container image
4. Image pushed to GHCR
5. GitHub Actions connects to VM using SSH private key
6. Compose file copied to /opt/rustpulse
7. systemd service file installed
8. Environment file written to /opt/rustpulse/env/rustpulse.staging.env
9. systemd restarts rustpulse-staging service
10. Docker pulls new image and starts postgres + rustpulse containers
11. Health endpoint /health is called
12. API available at http://204.168.188.43:8080/health

# 1. One-time VM bootstrap
Provision a Linux VM.
Recommended OS:Ubuntu 22.04 LTSConnect via SSH as root or admin user.

## 1.1 Create deploy user
```bash
adduser deployusermod -aG sudo deploy
```

Reconnect using the deploy user:
```bash
ssh deploy@SERVER_IP
```

## 1.2 Install Docker
```bash
sudo apt update
sudo apt install -y ca-certificates curl gnupg

sudo install -m 0755 -d /etc/apt/keyrings

curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

sudo chmod a+r /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo $VERSION_CODENAME) stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt update

sudo apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

Allow deploy user to run docker:
```bash
sudo usermod -aG docker deploy
```

Reconnect SSH session:
```bash
exit ssh deploy@SERVER_IP
```

Verify Docker:
```bash
docker ps
docker compose version
docker run hello-world
```

## 1.3 Create directories
```bash
sudo mkdir -p /opt/rustpulse/env
sudo mkdir -p /opt/rustpulse/compose
sudo chown -R deploy:deploy /opt/rustpulse
```

## 1.4 Install systemd service
Locally > Copy service file to server:
```bash
scp deploy/systemd/rustpulse-staging.service deploy@SERVER_IP:/tmp/
````

Install service:
```bash
sudo mv /tmp/rustpulse-staging.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable rustpulse-staging
````

Verify service registration:
```bash
systemctl status rustpulse-staging
```

Service may fail until first deployment. This is normal.

## 2. Environment configuration
Locally > Create env file locally using template: deploy/env/rustpulse.staging.env.example

Example:
```text
APP_ENV=staging
HOST=0.0.0.0
PORT=8080
RUSTPULSE_STORAGE=postgres
DATABASE_URL=postgres://rustpulse:password@postgres:5432/rustpulse
JWT_SECRET=replace-with-long-random-secret
POSTGRES_DB=rustpulse
POSTGRES_USER=rustpulse
POSTGRES_PASSWORD=password
LOG_JSON=1
RUST_LOG=info
```

Locally > Encode env file:
```bash
base64 -i rustpulse.staging.env | tr -d '\n'
```
Save output in the next step with the following secret name:
```bash
RUSTPULSE_STAGING_ENV_B64
```

About 'STAGING_SSH_KEY' in the next step:
- generate with:
```bash
ssh-keygen -t ed25519 -C "rustpulse-staging" -f ~/.ssh/rustpulse_staging_ed25519
```
- display with:
```bash
cat ~/.ssh/rustpulse_staging_ed25519
```

## 3. GitHub configuration
Create GitHub Environment:
- staging
Add the following secrets:
- STAGING_SSH_HOST
- STAGING_SSH_PORT
- STAGING_SSH_USER
- STAGING_SSH_KEY
- STAGING_APP_HOST
- STAGING_APP_PORT

- RUSTPULSE_STAGING_ENV_B64

## 4. Required repository files
The repository must contain:
- Compose file: compose.staging.yaml
- systemd unit: deploy/systemd/rustpulse-staging.service
- CI workflow: .github/workflows/deploy-staging.yml

## 5. First deployment
Trigger deployment:
- git push origin main

Or manually:
- GitHub → Actions → deploy-staging → Run workflow

## 6. Verify after first deployment
SSH into server:
```bash
ssh root@204.168.188.43
````

Check files exist:
```bash
ls -l /opt/rustpulse
````

should show:
```text
compose.staging.yaml
env/
````

Check env file:
```bash
sudo ls -l /opt/rustpulse/env
````

Check systemd service:
```bash
sudo systemctl status rustpulse-staging
````

Expected:
```text
active (exited)
````

Check containers:
```bash
docker ps
````

Expected containers:
```text
rustpulse-staging
rustpulse-postgres-staging
```

Test API:
```bash
curl http://204.168.188.43:8080/healthz
````

## 6. Verify deployment
Check systemd status:
```bash
systemctl status rustpulse-staging
```

Check logs:
```bash
journalctl -u rustpulse-staging -n 100 --no-pager
````

Verify container:
```bash
docker ps
````

Check health endpoint:
```bash
curl http://SERVER_IP:8080/health
````

Expected response:
200 OK

## 7. Routine updates
Each push to main:
- builds new container image
- pushes image to GHCR
- deploys automatically
- restarts service
- verifies health endpoint


Image format: ghcr.io/vinecksie/rustpulse:staging-<commit_sha>

## 8. Rollback procedure
List images:
```bash
docker images | grep rustpulse
````

Update compose file with previous tag: ghcr.io/vinecksie/rustpulse:staging-previous_sha

Restart service:
```bash
sudo systemctl restart rustpulse-staging
````

Verify health endpoint again.

## 9. Production deployment
Same process as staging.

Differences:
- env file: /opt/rustpulse/env/rustpulse.prod.env
- systemd service: rustpulse.service
- GitHub environment: production
- GitHub secret: RUSTPULSE_PROD_ENV_B64

Production VM should be separate from staging VM.

## 10. Troubleshooting checklist
Check service:
```bash
systemctl status rustpulse-staging
````

Check logs:
```bash
journalctl -u rustpulse-staging -f
````

Check containers:
```bash
docker ps
````

Check compose logs:
```bash
docker compose logs
````

Check network:
```bash
curl http://SERVER_IP:8080/health
````

Verify env file exists:
```bash
cat /opt/rustpulse/env/rustpulse.staging.env
```

Verify Docker can pull image:
```bash
docker pull ghcr.io/vinecksie/rustpulse:staging-latest
```
