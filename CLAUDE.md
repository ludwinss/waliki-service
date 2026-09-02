# waliki-service-rust

## Código vivo vs. código muerto

- **Código vivo: solo `crates/`.** El workspace real vive en `crates/api`, `crates/platform` y
  (en construcción) `crates/context/{identity,catalog,operations}`.
- **`src/` en la raíz es código muerto.** Es la implementación legacy basada en actix
  (OIDC/Google, user context, adapters http). No compila como parte del trabajo actual,
  no se mantiene y se eliminará por completo en algún momento.
  - No leerlo, no referenciarlo, no "arreglarlo" ni proponer cambios sobre él.
  - Si algo de ahí hace falta, se re-implementa dentro de `crates/`, no se rescata el archivo.

## Bounded contexts (decisión fijada — issue #34)

Tres contextos, no seis. La frontera es de lenguaje y de equipo, no un grupo de tablas.

| Context      | Contenido                                                        |
|--------------|-----------------------------------------------------------------|
| `identity`   | User/Profile, Venture, Member, Invitation                       |
| `catalog`    | Product, Variant, historial de precios                          |
| `operations` | stock, ventas y caja (juntos a propósito)                       |

- **stock + ventas van juntos** para que la invariante "no sobreventa" se resuelva en
  **una sola transacción**, no con consistencia eventual.
- **Milestones (M4/M5/M6) siguen separados.** Milestone = incremento de entrega;
  context = frontera de código. No se fusionan.
- **Los agregados se referencian por id, nunca por objeto.** `Sale` guarda un `VariantId`,
  no un `Variant`. Regla desde el día uno.

Cada context es un crate bajo `crates/context/` y se registra en `members` del workspace.
