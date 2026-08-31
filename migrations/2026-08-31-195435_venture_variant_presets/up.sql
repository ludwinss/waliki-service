-- ============================================================
-- variant_presets: los valores de cada metrica (tallas, colores,
-- pesos, ...) salen de la configuracion del emprendimiento, no de
-- una constante del codigo. Espejo de la migracion homologa en
-- Supabase (20260829_venture_variant_presets.sql) para la app
-- movil, ahora en el esquema `waliki` propio del servicio.
--
-- Va como jsonb y no como tabla aparte por lo mismo que
-- enabled_variant_kinds: no es una entidad, son strings que
-- siempre se leen junto con el emprendimiento. Forma:
-- { "<kind>": ["<valor>", ...] }, solo con los kinds que el dueno
-- configuro; los que falten caen al preset del dominio.
--
-- Por decision 20 esta migracion NO lleva funciones ni triggers:
-- el CHECK de aca solo garantiza que sea un objeto jsonb. Que las
-- claves sean kinds conocidos y los valores arrays no vacios de
-- strings no vacios (lo mismo que valida la funcion
-- variant_presets_valid en Supabase) es un invariante del
-- agregado Venture y se valida en el dominio, no en la base.
-- ============================================================

alter table waliki.ventures
  add column variant_presets jsonb not null default '{}'::jsonb;

alter table waliki.ventures
  add constraint ventures_variant_presets_is_object
    check (jsonb_typeof(variant_presets) = 'object');
