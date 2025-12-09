use bostil_core::{
	database::exports::{establish_connection, User},
	gt as t,
};
use serenity::{
	all::{standard::CommandResult, Context, EditRole, Guild, GuildId, RoleId, User as SerenityUser, UserId},
	futures::StreamExt,
};
use tracing::{debug, error};

use crate::modules::core::roles::{get_all_roles, get_role_by_level};

/// Assign a role to a user in the guild
pub async fn add_role(ctx: &Context, guild: &Guild, user_id: &UserId, role_id: &RoleId) -> CommandResult<String> {
	match guild.member(ctx, *user_id).await {
		Ok(member) => {
			if let Err(why) = member.add_role(ctx, *role_id).await {
				error!("Failed to add role to user: {:?}", why);
				return Ok(t!("general.member.add_role_failed").to_string());
			}
			Ok(t!("general.member.add_role_success").to_string())
		}
		Err(why) => {
			error!("Failed to fetch member: {:?}", why);
			Ok(t!("general.member.fetch_member_failed").to_string())
		}
	}
}

/// Remove a role from a user in the guild
pub async fn remove_role(ctx: &Context, guild: &Guild, user_id: &UserId, role_id: &RoleId) -> CommandResult<String> {
	match guild.member(ctx, *user_id).await {
		Ok(member) => {
			if let Err(why) = member.remove_role(ctx, *role_id).await {
				error!("Failed to remove role from user: {:?}", why);
				return Ok(t!("general.role.remove_role_failed").to_string());
			}
			Ok(t!("general.role.remove_role_success").to_string())
		}
		Err(why) => {
			error!("Failed to fetch member: {:?}", why);
			Ok(t!("general.fetch_member_failed").to_string())
		}
	}
}

/// Create a new role in the guild
pub async fn create_role(ctx: &Context, guild_id: &GuildId, role: EditRole<'_>) -> CommandResult<String> {
	match guild_id.create_role(&ctx.http, role).await {
		Ok(_) => Ok(t!("general.role.create_role_success").to_string()),
		Err(why) => {
			debug!("Failed to create role: {:?}", why);
			Ok(t!("general.role.create_role_failed").to_string())
		}
	}
}

/// Register all bot roles in the guild if they do not exist
pub async fn register_bot_roles(ctx: &Context, guild_id: &GuildId) -> CommandResult<String> {
	for (role_name, role) in get_all_roles() {
		if let Err(why) = create_role(ctx, guild_id, role.clone()).await {
			debug!("Failed to create role {}: {:?}", role_name, why);
			return Ok(t!("general.role.register_roles_failed").to_string());
		}
	}

	Ok(t!("general.role.register_roles_success").to_string())
}

pub async fn register_users_on_guild(ctx: &Context, guild: &GuildId) -> CommandResult<String> {
	let mut members = guild.members_iter(&ctx.http).boxed();
	while let Some(member_result) = members.next().await {
		match member_result {
			Ok(member) => {
				debug!("Registering user: {} ({})", member.user.name, member.user.id);
				register_user_in_db(&member.user).await;
			}
			Err(error) => eprintln!("Uh oh!  Error: {}", error),
		}
	}

	Ok(t!("general.register_users_success").to_string())
}

async fn register_user_in_db(user: &SerenityUser) {
	if user.bot {
		debug!("Skipping bot user: {} ({})", user.name, user.id);
		return;
	}

	let conn = &mut establish_connection();

	match User::new(user.id, &user.name, true).save(conn) {
		Ok(_) => debug!("User {} ({}) saved to the database.", user.name, user.id),
		Err(why) => error!("Failed to save user {} ({}): {:?}", user.name, user.id, why),
	}
}

/// Fetch a user from the database by their Discord ID and sets current role base our level
async fn fetch_user_and_set_role(ctx: &Context, guild: &Guild, user_id: &UserId) -> CommandResult<Option<User>> {
	let conn = &mut establish_connection();
	if let Some(mut user) = User::get_by_id(conn, user_id) {
		debug!("Fetched user {} ({}) from the database.", user.username, user.id);

		// Set role based on level
		let role = get_role_by_level(user.level);
		if let Some(role) = role {
			if let Err(why) = add_role(&ctx, guild, user_id, &role.name).await {
				error!("Failed to add role to user {} ({}): {:?}", user.username, user.id, why);
			} else {
				debug!(
					"Assigned role {} to user {} ({}) based on level {}.",
					role.name, user.username, user.id, user.level
				);
			}
		}

		Ok(Some(user))
	} else {
		debug!("User with ID {} not found in the database.", user_id);
		Ok(None)
	}
}
