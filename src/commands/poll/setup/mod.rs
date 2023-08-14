use serenity::builder::CreateApplicationCommand;

pub mod create;
pub mod options;

/**
 * commands:
 * - poll setup (name, description, type, timer)
 *   ~ Setup creates a thread to add options with the poll (status: stopped)
 * - poll options (name, description)
 *  ~ Options adds a new option to the poll (status: stopped)
 * - poll status set (status: open, close, stop)
 */
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("poll")
        .name_localized("pt-BR", "urna")
        .description("Create, edit or remove a poll")
        .description_localized("pt-BR", "Cria, edita ou remove uma votação")
        .add_option(self::options::register_option())
        .add_option(self::create::register_option())
        .add_option(super::help::register_option())
}
