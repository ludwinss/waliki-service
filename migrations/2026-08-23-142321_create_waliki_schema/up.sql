-- ============================================================
-- Waliki - M1: esquema base del backend en Rust.
--
-- Crea el esquema `waliki` DESDE CERO y autocontenido: no depende
-- de `public` ni de las migraciones de Supabase. Las tablas viejas
-- de `public` quedan intactas para que la app movil siga andando
-- hasta que el servicio este listo.
--
-- REGLA DE ESTA MIGRACION
--   Aca va SOLO estructura: tablas, indices y constraints. Nada mas.
--   Cero funciones, cero triggers, cero vistas, cero secuencias, cero
--   columnas calculadas.
--
--   Todo lo que sea comportamiento vive en el repo de Rust, no en la
--   base. Un trigger es codigo escondido donde nadie lo revisa, no se
--   testea con el resto y no aparece en el diff de un PR.
--
--   Lo que la base SI hace es garantizar la forma de los datos:
--     * FK             -> lo referenciado existe
--     * UNIQUE         -> no hay duplicados
--     * CHECK          -> el valor esta dentro del conjunto o del rango
--     * NOT NULL       -> el dato esta
--     * indices        -> las consultas del dominio no escanean tablas
--
--   Y lo que la base NO hace, y por lo tanto le toca al dominio:
--     * generar el codigo de producto (ProductCode)
--     * mantener `updated_at`
--     * impedir que se editen las lineas ya registradas
--     * calcular totales, `avg_cost` y la diferencia de caja
--     * registrar el historial de precios
--     * aplicar y revertir el stock
--
-- CONVENCIONES
--   * Los conjuntos cerrados van como `varchar` + CHECK con nombre,
--     NO como tipos enum de Postgres. Motivos:
--       - la verdad del conjunto vive en el enum de Rust (decision 16);
--         el CHECK es un guardia, no la definicion.
--       - migrar es `drop constraint` + `add constraint` dentro de una
--         transaccion. Un enum de Postgres no deja usar un valor nuevo
--         en la misma transaccion, y no deja borrar ni renombrar valores.
--       - Diesel lo lee como String y el dominio hace TryFrom, sin
--         diesel-derive-enum ni mapeo de OIDs por esquema.
--
--   * Los actores apuntan SIEMPRE a members, nunca a profiles.
-- ============================================================

create schema if not exists waliki;

create extension if not exists pgcrypto;

-- ============================================================
-- IDENTIDAD  (decision 14: RESUELTA)
--
-- El servicio en Rust absorbe el login, asi que la identidad vive
-- aca dentro y no en un esquema `user` aparte. Se replica lo que en
-- la app movil daba Supabase: `profiles` era la persona y `auth.users`
-- guardaba el correo y el vinculo con el proveedor externo.
--
--   profiles           -> la persona, una sola vez en todo el sistema.
--   profile_identities -> por que proveedor entra esa persona.
--
-- Van separadas para que la misma persona pueda entrar manana por
-- Google y por otro proveedor sin duplicarse. Todo actor del dominio
-- apunta a members, nunca aca.
-- ============================================================

create table waliki.profiles (
  id           uuid        not null default gen_random_uuid(),

  -- Correo canonico de la persona. La unicidad es sobre la forma
  -- normalizada, por eso el indice y no un UNIQUE de columna.
  email        varchar     not null check (length(btrim(email)) > 0),

  display_name varchar     not null check (length(btrim(display_name)) > 0),
  avatar_url   varchar,

  -- Deshabilitar a una persona la saca de TODOS los emprendimientos.
  -- Para sacarla de uno solo esta members.active.
  status       varchar     not null default 'active',

  created_at   timestamptz not null default now(),
  updated_at   timestamptz not null default now(),

  constraint profiles_pkey primary key (id),
  constraint profiles_status_check check (status in ('active', 'disabled'))
);

create unique index profiles_email_key
  on waliki.profiles (lower(btrim(email)));

-- Vinculo con el proveedor externo. `subject` es el `sub` del token
-- OIDC: opaco, estable y unico dentro del proveedor.
create table waliki.profile_identities (
  id            uuid        not null default gen_random_uuid(),
  profile_id    uuid        not null,
  provider      varchar     not null,
  subject       varchar     not null check (length(btrim(subject)) > 0),
  last_login_at timestamptz,
  created_at    timestamptz not null default now(),

  constraint profile_identities_pkey primary key (id),
  constraint profile_identities_unique unique (provider, subject),
  constraint profile_identities_profile_fkey foreign key (profile_id)
    references waliki.profiles (id) on delete cascade,
  constraint profile_identities_provider_check
    check (provider in ('google'))
);

create index profile_identities_profile_idx
  on waliki.profile_identities (profile_id);

-- ============================================================
-- EMPRENDIMIENTOS
-- ============================================================

create table waliki.ventures (
  id                       uuid        not null default gen_random_uuid(),
  code                     varchar     not null,
  name                     varchar     not null check (length(btrim(name)) > 0),
  active                   boolean     not null default true,

  setup_status             varchar     not null default 'pending',
  setup_completed_at       timestamptz,

  -- Con que arranca cada producto nuevo: la PRESELECCION del formulario.
  -- NO es una propiedad del negocio: una misma tienda vende zapatillas
  -- por numero y poleras por letra, por eso el kind de verdad vive en
  -- products.
  default_variant_kind     varchar     not null default 'size_letter',

  -- Que kinds le OFRECE el formulario, elegidos en el onboarding. El
  -- usuario puede marcar talla por letra, por numero, o las dos: por eso
  -- es un conjunto y no un solo valor.
  --
  -- Va como array y no como tabla aparte porque no es una entidad: son
  -- dos o tres strings que siempre se leen junto con el emprendimiento y
  -- nunca se consultan solos.
  --
  -- Que el default este DENTRO de este conjunto lo garantiza el agregado
  -- Venture en Rust, no la base: es un invariante, no una forma.
  -- text[] y no varchar[]: Diesel avisa que los arrays de varchar dan
  -- problemas de mapeo. Para el resto de las columnas varchar esta bien.
  enabled_variant_kinds    text[]      not null default '{size_letter}',

  logo_seed                varchar,
  logo_url                 varchar,

  -- Si es true, vender con on_hand insuficiente falla.
  block_sale_without_stock boolean     not null default false,

  created_at               timestamptz not null default now(),
  updated_at               timestamptz not null default now(),

  constraint ventures_pkey primary key (id),
  constraint ventures_code_key unique (code),
  constraint ventures_setup_status_check
    check (setup_status in ('pending', 'completed')),
  constraint ventures_default_variant_kind_check
    check (default_variant_kind in (
      'size_letter', 'size_number', 'volume',
      'color', 'presentation', 'weight', 'unique'
    )),

  -- cardinality y no array_length: para un array vacio array_length
  -- devuelve NULL y el CHECK lo dejaria pasar.
  constraint ventures_enabled_variant_kinds_not_empty
    check (cardinality(enabled_variant_kinds) >= 1),

  constraint ventures_enabled_variant_kinds_check
    check (enabled_variant_kinds <@ array[
      'size_letter', 'size_number', 'volume',
      'color', 'presentation', 'weight', 'unique'
    ]::text[]),

  -- <@ contra el conjunto no alcanza: un NULL adentro del array lo pasa.
  constraint ventures_enabled_variant_kinds_no_nulls
    check (array_position(enabled_variant_kinds, null) is null)
);

-- ============================================================
-- MIEMBROS
--
-- Una persona puede trabajar en varios emprendimientos: la membresia
-- es (venture, profile), y es ella la que se usa como actor en todos
-- los movimientos.
-- ============================================================

create table waliki.members (
  id                  uuid        not null default gen_random_uuid(),
  venture_id          uuid        not null,
  profile_id          uuid        not null,
  role                varchar     not null,
  active              boolean     not null default true,
  product_tutorial_at timestamptz,
  created_at          timestamptz not null default now(),
  updated_at          timestamptz not null default now(),

  constraint members_pkey primary key (id),
  constraint members_unique unique (venture_id, profile_id),
  constraint members_venture_fkey foreign key (venture_id)
    references waliki.ventures (id) on delete cascade,
  constraint members_profile_fkey foreign key (profile_id)
    references waliki.profiles (id),
  constraint members_role_check check (role in ('owner', 'seller'))
);

create index members_profile_idx on waliki.members (profile_id);

-- ============================================================
-- INVITACIONES
-- ============================================================

create table waliki.invitations (
  id                   uuid        not null default gen_random_uuid(),
  venture_id           uuid        not null,
  email                varchar,
  phone                varchar,
  role                 varchar     not null,
  token                varchar     not null,
  invited_by_member_id uuid        not null,
  expires_at           timestamptz not null,
  accepted_at          timestamptz,
  accepted_profile_id  uuid,
  created_at           timestamptz not null default now(),

  constraint invitations_pkey primary key (id),
  constraint invitations_token_key unique (token),
  constraint invitations_venture_fkey foreign key (venture_id)
    references waliki.ventures (id) on delete cascade,
  constraint invitations_inviter_fkey foreign key (invited_by_member_id)
    references waliki.members (id),
  constraint invitations_profile_fkey foreign key (accepted_profile_id)
    references waliki.profiles (id),
  constraint invitations_role_check check (role in ('owner', 'seller')),
  constraint invitations_target_check check (email is not null or phone is not null)
);

create index invitations_venture_idx
  on waliki.invitations (venture_id, created_at desc);

-- ============================================================
-- PROVEEDORES
-- ============================================================

create table waliki.suppliers (
  id         uuid        not null default gen_random_uuid(),
  venture_id uuid        not null,
  name       varchar     not null check (length(btrim(name)) > 0),
  phone      varchar,
  notes      varchar,
  active     boolean     not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),

  constraint suppliers_pkey primary key (id),
  constraint suppliers_venture_fkey foreign key (venture_id)
    references waliki.ventures (id) on delete cascade
);

create unique index suppliers_name_venture_key
  on waliki.suppliers (venture_id, upper(btrim(name)));

-- ============================================================
-- CATALOGO
-- ============================================================

create table waliki.products (
  id           uuid        not null default gen_random_uuid(),
  venture_id   uuid        not null,

  -- Lo genera y lo congela el dominio (ProductCode). Aca solo vive la
  -- garantia estructural: no vacio y unico dentro del emprendimiento.
  code         varchar     not null check (length(btrim(code)) > 0),

  name         varchar     not null check (length(btrim(name)) > 0),

  -- El kind vive ACA, no en ventures: una misma tienda vende zapatillas
  -- por numero (38, 40) y poleras por letra (M, L). Arranca con
  -- ventures.default_variant_kind y se cambia por producto.
  variant_kind varchar     not null,

  active       boolean     not null default true,
  created_at   timestamptz not null default now(),
  updated_at   timestamptz not null default now(),

  constraint products_pkey primary key (id),
  constraint products_code_venture_unique unique (venture_id, code),
  constraint products_venture_fkey foreign key (venture_id)
    references waliki.ventures (id) on delete cascade,
  constraint products_variant_kind_check
    check (variant_kind in (
      'size_letter', 'size_number', 'volume',
      'color', 'presentation', 'weight', 'unique'
    ))
);

create unique index products_name_venture_key
  on waliki.products (venture_id, upper(btrim(name)));

-- Variante: entidad interna del agregado Product. Se accede solo a
-- traves de ProductRepository, no tiene repositorio propio.
--
-- `label` es texto libre a proposito: "M", "38", "1L" y "NEGRO" son
-- todos labels validos. Lo que cambia entre rubros es que le sugiere
-- el formulario, y eso lo decide products.variant_kind.
create table waliki.variants (
  id               uuid        not null default gen_random_uuid(),
  product_id       uuid        not null,
  label            varchar     not null check (length(btrim(label)) > 0),

  -- Precio VIGENTE. El historico esta en variant_price_changes y lo
  -- realmente cobrado queda congelado en sale_items.
  suggested_price  numeric     not null default 0 check (suggested_price >= 0),

  -- Promedio ponderado de TODO lo que entro alguna vez. El costo es del
  -- emprendimiento, no de la sucursal, por eso vive aca y no en
  -- stock_balances. Lo recalcula el dominio, solo en entradas.
  avg_cost         numeric     not null default 0 check (avg_cost >= 0),

  active           boolean     not null default true,
  created_at       timestamptz not null default now(),

  constraint variants_pkey primary key (id),
  constraint variants_unique unique (product_id, label),
  constraint variants_product_fkey foreign key (product_id)
    references waliki.products (id) on delete cascade
);

create index variants_product_idx on waliki.variants (product_id);

-- ============================================================
-- UBICACIONES
--
-- La crea el caso de uso al completar el alta del emprendimiento
-- (decision 16): sin ubicacion no se puede vender ni recibir.
-- ============================================================

create table waliki.locations (
  id         uuid        not null default gen_random_uuid(),
  venture_id uuid        not null,
  name       varchar     not null check (length(btrim(name)) > 0),
  is_default boolean     not null default false,
  active     boolean     not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),

  constraint locations_pkey primary key (id),
  constraint locations_venture_fkey foreign key (venture_id)
    references waliki.ventures (id) on delete cascade
);

create unique index locations_name_venture_key
  on waliki.locations (venture_id, upper(btrim(name)));

-- Una sola ubicacion por defecto por emprendimiento.
create unique index locations_one_default
  on waliki.locations (venture_id) where is_default;

-- ============================================================
-- SALDOS DE STOCK
--
-- El stock es un saldo GUARDADO, no un sum() sobre el historial.
-- Lo mantiene el dominio dentro de la misma transaccion que el
-- movimiento que lo origina.
-- ============================================================

create table waliki.stock_balances (
  variant_id  uuid        not null,
  location_id uuid        not null,
  on_hand     integer     not null default 0,
  updated_at  timestamptz not null default now(),

  constraint stock_balances_pkey primary key (variant_id, location_id),
  constraint stock_balances_variant_fkey foreign key (variant_id)
    references waliki.variants (id) on delete cascade,
  constraint stock_balances_location_fkey foreign key (location_id)
    references waliki.locations (id)
);

create index stock_balances_location_idx on waliki.stock_balances (location_id);

-- ============================================================
-- ENTRADAS
-- ============================================================

create table waliki.stock_entries (
  id                     uuid        not null default gen_random_uuid(),
  venture_id             uuid        not null,
  location_id            uuid        not null,

  -- Idempotencia: lo genera el cliente. Reintentar la misma entrada
  -- desde la cola offline no la duplica.
  request_id             uuid        not null,

  supplier_id            uuid,
  received_by_member_id  uuid        not null,
  entered_at             timestamptz not null default now(),
  notes                  varchar,

  voided_at              timestamptz,
  voided_by_member_id    uuid,
  void_reason            varchar,

  created_by_member_id   uuid        not null,
  client_created_at      timestamptz,
  created_at             timestamptz not null default now(),

  constraint stock_entries_pkey primary key (id),
  constraint stock_entries_request_key unique (request_id),
  constraint stock_entries_venture_fkey foreign key (venture_id)
    references waliki.ventures (id),
  constraint stock_entries_location_fkey foreign key (location_id)
    references waliki.locations (id),
  constraint stock_entries_supplier_fkey foreign key (supplier_id)
    references waliki.suppliers (id),
  constraint stock_entries_receiver_fkey foreign key (received_by_member_id)
    references waliki.members (id),
  constraint stock_entries_creator_fkey foreign key (created_by_member_id)
    references waliki.members (id),
  constraint stock_entries_voider_fkey foreign key (voided_by_member_id)
    references waliki.members (id),
  constraint stock_entries_void_check
    check ((voided_at is null) = (voided_by_member_id is null))
);

create index stock_entries_venture_date_idx
  on waliki.stock_entries (venture_id, entered_at desc);
create index stock_entries_supplier_idx on waliki.stock_entries (supplier_id);

create table waliki.stock_entry_items (
  id            uuid    not null default gen_random_uuid(),
  entry_id      uuid    not null,
  variant_id    uuid    not null,
  quantity      integer not null check (quantity > 0),
  unit_cost     numeric not null default 0 check (unit_cost >= 0),
  -- Lo calcula y lo escribe el dominio: la base no hace aritmetica.
  total_cost    numeric not null check (total_cost >= 0),

  -- Congelados: lo que valia el nombre en el momento. NO es
  -- denormalizacion, la dependencia variant_id -> label no vale a
  -- traves del tiempo.
  product_name  varchar not null,
  variant_label varchar not null,

  constraint stock_entry_items_pkey primary key (id),
  constraint stock_entry_items_unique unique (entry_id, variant_id),
  constraint stock_entry_items_entry_fkey foreign key (entry_id)
    references waliki.stock_entries (id) on delete cascade,
  constraint stock_entry_items_variant_fkey foreign key (variant_id)
    references waliki.variants (id)
);

create index stock_entry_items_variant_idx on waliki.stock_entry_items (variant_id);

-- ============================================================
-- CAJA
--
-- Por turno y por MIEMBRO, no por emprendimiento: en una feria cada
-- vendedor cierra la suya.
-- ============================================================

create table waliki.cash_sessions (
  id                  uuid        not null default gen_random_uuid(),
  venture_id          uuid        not null,
  member_id           uuid        not null,
  opened_at           timestamptz not null default now(),
  opening_amount      numeric     not null default 0 check (opening_amount >= 0),
  closed_at           timestamptz,
  expected_amount     numeric,
  counted_amount      numeric,
  -- Lo calcula el dominio al cerrar la caja. Puede ser negativo.
  difference          numeric,
  closed_by_member_id uuid,
  notes               varchar,

  constraint cash_sessions_pkey primary key (id),
  constraint cash_sessions_venture_fkey foreign key (venture_id)
    references waliki.ventures (id),
  constraint cash_sessions_member_fkey foreign key (member_id)
    references waliki.members (id),
  constraint cash_sessions_closer_fkey foreign key (closed_by_member_id)
    references waliki.members (id),
  constraint cash_sessions_close_check
    check ((closed_at is null) = (closed_by_member_id is null))
);

create index cash_sessions_venture_date_idx
  on waliki.cash_sessions (venture_id, opened_at desc);

-- Un solo turno abierto por persona.
create unique index cash_sessions_one_open
  on waliki.cash_sessions (member_id) where closed_at is null;

-- ============================================================
-- VENTAS
--
-- Cabecera + lineas. Una compra de tres cosas es 1 fila en sales y 3
-- en sale_items. La venta es ANONIMA: no hay clientes ni fiado.
-- ============================================================

create table waliki.sales (
  id                       uuid        not null default gen_random_uuid(),
  request_id               uuid        not null,
  venture_id               uuid        not null,
  location_id              uuid        not null,
  collected_by_member_id   uuid        not null,
  payment_method           varchar     not null,
  sold_at                  timestamptz not null default now(),
  notes                    varchar,

  created_by_member_id     uuid        not null,
  cash_session_id          uuid,

  voided_at                timestamptz,
  voided_by_member_id      uuid,
  void_reason              varchar,

  client_created_at        timestamptz,
  created_at               timestamptz not null default now(),
  updated_at               timestamptz not null default now(),

  constraint sales_pkey primary key (id),
  constraint sales_request_key unique (request_id),
  constraint sales_venture_fkey foreign key (venture_id)
    references waliki.ventures (id),
  constraint sales_location_fkey foreign key (location_id)
    references waliki.locations (id),
  constraint sales_collector_fkey foreign key (collected_by_member_id)
    references waliki.members (id),
  constraint sales_creator_fkey foreign key (created_by_member_id)
    references waliki.members (id),
  constraint sales_voider_fkey foreign key (voided_by_member_id)
    references waliki.members (id),
  constraint sales_session_fkey foreign key (cash_session_id)
    references waliki.cash_sessions (id),
  constraint sales_payment_method_check
    check (payment_method in ('cash', 'transfer', 'qr', 'other')),
  constraint sales_void_check
    check ((voided_at is null) = (voided_by_member_id is null))
);

create index sales_venture_date_idx on waliki.sales (venture_id, sold_at desc);
create index sales_collector_idx on waliki.sales (collected_by_member_id);
create index sales_session_idx on waliki.sales (cash_session_id);

create table waliki.sale_items (
  id              uuid    not null default gen_random_uuid(),
  sale_id         uuid    not null,
  variant_id      uuid    not null,
  quantity        integer not null check (quantity > 0),

  -- Lo realmente cobrado, que puede diferir del sugerido.
  unit_amount     numeric not null check (unit_amount >= 0),
  -- Lo calcula y lo escribe el dominio: la base no hace aritmetica.
  total_amount    numeric not null check (total_amount >= 0),

  -- Congelados al vender.
  product_name    varchar not null,
  variant_label   varchar not null,
  suggested_price numeric not null check (suggested_price >= 0),
  unit_cost       numeric not null check (unit_cost >= 0),

  constraint sale_items_pkey primary key (id),
  -- Decision abierta (d): si algun dia se vende "dos iguales, la
  -- segunda mas barata", este unique sale y la clave candidata pasa a
  -- ser solo id. Hoy eso se resuelve con quantity.
  constraint sale_items_unique unique (sale_id, variant_id),
  constraint sale_items_sale_fkey foreign key (sale_id)
    references waliki.sales (id) on delete cascade,
  constraint sale_items_variant_fkey foreign key (variant_id)
    references waliki.variants (id)
);

create index sale_items_variant_idx on waliki.sale_items (variant_id);

-- ============================================================
-- TRASPASOS ENTRE SUCURSALES
-- ============================================================

create table waliki.stock_transfers (
  id                    uuid        not null default gen_random_uuid(),
  venture_id            uuid        not null,
  from_location_id      uuid        not null,
  to_location_id        uuid        not null,
  request_id            uuid        not null,
  sent_by_member_id     uuid        not null,
  received_by_member_id uuid,
  sent_at               timestamptz not null default now(),
  received_at           timestamptz,
  notes                 varchar,

  voided_at             timestamptz,
  voided_by_member_id   uuid,

  client_created_at     timestamptz,
  created_at            timestamptz not null default now(),

  constraint stock_transfers_pkey primary key (id),
  constraint stock_transfers_request_key unique (request_id),
  constraint stock_transfers_venture_fkey foreign key (venture_id)
    references waliki.ventures (id),
  constraint stock_transfers_from_fkey foreign key (from_location_id)
    references waliki.locations (id),
  constraint stock_transfers_to_fkey foreign key (to_location_id)
    references waliki.locations (id),
  constraint stock_transfers_sender_fkey foreign key (sent_by_member_id)
    references waliki.members (id),
  constraint stock_transfers_receiver_fkey foreign key (received_by_member_id)
    references waliki.members (id),
  constraint stock_transfers_voider_fkey foreign key (voided_by_member_id)
    references waliki.members (id),
  constraint stock_transfers_different_locations
    check (from_location_id <> to_location_id),
  constraint stock_transfers_receive_check
    check ((received_at is null) = (received_by_member_id is null))
);

create index stock_transfers_venture_date_idx
  on waliki.stock_transfers (venture_id, sent_at desc);

create table waliki.stock_transfer_items (
  id            uuid    not null default gen_random_uuid(),
  transfer_id   uuid    not null,
  variant_id    uuid    not null,
  quantity      integer not null check (quantity > 0),
  product_name  varchar not null,
  variant_label varchar not null,

  constraint stock_transfer_items_pkey primary key (id),
  constraint stock_transfer_items_unique unique (transfer_id, variant_id),
  constraint stock_transfer_items_transfer_fkey foreign key (transfer_id)
    references waliki.stock_transfers (id) on delete cascade,
  constraint stock_transfer_items_variant_fkey foreign key (variant_id)
    references waliki.variants (id)
);

create index stock_transfer_items_variant_idx
  on waliki.stock_transfer_items (variant_id);

-- ============================================================
-- AJUSTES
--
-- No se anulan: un ajuste equivocado se corrige con otro ajuste
-- opuesto, que queda en el historial.
-- ============================================================

create table waliki.stock_adjustments (
  id                uuid        not null default gen_random_uuid(),
  venture_id        uuid        not null,
  variant_id        uuid        not null,
  location_id       uuid        not null,
  delta             integer     not null check (delta <> 0),
  reason            varchar     not null,
  notes             varchar,
  member_id         uuid        not null,
  request_id        uuid        not null,
  adjusted_at       timestamptz not null default now(),
  client_created_at timestamptz,
  created_at        timestamptz not null default now(),

  constraint stock_adjustments_pkey primary key (id),
  constraint stock_adjustments_request_key unique (request_id),
  constraint stock_adjustments_venture_fkey foreign key (venture_id)
    references waliki.ventures (id),
  constraint stock_adjustments_variant_fkey foreign key (variant_id)
    references waliki.variants (id),
  constraint stock_adjustments_location_fkey foreign key (location_id)
    references waliki.locations (id),
  constraint stock_adjustments_member_fkey foreign key (member_id)
    references waliki.members (id),
  constraint stock_adjustments_reason_check
    check (reason in ('count', 'loss', 'customer_return', 'correction'))
);

create index stock_adjustments_venture_date_idx
  on waliki.stock_adjustments (venture_id, adjusted_at desc);
create index stock_adjustments_variant_idx
  on waliki.stock_adjustments (variant_id);

-- ============================================================
-- HISTORIAL DE PRECIOS (auditoria)
-- ============================================================

create table waliki.variant_price_changes (
  id                   bigint      generated always as identity,
  variant_id           uuid        not null,
  old_price            numeric     not null,
  new_price            numeric     not null,
  changed_by_member_id uuid,
  changed_at           timestamptz not null default now(),

  constraint variant_price_changes_pkey primary key (id),
  constraint variant_price_changes_variant_fkey foreign key (variant_id)
    references waliki.variants (id) on delete cascade,
  constraint variant_price_changes_member_fkey foreign key (changed_by_member_id)
    references waliki.members (id)
);

create index variant_price_changes_variant_date_idx
  on waliki.variant_price_changes (variant_id, changed_at desc);

-- ============================================================
-- BITACORA DE ACTIVIDAD (auditoria)
-- ============================================================

create table waliki.activity_log (
  id         bigint      generated always as identity,
  venture_id uuid        not null,
  member_id  uuid,
  action     varchar     not null,
  entity     varchar     not null,
  entity_id  uuid        not null,
  changes    jsonb,
  created_at timestamptz not null default now(),

  constraint activity_log_pkey primary key (id),
  constraint activity_log_venture_fkey foreign key (venture_id)
    references waliki.ventures (id) on delete cascade,
  constraint activity_log_member_fkey foreign key (member_id)
    references waliki.members (id),
  constraint activity_log_action_check
    check (action in ('create', 'update', 'delete', 'status_change'))
);

create index activity_venture_date_idx
  on waliki.activity_log (venture_id, created_at desc);
create index activity_entity_idx
  on waliki.activity_log (entity, entity_id);

-- ============================================================
