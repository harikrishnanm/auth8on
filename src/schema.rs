table! {
    jwks (id) {
        id -> Int4,
        created -> Timestamp,
        current -> Bool,
        p -> Text,
        q -> Text,
        d -> Text,
        qi -> Text,
        dp -> Text,
        dq -> Text,
        n -> Text,
        e -> Text,
        kty -> Varchar,
        #[sql_name = "use"]
        use_ -> Varchar,
        alg -> Varchar,
        kid -> Uuid,
    }
}
