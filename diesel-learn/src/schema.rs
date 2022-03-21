table! {
    /// Representation of the `posts` table.
    ///
    /// (Automatically generated by Diesel.)
    posts (id) {
        /// The `id` column of the `posts` table.
        ///
        /// Its SQL type is `Bigint`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Bigint,
        /// The `title` column of the `posts` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Varchar,
        /// The `body` column of the `posts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        body -> Text,
        /// The `published` column of the `posts` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        published -> Bool,
    }
}

table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Integer,
        /// The `name` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `hair_color` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        hair_color -> Nullable<Text>,
        /// The `created_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);