# ALICE-3D-Print-Cloud

SDF-to-G-code 3D printing cloud with real-time preview and physics validation

## License

AGPL-3.0

## Pipeline

```
SDF Model  →  ALICE-SDF (evaluate)
     │
     ▼
ALICE-View (3D preview / cross-section visualization)
     │
     ▼
ALICE-Physics (structural stress analysis / validation)
     │
     ▼
ALICE-Print (slice → G-code generation)
     │
     ▼
G-code output → 3D Printer
```

## Architecture

```
Frontend :3000  -->  API Gateway :8080  -->  Core Engine :8081
   │                                           │
   └── 3Dプレビュー (WebGL)         ALICE-SDF ──┤
                                    ALICE-View ──┤
                                    ALICE-Physics─┤
                                    ALICE-Print ──┘
```

| Layer | Port | Technology |
|-------|------|-----------|
| Frontend | 3000 | Next.js 14, Tailwind CSS, WebGL |
| API Gateway | 8080 | Rust, Axum |
| Core Engine | 8081 | Rust, Axum |

## ALICE Crate Integration

| Crate | Role |
|-------|------|
| `alice-sdf` | SDF式評価、CSGブーリアン、メッシュ変換 |
| `alice-view` | リアルタイム3Dプレビュー、断面表示 |
| `alice-physics` | 構造応力解析、強度バリデーション |
| `alice-print` | スライス、G-code生成、プリンター制御 |

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/v1/upload` | SDFモデルアップロード |
| `POST` | `/api/v1/preview` | 3Dプレビュー生成（View経由） |
| `POST` | `/api/v1/validate` | 物理強度バリデーション |
| `POST` | `/api/v1/slice` | スライス＆G-code生成 |
| `GET`  | `/api/v1/jobs` | プリントジョブ一覧 |
| `GET`  | `/health` | ヘルスチェック |

## Quick Start

```bash
cd services/core-engine
cargo run --release
curl http://localhost:8081/health
```

## Author

Moroya Sakamoto
