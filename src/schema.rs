// @generated automatically by Diesel CLI.

pub mod waliki {
    diesel::table! {
        waliki.activity_log (id) {
            id -> Int8,
            venture_id -> Uuid,
            member_id -> Nullable<Uuid>,
            action -> Varchar,
            entity -> Varchar,
            entity_id -> Uuid,
            changes -> Nullable<Jsonb>,
            created_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.cash_sessions (id) {
            id -> Uuid,
            venture_id -> Uuid,
            member_id -> Uuid,
            opened_at -> Timestamptz,
            opening_amount -> Numeric,
            closed_at -> Nullable<Timestamptz>,
            expected_amount -> Nullable<Numeric>,
            counted_amount -> Nullable<Numeric>,
            difference -> Nullable<Numeric>,
            closed_by_member_id -> Nullable<Uuid>,
            notes -> Nullable<Varchar>,
        }
    }

    diesel::table! {
        waliki.invitations (id) {
            id -> Uuid,
            venture_id -> Uuid,
            email -> Nullable<Varchar>,
            phone -> Nullable<Varchar>,
            role -> Varchar,
            token -> Varchar,
            invited_by_member_id -> Uuid,
            expires_at -> Timestamptz,
            accepted_at -> Nullable<Timestamptz>,
            accepted_profile_id -> Nullable<Uuid>,
            created_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.locations (id) {
            id -> Uuid,
            venture_id -> Uuid,
            name -> Varchar,
            is_default -> Bool,
            active -> Bool,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.members (id) {
            id -> Uuid,
            venture_id -> Uuid,
            profile_id -> Uuid,
            role -> Varchar,
            active -> Bool,
            product_tutorial_at -> Nullable<Timestamptz>,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.products (id) {
            id -> Uuid,
            venture_id -> Uuid,
            code -> Varchar,
            name -> Varchar,
            variant_kind -> Varchar,
            active -> Bool,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.profile_identities (id) {
            id -> Uuid,
            profile_id -> Uuid,
            provider -> Varchar,
            subject -> Varchar,
            last_login_at -> Nullable<Timestamptz>,
            created_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.profiles (id) {
            id -> Uuid,
            email -> Varchar,
            display_name -> Varchar,
            avatar_url -> Nullable<Varchar>,
            status -> Varchar,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.sale_items (id) {
            id -> Uuid,
            sale_id -> Uuid,
            variant_id -> Uuid,
            quantity -> Int4,
            unit_amount -> Numeric,
            total_amount -> Numeric,
            product_name -> Varchar,
            variant_label -> Varchar,
            suggested_price -> Numeric,
            unit_cost -> Numeric,
        }
    }

    diesel::table! {
        waliki.sales (id) {
            id -> Uuid,
            request_id -> Uuid,
            venture_id -> Uuid,
            location_id -> Uuid,
            collected_by_member_id -> Uuid,
            payment_method -> Varchar,
            sold_at -> Timestamptz,
            notes -> Nullable<Varchar>,
            created_by_member_id -> Uuid,
            cash_session_id -> Nullable<Uuid>,
            voided_at -> Nullable<Timestamptz>,
            voided_by_member_id -> Nullable<Uuid>,
            void_reason -> Nullable<Varchar>,
            client_created_at -> Nullable<Timestamptz>,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.stock_adjustments (id) {
            id -> Uuid,
            venture_id -> Uuid,
            variant_id -> Uuid,
            location_id -> Uuid,
            delta -> Int4,
            reason -> Varchar,
            notes -> Nullable<Varchar>,
            member_id -> Uuid,
            request_id -> Uuid,
            adjusted_at -> Timestamptz,
            client_created_at -> Nullable<Timestamptz>,
            created_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.stock_balances (variant_id, location_id) {
            variant_id -> Uuid,
            location_id -> Uuid,
            on_hand -> Int4,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.stock_entries (id) {
            id -> Uuid,
            venture_id -> Uuid,
            location_id -> Uuid,
            request_id -> Uuid,
            supplier_id -> Nullable<Uuid>,
            received_by_member_id -> Uuid,
            entered_at -> Timestamptz,
            notes -> Nullable<Varchar>,
            voided_at -> Nullable<Timestamptz>,
            voided_by_member_id -> Nullable<Uuid>,
            void_reason -> Nullable<Varchar>,
            created_by_member_id -> Uuid,
            client_created_at -> Nullable<Timestamptz>,
            created_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.stock_entry_items (id) {
            id -> Uuid,
            entry_id -> Uuid,
            variant_id -> Uuid,
            quantity -> Int4,
            unit_cost -> Numeric,
            total_cost -> Numeric,
            product_name -> Varchar,
            variant_label -> Varchar,
        }
    }

    diesel::table! {
        waliki.stock_transfer_items (id) {
            id -> Uuid,
            transfer_id -> Uuid,
            variant_id -> Uuid,
            quantity -> Int4,
            product_name -> Varchar,
            variant_label -> Varchar,
        }
    }

    diesel::table! {
        waliki.stock_transfers (id) {
            id -> Uuid,
            venture_id -> Uuid,
            from_location_id -> Uuid,
            to_location_id -> Uuid,
            request_id -> Uuid,
            sent_by_member_id -> Uuid,
            received_by_member_id -> Nullable<Uuid>,
            sent_at -> Timestamptz,
            received_at -> Nullable<Timestamptz>,
            notes -> Nullable<Varchar>,
            voided_at -> Nullable<Timestamptz>,
            voided_by_member_id -> Nullable<Uuid>,
            client_created_at -> Nullable<Timestamptz>,
            created_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.suppliers (id) {
            id -> Uuid,
            venture_id -> Uuid,
            name -> Varchar,
            phone -> Nullable<Varchar>,
            notes -> Nullable<Varchar>,
            active -> Bool,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.variant_price_changes (id) {
            id -> Int8,
            variant_id -> Uuid,
            old_price -> Numeric,
            new_price -> Numeric,
            changed_by_member_id -> Nullable<Uuid>,
            changed_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.variants (id) {
            id -> Uuid,
            product_id -> Uuid,
            label -> Varchar,
            suggested_price -> Numeric,
            avg_cost -> Numeric,
            active -> Bool,
            created_at -> Timestamptz,
        }
    }

    diesel::table! {
        waliki.ventures (id) {
            id -> Uuid,
            code -> Varchar,
            name -> Varchar,
            active -> Bool,
            setup_status -> Varchar,
            setup_completed_at -> Nullable<Timestamptz>,
            default_variant_kind -> Varchar,
            enabled_variant_kinds -> Array<Nullable<Text>>,
            logo_seed -> Nullable<Varchar>,
            logo_url -> Nullable<Varchar>,
            block_sale_without_stock -> Bool,
            created_at -> Timestamptz,
            updated_at -> Timestamptz,
        }
    }

    diesel::joinable!(activity_log -> members (member_id));
    diesel::joinable!(activity_log -> ventures (venture_id));
    diesel::joinable!(cash_sessions -> ventures (venture_id));
    diesel::joinable!(invitations -> members (invited_by_member_id));
    diesel::joinable!(invitations -> profiles (accepted_profile_id));
    diesel::joinable!(invitations -> ventures (venture_id));
    diesel::joinable!(locations -> ventures (venture_id));
    diesel::joinable!(members -> profiles (profile_id));
    diesel::joinable!(members -> ventures (venture_id));
    diesel::joinable!(products -> ventures (venture_id));
    diesel::joinable!(profile_identities -> profiles (profile_id));
    diesel::joinable!(sale_items -> sales (sale_id));
    diesel::joinable!(sale_items -> variants (variant_id));
    diesel::joinable!(sales -> cash_sessions (cash_session_id));
    diesel::joinable!(sales -> locations (location_id));
    diesel::joinable!(sales -> ventures (venture_id));
    diesel::joinable!(stock_adjustments -> locations (location_id));
    diesel::joinable!(stock_adjustments -> members (member_id));
    diesel::joinable!(stock_adjustments -> variants (variant_id));
    diesel::joinable!(stock_adjustments -> ventures (venture_id));
    diesel::joinable!(stock_balances -> locations (location_id));
    diesel::joinable!(stock_balances -> variants (variant_id));
    diesel::joinable!(stock_entries -> locations (location_id));
    diesel::joinable!(stock_entries -> suppliers (supplier_id));
    diesel::joinable!(stock_entries -> ventures (venture_id));
    diesel::joinable!(stock_entry_items -> stock_entries (entry_id));
    diesel::joinable!(stock_entry_items -> variants (variant_id));
    diesel::joinable!(stock_transfer_items -> stock_transfers (transfer_id));
    diesel::joinable!(stock_transfer_items -> variants (variant_id));
    diesel::joinable!(stock_transfers -> ventures (venture_id));
    diesel::joinable!(suppliers -> ventures (venture_id));
    diesel::joinable!(variant_price_changes -> members (changed_by_member_id));
    diesel::joinable!(variant_price_changes -> variants (variant_id));
    diesel::joinable!(variants -> products (product_id));

    diesel::allow_tables_to_appear_in_same_query!(
        activity_log,
        cash_sessions,
        invitations,
        locations,
        members,
        products,
        profile_identities,
        profiles,
        sale_items,
        sales,
        stock_adjustments,
        stock_balances,
        stock_entries,
        stock_entry_items,
        stock_transfer_items,
        stock_transfers,
        suppliers,
        variant_price_changes,
        variants,
        ventures,
    );
}
