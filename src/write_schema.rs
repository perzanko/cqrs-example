table! {
    users (id) {
        id -> Uuid,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}
