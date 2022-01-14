table! {
    messages (id) {
        id -> Int4,
        msg -> Text,
        user_id -> Int4,
        room_id -> Int4,
    }
}

table! {
    rooms (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    rooms_users (id) {
        id -> Int4,
        user_id -> Int4,
        room_id -> Int4,
        accepted -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        nickname -> Varchar,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(messages -> rooms (room_id));
joinable!(messages -> users (user_id));
joinable!(rooms_users -> rooms (room_id));
joinable!(rooms_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    messages,
    rooms,
    rooms_users,
    users,
);
