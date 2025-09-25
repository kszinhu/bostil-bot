//! Macros for guild-aware internationalization

/// Guild-aware translation macro
///
/// This macro works like the standard `t!` macro but automatically uses
/// the current guild's language preference from the database.
///
/// # Examples
///
/// ```rust
/// use bostil_core::gt;
///
/// // Simple translation
/// let message = gt!("commands.ping.response");
///
/// // Translation with arguments
/// let message = gt!("commands.language.reply", language_name => "English");
///
/// // Translation with explicit locale fallback
/// let message = gt!("commands.ping.response", locale = "pt-BR");
/// ```
#[macro_export]
macro_rules! gt {
    // Simple case: gt!("key")
    ($key:expr) => {
        $crate::backends::i18n::translate_for_guild($key, None, None)
    };

    // With locale fallback: gt!("key", locale = "en-US")
    ($key:expr, locale = $locale:expr) => {
        $crate::backends::i18n::translate_for_guild($key, Some($locale), None)
    };

    // With single argument: gt!("key", arg => value)
    ($key:expr, $arg:ident => $value:expr) => {
        $crate::backends::i18n::translate_for_guild(
            $key,
            None,
            Some(&[(stringify!($arg), &$value)])
        )
    };

    // With single argument and locale: gt!("key", locale = "en-US", arg => value)
    ($key:expr, locale = $locale:expr, $arg:ident => $value:expr) => {
        $crate::backends::i18n::translate_for_guild(
            $key,
            Some($locale),
            Some(&[(stringify!($arg), &$value)])
        )
    };

    // With multiple arguments: gt!("key", arg1 => value1, arg2 => value2)
    ($key:expr, $($arg:ident => $value:expr),+) => {
        $crate::backends::i18n::translate_for_guild(
            $key,
            None,
            Some(&[$(( stringify!($arg), &$value )),+])
        )
    };

    // With multiple arguments and locale: gt!("key", locale = "en-US", arg1 => value1, arg2 => value2)
    ($key:expr, locale = $locale:expr, $($arg:ident => $value:expr),+) => {
        $crate::backends::i18n::translate_for_guild(
            $key,
            Some($locale),
            Some(&[$(( stringify!($arg), &$value )),+])
        )
    };
}

/// Sets the current guild context for translations
///
/// This should be called at the beginning of command execution
/// to ensure translations use the correct guild language.
///
/// # Example
///
/// ```rust
/// use bostil_core::set_guild_context;
/// use serenity::all::GuildId;
///
/// set_guild_context!(GuildId::new(123456789));
/// ```
#[macro_export]
macro_rules! set_guild_context {
	($guild_id:expr) => {
		$crate::i18n::set_current_guild($guild_id);
	};
}
