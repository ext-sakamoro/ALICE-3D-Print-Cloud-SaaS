# ALICE-3D-Print-Cloud

SDF-to-G-code 3D printing cloud with physics validation

## License

AGPL-3.0

## Architecture

```
Frontend :3000  -->  API Gateway :8080  -->  Core Engine :8081
```

| Layer | Port | Technology |
|-------|------|-----------|
| Frontend | 3000 | Next.js 14, Tailwind CSS |
| API Gateway | 8080 | Rust, Axum |
| Core Engine | 8081 | Rust, Axum |

## ALICE Crate Integration

alice-sdf, alice-print, alice-physics, alice-view

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/v1/upload` | SDFモデルアップロード |
| `POST` | `/api/v1/slice` | スライス＆G-code生成 |
| `POST` | `/api/v1/validate` | 物理強度バリデーション |
| `GET ` | `/api/v1/jobs` | プリントジョブ一覧 |
| `GET ` | `/health` | ヘルスチェック |

## Quick Start

```bash
cd services/core-engine
cargo run --release
curl http://localhost:8081/health
```

## Author

Moroya Sakamoto
