## 🧱 Week 2 – Domain Modeling & SQLx Setup

**🎯 Goal:** Define domain entities and integrate async PostgreSQL connection using `sqlx`.

### ✅ Features Implemented
- Defined core domain models: `Server`, `Metric` (with `Uuid`, `IpAddr`, `DateTime`, `Duration`)
- Integrated `dotenvy` for loading environment variables from `.env`
- Parsed `DATABASE_URL` and connected via `PgPoolOptions`
- Configured structured logging with `tracing_subscriber` and optional JSON format
- Explored `.env` runtime loading behavior based on workspace vs. crate path

### 🧰 Tooling
| Tool        | Purpose                              |
|-------------|--------------------------------------|
| sqlx        | Async DB access with compile-time safety |
| uuid        | Unique identifiers for models        |
| chrono      | Timestamps and durations             |
| dotenvy     | Load environment variables            |
| tracing     | Structured logs                      |
| tracing-subscriber | Log output and formatting     |

### 🚧 In Progress
- Database creation (manual or migration)
- Initial schema and migration setup using `sqlx-cli`
- Injecting DB pool into `AppState` for shared access

### 📌 Notes
- PostgreSQL must be manually created before connecting
- Connection `.await` should use `?` and proper error handling (`main() -> Result`)
- Running from the crate root (`cd bac

