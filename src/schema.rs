table! {
    locations (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        location_type -> Nullable<Varchar>,
    }
}

table! {
    permissions (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Integer,
        uname -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        active -> Integer,
    }
}

table! {
    users_permissions (id) {
        id -> Integer,
        user_id -> Integer,
        permission_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    locations,
    permissions,
    users,
    users_permissions,
);
