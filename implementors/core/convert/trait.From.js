(function() {var implementors = {};
implementors["twilight_cache_inmemory"] = [{"text":"impl From&lt;InMemoryConfigBuilder&gt; for InMemoryConfig","synthetic":false,"types":[]},{"text":"impl From&lt;Message&gt; for CachedMessage","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; From&lt;&amp;'_ Presence&gt; for CachedPresence","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;InMemoryConfig&gt;&gt; From&lt;T&gt; for InMemoryCache","synthetic":false,"types":[]}];
implementors["twilight_command_parser"] = [{"text":"impl&lt;'a&gt; From&lt;&amp;'a str&gt; for Arguments&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Into&lt;CommandParserConfig&lt;'a&gt;&gt;&gt; From&lt;T&gt; for Parser&lt;'a&gt;","synthetic":false,"types":[]}];
implementors["twilight_embed_builder"] = [{"text":"impl From&lt;EmbedAuthorBuilder&gt; for EmbedAuthor","synthetic":false,"types":[]},{"text":"impl From&lt;EmbedFieldBuilder&gt; for EmbedField","synthetic":false,"types":[]},{"text":"impl From&lt;EmbedFooterBuilder&gt; for EmbedFooter","synthetic":false,"types":[]}];
implementors["twilight_gateway"] = [{"text":"impl From&lt;ClusterConfigBuilder&gt; for ClusterConfig","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;String&gt;&gt; From&lt;T&gt; for ClusterConfig","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;String&gt;&gt; From&lt;T&gt; for ClusterConfigBuilder","synthetic":false,"types":[]},{"text":"impl From&lt;ShardConfigBuilder&gt; for ShardConfig","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;String&gt;&gt; From&lt;T&gt; for ShardConfig","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;String&gt;&gt; From&lt;T&gt; for ShardConfigBuilder","synthetic":false,"types":[]},{"text":"impl From&lt;EventType&gt; for EventTypeFlags","synthetic":false,"types":[]}];
implementors["twilight_http"] = [{"text":"impl From&lt;u64&gt; for ErrorCode","synthetic":false,"types":[]},{"text":"impl From&lt;Client&gt; for Client","synthetic":false,"types":[]},{"text":"impl From&lt;ParseError&gt; for UrlError","synthetic":false,"types":[]},{"text":"impl From&lt;ParseIntError&gt; for UrlError","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for Error","synthetic":false,"types":[]},{"text":"impl From&lt;Error&gt; for Error","synthetic":false,"types":[]},{"text":"impl From&lt;UrlError&gt; for Error","synthetic":false,"types":[]},{"text":"impl From&lt;Route&gt; for Request","synthetic":false,"types":[]},{"text":"impl From&lt;(Vec&lt;u8&gt;, Route)&gt; for Request","synthetic":false,"types":[]},{"text":"impl From&lt;(Form, Route)&gt; for Request","synthetic":false,"types":[]},{"text":"impl From&lt;(Vec&lt;u8&gt;, Form, Route)&gt; for Request","synthetic":false,"types":[]},{"text":"impl From&lt;(HeaderMap&lt;HeaderValue&gt;, Route)&gt; for Request","synthetic":false,"types":[]},{"text":"impl From&lt;(Vec&lt;u8&gt;, HeaderMap&lt;HeaderValue&gt;, Route)&gt; for Request","synthetic":false,"types":[]},{"text":"impl From&lt;ParseIntError&gt; for PathParseError","synthetic":false,"types":[]}];
implementors["twilight_lavalink"] = [{"text":"impl From&lt;Destroy&gt; for OutgoingEvent","synthetic":false,"types":[]},{"text":"impl From&lt;Equalizer&gt; for OutgoingEvent","synthetic":false,"types":[]},{"text":"impl From&lt;Pause&gt; for OutgoingEvent","synthetic":false,"types":[]},{"text":"impl From&lt;Play&gt; for OutgoingEvent","synthetic":false,"types":[]},{"text":"impl From&lt;Seek&gt; for OutgoingEvent","synthetic":false,"types":[]},{"text":"impl From&lt;Stop&gt; for OutgoingEvent","synthetic":false,"types":[]},{"text":"impl From&lt;VoiceUpdate&gt; for OutgoingEvent","synthetic":false,"types":[]},{"text":"impl From&lt;Volume&gt; for OutgoingEvent","synthetic":false,"types":[]},{"text":"impl From&lt;GuildId&gt; for Destroy","synthetic":false,"types":[]},{"text":"impl From&lt;(GuildId, Vec&lt;EqualizerBand&gt;)&gt; for Equalizer","synthetic":false,"types":[]},{"text":"impl From&lt;(i64, f64)&gt; for EqualizerBand","synthetic":false,"types":[]},{"text":"impl From&lt;(GuildId, bool)&gt; for Pause","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;String&gt;&gt; From&lt;(GuildId, T)&gt; for Play","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;String&gt;, S:&nbsp;Into&lt;Option&lt;u64&gt;&gt;&gt; From&lt;(GuildId, T, S)&gt; for Play","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;String&gt;, S:&nbsp;Into&lt;Option&lt;u64&gt;&gt;, E:&nbsp;Into&lt;Option&lt;u64&gt;&gt;&gt; From&lt;(GuildId, T, S, E)&gt; for Play","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;String&gt;, S:&nbsp;Into&lt;Option&lt;u64&gt;&gt;, E:&nbsp;Into&lt;Option&lt;u64&gt;&gt;&gt; From&lt;(GuildId, T, S, E, bool)&gt; for Play","synthetic":false,"types":[]},{"text":"impl From&lt;(GuildId, i64)&gt; for Seek","synthetic":false,"types":[]},{"text":"impl From&lt;GuildId&gt; for Stop","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;String&gt;&gt; From&lt;(GuildId, T, SlimVoiceServerUpdate)&gt; for VoiceUpdate","synthetic":false,"types":[]},{"text":"impl From&lt;VoiceServerUpdate&gt; for SlimVoiceServerUpdate","synthetic":false,"types":[]},{"text":"impl From&lt;(GuildId, i64)&gt; for Volume","synthetic":false,"types":[]},{"text":"impl From&lt;PlayerUpdate&gt; for IncomingEvent","synthetic":false,"types":[]},{"text":"impl From&lt;Stats&gt; for IncomingEvent","synthetic":false,"types":[]}];
implementors["twilight_model"] = [{"text":"impl From&lt;Box&lt;DispatchEvent&gt;&gt; for Event","synthetic":false,"types":[]},{"text":"impl From&lt;GatewayEvent&gt; for Event","synthetic":false,"types":[]},{"text":"impl From&lt;ShardEvent&gt; for Event","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for ApplicationId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for AttachmentId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for AuditLogEntryId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for ChannelId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for EmojiId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for GenericId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for GuildId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for IntegrationId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for MessageId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for RoleId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for UserId","synthetic":false,"types":[]},{"text":"impl From&lt;u64&gt; for WebhookId","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()