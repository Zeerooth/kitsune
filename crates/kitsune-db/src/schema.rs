// @generated automatically by Diesel CLI.

/// A module containing custom SQL type definitions
///
/// (Automatically generated by Diesel.)
pub mod sql_types {
    /// The `language_iso_code` SQL type
    ///
    /// (Automatically generated by Diesel.)
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "language_iso_code"))]
    pub struct LanguageIsoCode;
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `accounts` table.
    ///
    /// (Automatically generated by Diesel.)
    accounts (id) {
        /// The `id` column of the `accounts` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `display_name` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        display_name -> Nullable<Text>,
        /// The `note` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        note -> Nullable<Text>,
        /// The `username` column of the `accounts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        username -> Text,
        /// The `locked` column of the `accounts` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        locked -> Bool,
        /// The `local` column of the `accounts` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        local -> Bool,
        /// The `domain` column of the `accounts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        domain -> Text,
        /// The `actor_type` column of the `accounts` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        actor_type -> Int4,
        /// The `url` column of the `accounts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        url -> Text,
        /// The `featured_collection_url` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        featured_collection_url -> Nullable<Text>,
        /// The `followers_url` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        followers_url -> Nullable<Text>,
        /// The `following_url` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        following_url -> Nullable<Text>,
        /// The `inbox_url` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        inbox_url -> Nullable<Text>,
        /// The `outbox_url` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        outbox_url -> Nullable<Text>,
        /// The `shared_inbox_url` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        shared_inbox_url -> Nullable<Text>,
        /// The `public_key_id` column of the `accounts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        public_key_id -> Text,
        /// The `public_key` column of the `accounts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        public_key -> Text,
        /// The `created_at` column of the `accounts` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `accounts` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
        /// The `account_ts` column of the `accounts` table.
        ///
        /// Its SQL type is `Tsvector`.
        ///
        /// (Automatically generated by Diesel.)
        account_ts -> Tsvector,
        /// The `avatar_id` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        avatar_id -> Nullable<Uuid>,
        /// The `header_id` column of the `accounts` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        header_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `accounts_follows` table.
    ///
    /// (Automatically generated by Diesel.)
    accounts_follows (id) {
        /// The `id` column of the `accounts_follows` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `account_id` column of the `accounts_follows` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `follower_id` column of the `accounts_follows` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        follower_id -> Uuid,
        /// The `approved_at` column of the `accounts_follows` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        approved_at -> Nullable<Timestamptz>,
        /// The `url` column of the `accounts_follows` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        url -> Text,
        /// The `notify` column of the `accounts_follows` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        notify -> Bool,
        /// The `created_at` column of the `accounts_follows` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `accounts_follows` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `accounts_preferences` table.
    ///
    /// (Automatically generated by Diesel.)
    accounts_preferences (account_id) {
        /// The `account_id` column of the `accounts_preferences` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `notify_on_follow` column of the `accounts_preferences` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        notify_on_follow -> Bool,
        /// The `notify_on_follow_request` column of the `accounts_preferences` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        notify_on_follow_request -> Bool,
        /// The `notify_on_repost` column of the `accounts_preferences` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        notify_on_repost -> Bool,
        /// The `notify_on_post_update` column of the `accounts_preferences` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        notify_on_post_update -> Bool,
        /// The `notify_on_favourite` column of the `accounts_preferences` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        notify_on_favourite -> Bool,
        /// The `notify_on_mention` column of the `accounts_preferences` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        notify_on_mention -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `custom_emojis` table.
    ///
    /// (Automatically generated by Diesel.)
    custom_emojis (id) {
        /// The `id` column of the `custom_emojis` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `shortcode` column of the `custom_emojis` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        shortcode -> Text,
        /// The `domain` column of the `custom_emojis` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        domain -> Nullable<Text>,
        /// The `remote_id` column of the `custom_emojis` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        remote_id -> Nullable<Text>,
        /// The `media_attachment_id` column of the `custom_emojis` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        media_attachment_id -> Uuid,
        /// The `endorsed` column of the `custom_emojis` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        endorsed -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `job_context` table.
    ///
    /// (Automatically generated by Diesel.)
    job_context (id) {
        /// The `id` column of the `job_context` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `context` column of the `job_context` table.
        ///
        /// Its SQL type is `Jsonb`.
        ///
        /// (Automatically generated by Diesel.)
        context -> Jsonb,
        /// The `created_at` column of the `job_context` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `job_context` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `link_previews` table.
    ///
    /// (Automatically generated by Diesel.)
    link_previews (url) {
        /// The `url` column of the `link_previews` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        url -> Text,
        /// The `embed_data` column of the `link_previews` table.
        ///
        /// Its SQL type is `Jsonb`.
        ///
        /// (Automatically generated by Diesel.)
        embed_data -> Jsonb,
        /// The `expires_at` column of the `link_previews` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        expires_at -> Timestamptz,
        /// The `created_at` column of the `link_previews` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `link_previews` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `media_attachments` table.
    ///
    /// (Automatically generated by Diesel.)
    media_attachments (id) {
        /// The `id` column of the `media_attachments` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `account_id` column of the `media_attachments` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Nullable<Uuid>,
        /// The `content_type` column of the `media_attachments` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        content_type -> Text,
        /// The `description` column of the `media_attachments` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Nullable<Text>,
        /// The `blurhash` column of the `media_attachments` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        blurhash -> Nullable<Text>,
        /// The `file_path` column of the `media_attachments` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        file_path -> Nullable<Text>,
        /// The `remote_url` column of the `media_attachments` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        remote_url -> Nullable<Text>,
        /// The `created_at` column of the `media_attachments` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `media_attachments` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `notifications` table.
    ///
    /// (Automatically generated by Diesel.)
    notifications (id) {
        /// The `id` column of the `notifications` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `receiving_account_id` column of the `notifications` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        receiving_account_id -> Uuid,
        /// The `triggering_account_id` column of the `notifications` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        triggering_account_id -> Nullable<Uuid>,
        /// The `post_id` column of the `notifications` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Nullable<Uuid>,
        /// The `notification_type` column of the `notifications` table.
        ///
        /// Its SQL type is `Int2`.
        ///
        /// (Automatically generated by Diesel.)
        notification_type -> Int2,
        /// The `created_at` column of the `notifications` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `oauth2_access_tokens` table.
    ///
    /// (Automatically generated by Diesel.)
    oauth2_access_tokens (token) {
        /// The `token` column of the `oauth2_access_tokens` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        token -> Text,
        /// The `user_id` column of the `oauth2_access_tokens` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Nullable<Uuid>,
        /// The `application_id` column of the `oauth2_access_tokens` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        application_id -> Nullable<Uuid>,
        /// The `scopes` column of the `oauth2_access_tokens` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        scopes -> Text,
        /// The `created_at` column of the `oauth2_access_tokens` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `expires_at` column of the `oauth2_access_tokens` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        expires_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `oauth2_applications` table.
    ///
    /// (Automatically generated by Diesel.)
    oauth2_applications (id) {
        /// The `id` column of the `oauth2_applications` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `name` column of the `oauth2_applications` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `secret` column of the `oauth2_applications` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        secret -> Text,
        /// The `scopes` column of the `oauth2_applications` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        scopes -> Text,
        /// The `redirect_uri` column of the `oauth2_applications` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        redirect_uri -> Text,
        /// The `website` column of the `oauth2_applications` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        website -> Nullable<Text>,
        /// The `created_at` column of the `oauth2_applications` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `oauth2_applications` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `oauth2_authorization_codes` table.
    ///
    /// (Automatically generated by Diesel.)
    oauth2_authorization_codes (code) {
        /// The `code` column of the `oauth2_authorization_codes` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        code -> Text,
        /// The `application_id` column of the `oauth2_authorization_codes` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        application_id -> Uuid,
        /// The `user_id` column of the `oauth2_authorization_codes` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Uuid,
        /// The `scopes` column of the `oauth2_authorization_codes` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        scopes -> Text,
        /// The `created_at` column of the `oauth2_authorization_codes` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `expires_at` column of the `oauth2_authorization_codes` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        expires_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `oauth2_refresh_tokens` table.
    ///
    /// (Automatically generated by Diesel.)
    oauth2_refresh_tokens (token) {
        /// The `token` column of the `oauth2_refresh_tokens` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        token -> Text,
        /// The `access_token` column of the `oauth2_refresh_tokens` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        access_token -> Text,
        /// The `application_id` column of the `oauth2_refresh_tokens` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        application_id -> Uuid,
        /// The `created_at` column of the `oauth2_refresh_tokens` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;
    use super::sql_types::LanguageIsoCode;

    /// Representation of the `posts` table.
    ///
    /// (Automatically generated by Diesel.)
    posts (id) {
        /// The `id` column of the `posts` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `account_id` column of the `posts` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `in_reply_to_id` column of the `posts` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        in_reply_to_id -> Nullable<Uuid>,
        /// The `reposted_post_id` column of the `posts` table.
        ///
        /// Its SQL type is `Nullable<Uuid>`.
        ///
        /// (Automatically generated by Diesel.)
        reposted_post_id -> Nullable<Uuid>,
        /// The `is_sensitive` column of the `posts` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        is_sensitive -> Bool,
        /// The `subject` column of the `posts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        subject -> Nullable<Text>,
        /// The `content` column of the `posts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        content -> Text,
        /// The `content_source` column of the `posts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        content_source -> Text,
        /// The `content_lang` column of the `posts` table.
        ///
        /// Its SQL type is `LanguageIsoCode`.
        ///
        /// (Automatically generated by Diesel.)
        content_lang -> LanguageIsoCode,
        /// The `visibility` column of the `posts` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        visibility -> Int4,
        /// The `is_local` column of the `posts` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        is_local -> Bool,
        /// The `url` column of the `posts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        url -> Text,
        /// The `created_at` column of the `posts` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `posts` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
        /// The `post_ts` column of the `posts` table.
        ///
        /// Its SQL type is `Tsvector`.
        ///
        /// (Automatically generated by Diesel.)
        post_ts -> Tsvector,
        /// The `link_preview_url` column of the `posts` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        link_preview_url -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `posts_custom_emojis` table.
    ///
    /// (Automatically generated by Diesel.)
    posts_custom_emojis (post_id, custom_emoji_id) {
        /// The `post_id` column of the `posts_custom_emojis` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Uuid,
        /// The `custom_emoji_id` column of the `posts_custom_emojis` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        custom_emoji_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `posts_favourites` table.
    ///
    /// (Automatically generated by Diesel.)
    posts_favourites (id) {
        /// The `id` column of the `posts_favourites` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `account_id` column of the `posts_favourites` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `post_id` column of the `posts_favourites` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Uuid,
        /// The `url` column of the `posts_favourites` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        url -> Text,
        /// The `created_at` column of the `posts_favourites` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `posts_media_attachments` table.
    ///
    /// (Automatically generated by Diesel.)
    posts_media_attachments (post_id, media_attachment_id) {
        /// The `post_id` column of the `posts_media_attachments` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Uuid,
        /// The `media_attachment_id` column of the `posts_media_attachments` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        media_attachment_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `posts_mentions` table.
    ///
    /// (Automatically generated by Diesel.)
    posts_mentions (post_id, account_id) {
        /// The `post_id` column of the `posts_mentions` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Uuid,
        /// The `account_id` column of the `posts_mentions` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `mention_text` column of the `posts_mentions` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        mention_text -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `account_id` column of the `users` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        account_id -> Uuid,
        /// The `oidc_id` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        oidc_id -> Nullable<Text>,
        /// The `username` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        username -> Text,
        /// The `email` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Text,
        /// The `password` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        password -> Nullable<Text>,
        /// The `domain` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        domain -> Text,
        /// The `private_key` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        private_key -> Text,
        /// The `confirmed_at` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        confirmed_at -> Nullable<Timestamptz>,
        /// The `confirmation_token` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        confirmation_token -> Text,
        /// The `created_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
        /// The `updated_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::Tsvector;

    /// Representation of the `users_roles` table.
    ///
    /// (Automatically generated by Diesel.)
    users_roles (id) {
        /// The `id` column of the `users_roles` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Uuid,
        /// The `user_id` column of the `users_roles` table.
        ///
        /// Its SQL type is `Uuid`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Uuid,
        /// The `role` column of the `users_roles` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        role -> Int4,
        /// The `created_at` column of the `users_roles` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamptz,
    }
}

diesel::joinable!(accounts_preferences -> accounts (account_id));
diesel::joinable!(custom_emojis -> media_attachments (media_attachment_id));
diesel::joinable!(notifications -> posts (post_id));
diesel::joinable!(oauth2_access_tokens -> oauth2_applications (application_id));
diesel::joinable!(oauth2_access_tokens -> users (user_id));
diesel::joinable!(oauth2_authorization_codes -> oauth2_applications (application_id));
diesel::joinable!(oauth2_authorization_codes -> users (user_id));
diesel::joinable!(oauth2_refresh_tokens -> oauth2_access_tokens (access_token));
diesel::joinable!(oauth2_refresh_tokens -> oauth2_applications (application_id));
diesel::joinable!(posts -> accounts (account_id));
diesel::joinable!(posts -> link_previews (link_preview_url));
diesel::joinable!(posts_custom_emojis -> custom_emojis (custom_emoji_id));
diesel::joinable!(posts_custom_emojis -> posts (post_id));
diesel::joinable!(posts_favourites -> accounts (account_id));
diesel::joinable!(posts_favourites -> posts (post_id));
diesel::joinable!(posts_media_attachments -> media_attachments (media_attachment_id));
diesel::joinable!(posts_media_attachments -> posts (post_id));
diesel::joinable!(posts_mentions -> accounts (account_id));
diesel::joinable!(posts_mentions -> posts (post_id));
diesel::joinable!(users -> accounts (account_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    accounts_follows,
    accounts_preferences,
    custom_emojis,
    job_context,
    link_previews,
    media_attachments,
    notifications,
    oauth2_access_tokens,
    oauth2_applications,
    oauth2_authorization_codes,
    oauth2_refresh_tokens,
    posts,
    posts_custom_emojis,
    posts_favourites,
    posts_media_attachments,
    posts_mentions,
    users,
    users_roles,
);
