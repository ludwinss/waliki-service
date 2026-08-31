alter table waliki.ventures
  drop constraint if exists ventures_variant_presets_is_object;

alter table waliki.ventures
  drop column if exists variant_presets;
