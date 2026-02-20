//! `OpenAPI` documentation and Swagger UI.
#![allow(clippy::needless_for_each)]

use axum::Router;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use super::AppState;

/// JWT Bearer authentication security scheme.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "BearerAuth",
                SecurityScheme::Http(
                    Http::builder()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "VoiceChat API",
        version = "0.1.0",
        description = "Self-hosted voice and text chat platform API",
        license(name = "MIT OR Apache-2.0"),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication and user management"),
        (name = "chat", description = "Text messaging and channels"),
        (name = "dm", description = "Direct messages"),
        (name = "guilds", description = "Guild (server) management"),
        (name = "roles", description = "Guild role management"),
        (name = "invites", description = "Guild invites"),
        (name = "categories", description = "Channel categories"),
        (name = "emojis", description = "Custom emojis"),
        (name = "admin", description = "System administration"),
        (name = "social", description = "Friends and blocking"),
        (name = "voice", description = "Voice channels and calls"),
        (name = "crypto", description = "End-to-end encryption key management"),
        (name = "bots", description = "Bot applications and gateway"),
        (name = "commands", description = "Slash commands"),
        (name = "webhooks", description = "Webhook management"),
        (name = "pins", description = "Pinned items"),
        (name = "favorites", description = "Favorite channels"),
        (name = "reactions", description = "Message reactions"),
        (name = "preferences", description = "User preferences"),
        (name = "moderation", description = "Content moderation and reports"),
        (name = "pages", description = "Platform pages (TOS, Privacy, etc.)"),
        (name = "system", description = "Health, settings, and setup"),
        (name = "connectivity", description = "Connection status"),
        (name = "search", description = "Search endpoints"),
    ),
    paths(
        // Auth (17)
        crate::auth::handlers::register,
        crate::auth::handlers::login,
        crate::auth::handlers::refresh_token,
        crate::auth::handlers::logout,
        crate::auth::handlers::get_profile,
        crate::auth::handlers::upload_avatar,
        crate::auth::handlers::update_profile,
        crate::auth::handlers::mfa_setup,
        crate::auth::handlers::mfa_verify,
        crate::auth::handlers::mfa_disable,
        crate::auth::handlers::mfa_generate_backup_codes,
        crate::auth::handlers::mfa_backup_code_count,
        crate::auth::handlers::oidc_providers,
        crate::auth::handlers::oidc_authorize,
        crate::auth::handlers::oidc_callback,
        crate::auth::handlers::forgot_password,
        crate::auth::handlers::reset_password,

        // Chat — channels (8)
        crate::chat::channels::create,
        crate::chat::channels::get,
        crate::chat::channels::update,
        crate::chat::channels::delete,
        crate::chat::channels::list_members,
        crate::chat::channels::add_member,
        crate::chat::channels::remove_member,
        crate::chat::channels::mark_as_read,

        // Chat — messages (6)
        crate::chat::messages::list,
        crate::chat::messages::create,
        crate::chat::messages::update,
        crate::chat::messages::delete,
        crate::chat::messages::list_thread_replies,
        crate::chat::messages::mark_thread_read,

        // Chat — DMs (9)
        crate::chat::dm::create_dm,
        crate::chat::dm::list_dms,
        crate::chat::dm::get_dm,
        crate::chat::dm::leave_dm,
        crate::chat::dm::update_dm_name,
        crate::chat::dm::upload_dm_icon,
        crate::chat::dm::get_dm_icon,
        crate::chat::dm::mark_as_read,
        crate::chat::dm::mark_all_dms_read,

        // Chat — DM search (1)
        crate::chat::dm_search::search_dm_messages,

        // Guilds (18)
        crate::guild::handlers::create_guild,
        crate::guild::handlers::list_guilds,
        crate::guild::handlers::get_guild,
        crate::guild::handlers::update_guild,
        crate::guild::handlers::delete_guild,
        crate::guild::handlers::join_guild,
        crate::guild::handlers::leave_guild,
        crate::guild::handlers::list_members,
        crate::guild::handlers::kick_member,
        crate::guild::handlers::list_channels,
        crate::guild::handlers::reorder_channels,
        crate::guild::handlers::add_bot_to_guild,
        crate::guild::handlers::list_guild_bots,
        crate::guild::handlers::remove_bot_from_guild,
        crate::guild::handlers::list_guild_commands,
        crate::guild::handlers::mark_all_channels_read,
        crate::guild::handlers::get_guild_settings,
        crate::guild::handlers::update_guild_settings,

        // Roles (6)
        crate::guild::roles::list_roles,
        crate::guild::roles::create_role,
        crate::guild::roles::update_role,
        crate::guild::roles::delete_role,
        crate::guild::roles::assign_role,
        crate::guild::roles::remove_role,

        // Invites (4)
        crate::guild::invites::list_invites,
        crate::guild::invites::create_invite,
        crate::guild::invites::delete_invite,
        crate::guild::invites::join_via_invite,

        // Categories (5)
        crate::guild::categories::list_categories,
        crate::guild::categories::create_category,
        crate::guild::categories::update_category,
        crate::guild::categories::delete_category,
        crate::guild::categories::reorder_categories,

        // Emojis (5)
        crate::guild::emojis::list_emojis,
        crate::guild::emojis::get_emoji,
        crate::guild::emojis::create_emoji,
        crate::guild::emojis::update_emoji,
        crate::guild::emojis::delete_emoji,

        // Guild search (1)
        crate::guild::search::search_messages,

        // Admin (26)
        crate::admin::handlers::get_admin_status,
        crate::admin::handlers::get_admin_stats,
        crate::admin::handlers::list_users,
        crate::admin::handlers::list_guilds,
        crate::admin::handlers::get_audit_log,
        crate::admin::handlers::elevate_session,
        crate::admin::handlers::de_elevate_session,
        crate::admin::handlers::ban_user,
        crate::admin::handlers::unban_user,
        crate::admin::handlers::suspend_guild,
        crate::admin::handlers::unsuspend_guild,
        crate::admin::handlers::create_announcement,
        crate::admin::handlers::get_user_details,
        crate::admin::handlers::get_guild_details,
        crate::admin::handlers::export_users_csv,
        crate::admin::handlers::export_guilds_csv,
        crate::admin::handlers::bulk_ban_users,
        crate::admin::handlers::bulk_suspend_guilds,
        crate::admin::handlers::get_auth_settings,
        crate::admin::handlers::update_auth_settings,
        crate::admin::handlers::list_oidc_providers,
        crate::admin::handlers::create_oidc_provider,
        crate::admin::handlers::update_oidc_provider,
        crate::admin::handlers::delete_oidc_provider,
        crate::admin::handlers::delete_user,
        crate::admin::handlers::delete_guild,

        // Social (9)
        crate::social::friends::send_friend_request,
        crate::social::friends::list_friends,
        crate::social::friends::list_pending_requests,
        crate::social::friends::list_blocked,
        crate::social::friends::accept_friend_request,
        crate::social::friends::reject_friend_request,
        crate::social::friends::block_user,
        crate::social::friends::unblock_user,
        crate::social::friends::remove_friend,

        // Voice (1)
        crate::voice::handlers::get_ice_servers,

        // Voice — calls (5)
        crate::voice::call_handlers::get_call,
        crate::voice::call_handlers::start_call,
        crate::voice::call_handlers::join_call,
        crate::voice::call_handlers::decline_call,
        crate::voice::call_handlers::leave_call,

        // Crypto (7)
        crate::crypto::handlers::upload_keys,
        crate::crypto::handlers::get_user_keys,
        crate::crypto::handlers::get_own_devices,
        crate::crypto::handlers::claim_prekey,
        crate::crypto::handlers::upload_backup,
        crate::crypto::handlers::get_backup,
        crate::crypto::handlers::get_backup_status,

        // Connectivity (3)
        crate::connectivity::handlers::get_summary,
        crate::connectivity::handlers::get_sessions,
        crate::connectivity::handlers::get_session_detail,

        // Moderation (1)
        crate::moderation::handlers::create_report,

        // Moderation — filters (8)
        crate::moderation::filter_handlers::list_filter_configs,
        crate::moderation::filter_handlers::update_filter_configs,
        crate::moderation::filter_handlers::list_custom_patterns,
        crate::moderation::filter_handlers::create_custom_pattern,
        crate::moderation::filter_handlers::update_custom_pattern,
        crate::moderation::filter_handlers::delete_custom_pattern,
        crate::moderation::filter_handlers::list_moderation_log,
        crate::moderation::filter_handlers::test_filter,

        // Pages (15)
        crate::pages::handlers::list_platform_pages,
        crate::pages::handlers::get_platform_page,
        crate::pages::handlers::create_platform_page,
        crate::pages::handlers::update_platform_page,
        crate::pages::handlers::delete_platform_page,
        crate::pages::handlers::reorder_platform_pages,
        crate::pages::handlers::list_guild_pages,
        crate::pages::handlers::get_guild_page,
        crate::pages::handlers::create_guild_page,
        crate::pages::handlers::update_guild_page,
        crate::pages::handlers::delete_guild_page,
        crate::pages::handlers::reorder_guild_pages,
        crate::pages::handlers::accept_page,
        crate::pages::handlers::accept_guild_page,
        crate::pages::handlers::get_pending_acceptance,

        // Webhooks (7)
        crate::webhooks::handlers::create_webhook,
        crate::webhooks::handlers::list_webhooks,
        crate::webhooks::handlers::get_webhook,
        crate::webhooks::handlers::update_webhook,
        crate::webhooks::handlers::delete_webhook,
        crate::webhooks::handlers::test_webhook,
        crate::webhooks::handlers::list_deliveries,

        // Bots (7)
        super::bots::create_application,
        super::bots::list_applications,
        super::bots::get_application,
        super::bots::create_bot,
        super::bots::reset_bot_token,
        super::bots::delete_application,
        super::bots::update_gateway_intents,

        // Commands (4)
        super::commands::register_commands,
        super::commands::list_commands,
        super::commands::delete_command,
        super::commands::delete_all_commands,

        // Favorites (5)
        super::favorites::list_favorites,
        super::favorites::add_favorite,
        super::favorites::remove_favorite,
        super::favorites::reorder_channels,
        super::favorites::reorder_guilds,

        // Reactions (3)
        super::reactions::add_reaction,
        super::reactions::remove_reaction,
        super::reactions::get_reactions,

        // Pins (5)
        super::pins::list_pins,
        super::pins::create_pin,
        super::pins::update_pin,
        super::pins::delete_pin,
        super::pins::reorder_pins,

        // Unread (2)
        super::unread::get_unread_aggregate,
        super::unread::mark_all_read,

        // Settings (2)
        super::settings::get_server_settings,
        super::settings::get_upload_limits,

        // Setup (3)
        super::setup::status,
        super::setup::get_config,
        super::setup::complete,

        // Preferences (2)
        super::preferences::get_preferences,
        super::preferences::update_preferences,

        // Global search (1)
        super::global_search::search_all,
    ),
    components(schemas(
        // db::models
        crate::db::Channel,
        crate::db::ChannelType,
        crate::db::PublicOidcProvider,
        crate::db::AuthMethodsConfig,
        crate::db::ChannelUnread,
        crate::db::GuildUnreadSummary,
        crate::db::UnreadAggregate,

        // auth::handlers
        crate::auth::handlers::RegisterRequest,
        crate::auth::handlers::LoginRequest,
        crate::auth::handlers::RefreshRequest,
        crate::auth::handlers::LogoutRequest,
        crate::auth::handlers::AuthResponse,
        crate::auth::handlers::UserProfile,
        crate::auth::handlers::MfaSetupResponse,
        crate::auth::handlers::MfaBackupCodesResponse,
        crate::auth::handlers::MfaBackupCodeCountResponse,
        crate::auth::handlers::MfaVerifyRequest,
        crate::auth::handlers::UpdateProfileRequest,
        crate::auth::handlers::UpdateProfileResponse,
        crate::auth::handlers::ForgotPasswordRequest,
        crate::auth::handlers::ResetPasswordRequest,

        // chat::channels
        crate::chat::channels::ChannelResponse,
        crate::chat::channels::CreateChannelRequest,
        crate::chat::channels::UpdateChannelRequest,
        crate::chat::channels::AddMemberRequest,
        crate::chat::channels::MemberResponse,
        crate::chat::channels::MarkChannelAsReadRequest,

        // chat::messages
        crate::chat::messages::AuthorProfile,
        crate::chat::messages::AttachmentInfo,
        crate::chat::messages::MentionType,
        crate::chat::messages::ThreadInfoResponse,
        crate::chat::messages::MessageResponse,
        crate::chat::messages::ReactionInfo,
        crate::chat::messages::CursorPaginatedMessageResponse,
        crate::chat::messages::CreateMessageRequest,
        crate::chat::messages::UpdateMessageRequest,

        // chat::dm
        crate::chat::dm::CreateDMRequest,
        crate::chat::dm::DMResponse,
        crate::chat::dm::LastMessagePreview,
        crate::chat::dm::DMListResponse,
        crate::chat::dm::DMParticipant,
        crate::chat::dm::UpdateDMNameRequest,
        crate::chat::dm::DMIconResponse,
        crate::chat::dm::MarkAsReadRequest,
        crate::chat::dm::MarkAsReadResponse,

        // chat::dm_search
        crate::chat::dm_search::DmSearchAuthor,
        crate::chat::dm_search::DmSearchResult,
        crate::chat::dm_search::DmSearchResponse,

        // guild::types
        crate::guild::types::Guild,
        crate::guild::types::GuildWithMemberCount,
        crate::guild::types::CreateGuildRequest,
        crate::guild::types::UpdateGuildRequest,
        crate::guild::types::JoinGuildRequest,
        crate::guild::types::GuildMember,
        crate::guild::types::GuildInvite,
        crate::guild::types::CreateInviteRequest,
        crate::guild::types::InviteResponse,
        crate::guild::types::CreateRoleRequest,
        crate::guild::types::UpdateRoleRequest,
        crate::guild::types::RoleResponse,
        crate::guild::types::GuildEmoji,
        crate::guild::types::CreateEmojiRequest,
        crate::guild::types::UpdateEmojiRequest,
        crate::guild::types::GuildSettings,
        crate::guild::types::UpdateGuildSettingsRequest,
        crate::guild::types::GuildCommandInfo,

        // guild::handlers
        crate::guild::handlers::ChannelWithUnread,
        crate::guild::handlers::InstalledBot,
        crate::guild::handlers::ChannelPosition,
        crate::guild::handlers::ReorderChannelsRequest,

        // guild::categories
        crate::guild::categories::Category,
        crate::guild::categories::CreateCategoryRequest,
        crate::guild::categories::UpdateCategoryRequest,
        crate::guild::categories::ReorderRequest,
        crate::guild::categories::CategoryPosition,

        // guild::search
        crate::guild::search::SearchAuthor,
        crate::guild::search::SearchResult,
        crate::guild::search::SearchResponse,

        // admin::types
        crate::admin::types::ElevateRequest,
        crate::admin::types::ElevateResponse,
        crate::admin::types::GlobalBanRequest,
        crate::admin::types::SuspendGuildRequest,
        crate::admin::types::CreateAnnouncementRequest,
        crate::admin::types::AdminStatusResponse,
        crate::admin::types::AdminStatsResponse,
        crate::admin::types::BulkBanRequest,
        crate::admin::types::BulkBanResponse,
        crate::admin::types::BulkSuspendRequest,
        crate::admin::types::BulkSuspendResponse,
        crate::admin::types::BulkActionFailure,

        // admin::handlers
        crate::admin::handlers::PaginatedUserSummary,
        crate::admin::handlers::PaginatedGuildSummary,
        crate::admin::handlers::PaginatedAuditLogEntry,
        crate::admin::handlers::UserSummary,
        crate::admin::handlers::GuildSummary,
        crate::admin::handlers::AuditLogEntryResponse,
        crate::admin::handlers::DeElevateResponse,
        crate::admin::handlers::UserGuildMembership,
        crate::admin::handlers::UserDetailsResponse,
        crate::admin::handlers::GuildMemberInfo,
        crate::admin::handlers::GuildOwnerInfo,
        crate::admin::handlers::GuildDetailsResponse,
        crate::admin::handlers::BanResponse,
        crate::admin::handlers::SuspendResponse,
        crate::admin::handlers::DeleteResponse,
        crate::admin::handlers::AnnouncementResponse,
        crate::admin::handlers::AuthSettingsResponse,
        crate::admin::handlers::UpdateAuthSettingsRequest,
        crate::admin::handlers::OidcProviderResponse,
        crate::admin::handlers::CreateOidcProviderRequest,
        crate::admin::handlers::UpdateOidcProviderRequest,

        // social::types
        crate::social::types::FriendshipStatus,
        crate::social::types::Friendship,
        crate::social::types::Friend,
        crate::social::types::SendFriendRequestBody,

        // voice::handlers
        crate::voice::handlers::IceServer,
        crate::voice::handlers::IceServersResponse,

        // voice::call
        crate::voice::call::EndReason,
        crate::voice::call::CallState,

        // voice::call_handlers
        crate::voice::call_handlers::CallStateResponse,

        // crypto::handlers
        crate::crypto::handlers::UploadKeysRequest,
        crate::crypto::handlers::PrekeyUpload,
        crate::crypto::handlers::UploadKeysResponse,
        crate::crypto::handlers::UserKeysResponse,
        crate::crypto::handlers::ClaimPrekeyRequest,
        crate::crypto::handlers::ClaimPrekeyResponse,
        crate::crypto::handlers::ClaimedPrekey,
        crate::crypto::handlers::DeviceKeys,
        crate::crypto::handlers::UploadBackupRequest,
        crate::crypto::handlers::BackupResponse,
        crate::crypto::handlers::BackupStatusResponse,

        // connectivity::handlers
        crate::connectivity::handlers::ConnectionSummary,
        crate::connectivity::handlers::DailyStat,
        crate::connectivity::handlers::SessionSummary,
        crate::connectivity::handlers::SessionListResponse,
        crate::connectivity::handlers::SessionDetail,
        crate::connectivity::handlers::MetricPoint,

        // moderation::types
        crate::moderation::types::ReportCategory,
        crate::moderation::types::ReportStatus,
        crate::moderation::types::ReportTargetType,
        crate::moderation::types::CreateReportRequest,
        crate::moderation::types::ResolveReportRequest,
        crate::moderation::types::Report,
        crate::moderation::types::ReportResponse,
        crate::moderation::types::ReportStatsResponse,
        crate::moderation::types::PaginatedReports,

        // moderation::filter_types
        crate::moderation::filter_types::FilterCategory,
        crate::moderation::filter_types::FilterAction,
        crate::moderation::filter_types::GuildFilterConfig,
        crate::moderation::filter_types::GuildFilterPattern,
        crate::moderation::filter_types::ModerationAction,
        crate::moderation::filter_types::FilterConfigEntry,
        crate::moderation::filter_types::UpdateFilterConfigsRequest,
        crate::moderation::filter_types::CreatePatternRequest,
        crate::moderation::filter_types::UpdatePatternRequest,
        crate::moderation::filter_types::TestFilterRequest,
        crate::moderation::filter_types::PaginatedModerationLog,
        crate::moderation::filter_types::TestFilterResponse,
        crate::moderation::filter_types::FilterMatchResponse,

        // pages::types
        crate::pages::types::Page,
        crate::pages::types::PageListItem,
        crate::pages::types::CreatePageRequest,
        crate::pages::types::UpdatePageRequest,
        crate::pages::types::ReorderRequest,

        // webhooks::types
        crate::webhooks::types::WebhookCreatedResponse,
        crate::webhooks::types::WebhookResponse,
        crate::webhooks::types::CreateWebhookRequest,
        crate::webhooks::types::UpdateWebhookRequest,
        crate::webhooks::types::DeliveryLogEntry,
        crate::webhooks::types::TestDeliveryResult,

        // webhooks::events
        crate::webhooks::events::BotEventType,

        // api::bots
        super::bots::CreateApplicationRequest,
        super::bots::ApplicationResponse,
        super::bots::BotTokenResponse,
        super::bots::UpdateIntentsRequest,

        // api::commands
        super::commands::CommandOptionType,
        super::commands::CommandOption,
        super::commands::RegisterCommandsRequest,
        super::commands::RegisterCommandData,
        super::commands::CommandResponse,

        // api::favorites
        super::favorites::FavoriteChannel,
        super::favorites::FavoritesResponse,
        super::favorites::Favorite,
        super::favorites::ReorderChannelsRequest,
        super::favorites::ReorderGuildsRequest,

        // api::reactions
        super::reactions::AddReactionRequest,
        super::reactions::ReactionResponse,

        // api::pins
        super::pins::PinType,
        super::pins::Pin,
        super::pins::CreatePinRequest,
        super::pins::UpdatePinRequest,
        super::pins::ReorderPinsRequest,

        // api::global_search
        super::global_search::GlobalSearchAuthor,
        super::global_search::GlobalSearchSource,
        super::global_search::GlobalSearchResult,
        super::global_search::GlobalSearchResponse,

        // api::setup
        super::setup::SetupStatusResponse,
        super::setup::SetupConfigResponse,
        super::setup::CompleteSetupRequest,

        // api::settings
        super::settings::ServerSettingsResponse,
        super::settings::UploadLimitsResponse,

        // api::preferences
        super::preferences::PreferencesResponse,
        super::preferences::UpdatePreferencesRequest,
    ))
)]
pub struct ApiDoc;

/// Create the Swagger UI router serving at `/api/docs`.
pub fn swagger_router() -> Router<AppState> {
    SwaggerUi::new("/api/docs")
        .url("/api/docs/openapi.json", ApiDoc::openapi())
        .into()
}
