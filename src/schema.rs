table! {
    activities (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        icon -> Bpchar,
    }
}

table! {
    entry_activities (id) {
        id -> Int4,
        entry_id -> Int4,
        activity_id -> Int4,
    }
}

table! {
    entry_images (id) {
        id -> Int4,
        user_id -> Int4,
        entry_id -> Int4,
        image_url -> Text,
    }
}

table! {
    entrys (id) {
        id -> Int4,
        user_id -> Int4,
        mood_id -> Int4,
        desc -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    moods (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        value -> Int4,
        icon -> Bpchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        hash -> Varchar,
    }
}

joinable!(activities -> users (user_id));
joinable!(entry_activities -> activities (activity_id));
joinable!(entry_activities -> entrys (entry_id));
joinable!(entry_images -> entrys (entry_id));
joinable!(entry_images -> users (user_id));
joinable!(entrys -> moods (mood_id));
joinable!(entrys -> users (user_id));
joinable!(moods -> users (user_id));

allow_tables_to_appear_in_same_query!(
    activities,
    entry_activities,
    entry_images,
    entrys,
    moods,
    users,
);
