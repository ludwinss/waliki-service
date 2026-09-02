# waliki-service

Backend de waliki en Rust. Workspace Cargo organizado por **bounded contexts**
(DDD) con arquitectura por capas (Clean).

## Estructura

```
crates/
├── api/          (bin)  composition root: HTTP server, config, wiring de dependencias
├── platform/     (lib)  infra transversal: pool PostgreSQL, config, ActivityLogger
└── context/            bounded contexts (decisión: issue #34)
    ├── shared/    (lib)  shared kernel de dominio (Id<T>, DomainError, Clock, IdGenerator)
    ├── identity/  (lib)  Profile, Venture, Member, Invitation
    ├── catalog/   (lib)  Product, Variant, historial de precios, Supplier
    └── operations/(lib)  stock, ventas y caja

migrations/       migraciones diesel (esquema `waliki`)
src/              ⚠️ código legacy muerto (actix/OIDC) — no se mantiene, se eliminará
```

### Reglas de arquitectura

- **Cada context es un `lib`**; el único binario es `api`.
- **Stock y ventas viven juntos** en `operations`: la invariante "no sobreventa"
  se resuelve en una sola transacción.
- **Los agregados se referencian por id, nunca por objeto** (`Sale` guarda
  `Id<Variant>`, no `Variant`). Un context no importa tipos de otro.
- Dirección de dependencias dentro de un context:
  `infrastructure → application → domain`. `domain` solo depende de `shared`.
- **Milestones ≠ contexts**: un milestone (M4/M5/M6) es un incremento de entrega;
  un context es una frontera de código.

## Desarrollo

```sh
cargo build --workspace        # compila todo
cargo test  --workspace        # tests
cargo clippy --workspace       # lints
diesel migration run           # aplica migraciones (requiere DATABASE_URL)
```

La configuración se toma de variables de entorno (ver `crates/platform/src/config.rs`);
`.env` se carga automáticamente en desarrollo.
