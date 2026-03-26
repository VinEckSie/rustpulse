# RustPulse Deployment Runbook (Dev / Staging / Prod)

This project is set up to use the same **deployment model** in staging and production:

- Linux VM
- Docker containers
- Docker Compose orchestration
- systemd manages the Compose stack
- Environment-specific behavior is controlled **only** by injected environment variables (via server-side env files)

The backend also supports a local dev flow using `.env`.

---

## 1) Environment Variables (What / Where)

### Local development
- File: `.env` (not committed)
- Template: `.env.example`
- `.env` is loaded only for local convenience (when `APP_ENV` is missing, or when `APP_ENV=dev`).

### Staging (VM)
- File on server: `/opt/rustpulse/env/rustpulse.staging.env`
- Template in repo: `deploy/env/rustpulse.staging.env.example`

### Production (VM)
- File on server: `/opt/rustpulse/env/rustpulse.prod.env`
- Template in repo: `deploy/env/rustpulse.prod.env.example`

### Prod fail-fast rules
If `APP_ENV=prod`, the backend exits with an error if any of these are missing:
- `PORT`
- `DATABASE_URL`
- `JWT_SECRET`

`JWT_SECRET` must be at least 32 characters in prod.

---

## 2) Docker Compose Files (How They Work)

### Local dev compose
- File: `compose.yaml`
- Runs `rustpulse` + `postgres`
- Uses `env_file: .env` to inject variables into the container.

### Staging / prod compose
- Files: `compose.staging.yaml`, `compose.prod.yaml`
- These files use `environment: ${VAR}` entries.
  - Compose reads `${VAR}` from the *environment of the `docker compose` process*.
  - systemd provides those vars via `EnvironmentFile=...` in the unit.

#### Example: `compose.prod.yaml` (what to notice)
- `services.rustpulse.image`: points to an already-built image (typically pushed to GHCR).
- `services.rustpulse.environment`: passes runtime config into the container.
- `services.postgres.environment`: configures the postgres container using the same env file.
- `depends_on`: ensures container start ordering (not full readiness).
- `restart: unless-stopped`: standard service resilience behavior.
- `volumes`: persists postgres data between restarts.

---

## 3) systemd Units (Why / Syntax)

Files:
- Staging: `deploy/systemd/rustpulse-staging.service`
- Production: `deploy/systemd/rustpulse.service`

Key directives used:
- `[Unit]`
  - `After=` / `Requires=`: makes sure Docker is available before starting.
- `[Service]`
  - `Type=oneshot` + `RemainAfterExit=yes`: systemd treats “compose up -d” as the start action and considers the unit active afterward.
  - `WorkingDirectory=/opt/rustpulse`: where Compose files live on the VM.
  - `EnvironmentFile=/opt/rustpulse/env/...`: loads environment variables into the `docker compose` process.
  - `ExecStart=`: `docker compose ... up -d`
  - `ExecStop=`: `docker compose ... down`
- `[Install]`
  - `WantedBy=multi-user.target`: enables starting at boot.

---

## 4) Testing: Dev (Local Machine)

### A) Persistence setup
For Postgres vs JSONL setup and how to verify DB writes locally, follow:
- `docs/persistence.md`

### B) Run the backend
1. Create your local env file:
   - `cp .env.example .env`
2. Start the server:
   - `cargo run --bin rustpulse`
3. Verify:
   - `curl -s http://127.0.0.1:3000/health | cat`

Notes:
- In dev, `PORT` defaults to `3000` only if missing.
- Logs print to stdout.

---

## 5) Testing: Staging (Linux VM)

### A) Prepare the VM
Install:
- Docker Engine
- Docker Compose v2

Create folders:
- `/opt/rustpulse`
- `/opt/rustpulse/env`

Copy files to the VM:
- `compose.staging.yaml` → `/opt/rustpulse/compose.staging.yaml`
- `deploy/systemd/rustpulse-staging.service` → `/etc/systemd/system/rustpulse-staging.service`
- Create `/opt/rustpulse/env/rustpulse.staging.env` from `deploy/env/rustpulse.staging.env.example`

### B) Choose the image
In `compose.staging.yaml`, set:
- `image: ghcr.io/<github-user-or-org>/rustpulse:staging`

You must replace `<github-user-or-org>` with your actual GitHub username or org that owns the GHCR repo.

### C) Start via systemd
1. `sudo systemctl daemon-reload`
2. `sudo systemctl enable --now rustpulse-staging`
3. Check logs:
   - `sudo journalctl -u rustpulse-staging -f`
4. Verify:
   - `curl -s http://<vm-ip>:8080/health | cat`

---

## 6) Testing: Production (Linux VM)

Same steps as staging, but using:
- `/opt/rustpulse/compose.prod.yaml`
- `/opt/rustpulse/env/rustpulse.prod.env`
- `/etc/systemd/system/rustpulse.service`

Commands:
1. `sudo systemctl daemon-reload`
2. `sudo systemctl enable --now rustpulse`
3. `sudo journalctl -u rustpulse -f`

---

## 7) What to Commit to GitHub (Best Practice)

Commit:
- `.env.example`
- `deploy/env/*.env.example`
- `compose*.yaml`
- `deploy/systemd/*.service`

Do NOT commit:
- `.env`
- `/opt/rustpulse/env/*.env` (real secrets)

Keep real secrets in server-side env files, a secret manager, or CI/CD secrets.

---

## 8) Automated Staging Deploy with GitHub Actions (Recommended)

This repo includes a staging deploy workflow:
- `.github/workflows/deploy-staging.yml`

It does:
- Build + push `ghcr.io/vinecksie/rustpulse:staging`
- SSH to your staging VM
- Write `/opt/rustpulse/env/rustpulse.staging.env` from a GitHub secret
- Install/refresh `compose.staging.yaml` and the systemd unit
- Restart the `rustpulse-staging` service

### A) Create a staging environment in GitHub
1. GitHub repo → Settings → Environments → New environment: `staging`
2. Add secrets to the `staging` environment:
   - `STAGING_SSH_HOST` (VM IP or DNS)
   - `STAGING_SSH_USER` (e.g. `ubuntu`)
   - `STAGING_SSH_KEY` (private SSH key with access to the VM)
   - `STAGING_SSH_PORT` (optional, defaults to `22`)
   - `RUSTPULSE_STAGING_ENV_B64` (base64 of the full env file contents)

### B) Build the env file secret (base64)
Create a file locally (do not commit it), for example `rustpulse.staging.env`, using `deploy/env/rustpulse.staging.env.example` as a template.

Then base64 encode it:
- `base64 -i rustpulse.staging.env | tr -d '\\n'`

Paste that output into the GitHub secret `RUSTPULSE_STAGING_ENV_B64`.

### C) VM prerequisites (one-time)
On the staging VM:
- Install Docker Engine + Docker Compose v2
- Ensure the SSH user can run docker and sudo the systemd actions
- Open port `8080` (or your chosen `PORT`) in the firewall / security group

### D) Trigger a deploy
Option 1: push to `main` (workflow auto-runs).

Option 2: run manually:
- GitHub → Actions → `deploy-staging` → “Run workflow”

### E) Verify on the VM
- `sudo journalctl -u rustpulse-staging -f`
- `curl -s http://<vm-ip>:8080/health | cat`
