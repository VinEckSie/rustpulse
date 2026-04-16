# Staging Environment Checklist

Use this checklist to verify the VM is ready to receive deployments.

Run commands in order.

---

## SSH connectivity

From local machine:

ssh -i ~/.ssh/rustpulse_staging_ed25519 deploy@SERVER_IP

Expected:

connection works without password

Check user:

whoami

Expected:

deploy

---

## sudo permissions

sudo whoami

Expected:

root

---

## docker installed

docker --version

Expected:

Docker version ...

---

## docker service running

systemctl status docker

Expected:

active (running)

---

## docker usable without sudo

docker ps

Expected:

no permission error

If error:

sudo usermod -aG docker deploy

Then reconnect SSH.

---

## docker compose available

docker compose version

Expected:

Docker Compose version v2.x

---

## deployment directories exist

ls -la /opt/rustpulse

Expected:

directory exists

---

## env directory exists

ls -la /opt/rustpulse/env

Expected:

directory exists

---

## systemd available

systemctl --version

Expected:

systemd version ...

---

## container runtime works

docker run hello-world

Expected:

Hello from Docker

---

## ready for deployment

If all checks pass:

environment ready for CI/CD deploy
