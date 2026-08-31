# Reglas que salieron de la base y ahora le tocan al dominio

La migración `2026-08-23-142321_create_waliki_schema` lleva **solo estructura**:
tablas, índices y constraints. Cero funciones, triggers, vistas, secuencias y
columnas generadas (decisión 20).

Lo que antes hacía Postgres y ahora hay que implementar y testear acá:

| # | Regla | Antes | Dónde va ahora |
|---|-------|-------|----------------|
| 1 | Generar `products.code` (`COD-001`, `COD-002`, …) | `sequence` + `default nextval` | value object `ProductCode` + el repositorio lo asigna al crear |
| 2 | `products.code` es inmutable | trigger `products_code_is_immutable` | el repositorio nunca incluye `code` en el `UPDATE` |
| 3 | Mantener `updated_at` | trigger `set_updated_at` en 7 tablas | el repositorio lo setea en cada `UPDATE` |
| 4 | `stock_entry_items.total_cost = quantity * unit_cost` | columna generada | el agregado lo calcula al construir la línea |
| 5 | `sale_items.total_amount = quantity * unit_amount` | columna generada | ídem |
| 6 | `cash_sessions.difference = counted - expected` | columna generada | el caso de uso de cierre de caja |
| 7 | Las líneas son de solo inserción | trigger `reject_line_mutation` en 4 tablas | el repositorio no expone `update` ni `delete` de líneas; para deshacer se anula la cabecera |
| 8 | Historial de precios | trigger `log_variant_price_change` | el caso de uso que cambia `suggested_price` inserta en `variant_price_changes` en la **misma transacción** |
| 9 | Detección de deriva del stock | vista `stock_check` | consulta del servicio, expuesta como health check |
| 10 | `default_variant_kind` ∈ `enabled_variant_kinds` | — (nunca existió) | invariante del agregado `Venture` |
| 11 | Forma de `variant_presets`: claves son kinds conocidos, valores son arrays no vacíos de strings no vacíos | función `variant_presets_valid` + CHECK (Supabase, `20260829_venture_variant_presets.sql`) | invariante del agregado `Venture`; la base solo garantiza `jsonb_typeof(variant_presets) = 'object'` |

## Detalle de las dos que tienen trampa

### 1. `products.code` sin secuencia

La secuencia daba unicidad global gratis. Ahora el código es **por
emprendimiento** (`unique (venture_id, code)`), así que el dominio tiene que
resolver la concurrencia: dos altas simultáneas en el mismo emprendimiento
pueden pedir el mismo número.

Opciones, de menos a más trabajo:

- Reintentar sobre la violación de unicidad. Simple y correcto; la colisión es
  rara y el reintento es barato.
- `select max(code) ... for update` sobre las filas del emprendimiento dentro
  de la transacción.
- Una tabla `venture_counters (venture_id, next_product_code)` con `for update`.

Recomiendo la primera hasta que haya evidencia de contención.

### 7. Líneas de solo inserción

Sin el trigger, la base ya no bloquea nada: un `UPDATE waliki.sale_items` pasa.
La garantía tiene que venir del tipo, no de la disciplina. Que el repositorio de
`Sale` no tenga ningún método que emita `UPDATE` o `DELETE` sobre `sale_items`
—que la única operación sea insertar la venta completa y anular la cabecera—
y un test que lo verifique.

### 10. El kind por defecto tiene que estar entre los habilitados

En el onboarding se elige talla por letra, por número, o **las dos**
(`enabled_variant_kinds`), y una de ellas queda como preselección
(`default_variant_kind`). Que la preselección esté dentro del conjunto es un
invariante, no una forma: la base no lo puede expresar con un CHECK simple, así
que lo hace el agregado.

```rust
impl Venture {
    fn set_variant_kinds(&mut self, enabled: HashSet<VariantKind>, default: VariantKind)
        -> Result<(), VentureError>
    {
        if enabled.is_empty() { return Err(VentureError::SinVariantKinds); }
        if !enabled.contains(&default) { return Err(VentureError::DefaultFueraDelConjunto); }
        // ...
    }
}
```

Lo que sí garantiza la base: el array no está vacío (`cardinality >= 1`, **no**
`array_length`, que para un array vacío devuelve `NULL` y dejaría pasar el
CHECK), no tiene `NULL` adentro, y todos los valores están en el conjunto de
`variant_kind`.

**Nota de Diesel:** `print-schema` genera `Array<Nullable<Text>>`, porque en
Postgres cualquier array puede contener `NULL`. Como el CHECK lo impide, se
puede ajustar a mano a `Array<Text>` en `schema.rs` y trabajar con
`Vec<String>` en vez de `Vec<Option<String>>`. La columna es `text[]` y no
`varchar[]` porque Diesel avisa explícitamente de problemas con los arrays de
varchar.

## Lo que la base SÍ sigue garantizando

- **FK** — lo referenciado existe
- **UNIQUE** — no hay duplicados (incluidos los índices únicos parciales, como
  una sola caja abierta por miembro o una sola ubicación por defecto)
- **CHECK** — el valor está en el conjunto (`variant_kind`, `role`,
  `payment_method`, …) o en el rango (`quantity > 0`, `price >= 0`)
- **NOT NULL** — el dato está
- **índices** — las consultas del dominio no escanean tablas

Los conjuntos cerrados son `varchar` + CHECK, no enums de Postgres
(decisión 18): la verdad vive en el enum de Rust y Diesel los lee como `String`
con un `TryFrom` en el dominio.

### 11. `variant_presets` sin función de validación

Supabase resuelve la forma de `variant_presets` con una función SQL marcada
`immutable` porque un CHECK no admite subconsultas y validar un jsonb necesita
recorrerlo con `jsonb_each`. Acá esa función no puede existir (decisión 20:
cero funciones en la base), así que el CHECK se queda corto a propósito —
solo `jsonb_typeof(variant_presets) = 'object'` — y el agregado `Venture`
hace el resto al construir o actualizar los presets: que cada clave sea un
`VariantKind` conocido y cada valor un `Vec<String>` no vacío de strings no
vacíos.
