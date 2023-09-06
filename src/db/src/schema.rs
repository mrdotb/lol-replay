// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Nullable<Integer>,
        version -> Text,
        endpoint -> Text,
        base_url -> Text,
        platform_id -> Text,
        game_id -> Text,
        encryption_key -> Text,
        metadata -> Text,
        keyframes -> Text,
        game_data_chunks -> Text,
        storage -> Text,
    }
}
