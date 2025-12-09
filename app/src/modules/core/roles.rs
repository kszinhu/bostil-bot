use lazy_static::lazy_static;
use serenity::all::EditRole;

// Nível 1-10: Eleitor
// Nível 11-20: Cabo Eleitoral
// Nível 21-30: Vereador
// Nível 31-40: Prefeito
// Nível 41-50: Deputado
// Nível 51-60: Senador
// Nível 61-70: Governador
// Nível 71-80: Ministro
// Nível 81-90: Vice-Presidente
// Nível 91-100: Presidente (da zueira)

pub const ELECTOR: &str = "Eleitor";
pub const ELECTOR_CABO: &str = "Cabo Eleitoral";
pub const COUNCILMAN: &str = "Vereador";
pub const MAYOR: &str = "Prefeito";
pub const DEPUTY: &str = "Deputado";
pub const SENATOR: &str = "Senador";
pub const GOVERNOR: &str = "Governador";
pub const MINISTER: &str = "Ministro";
pub const VICE_PRESIDENT: &str = "Vice-Presidente";
pub const PRESIDENT: &str = "Presidente";

lazy_static! {
	pub static ref ELECTOR_ROLE: EditRole<'static> = EditRole::new().name(ELECTOR).hoist(false).mentionable(false);
	pub static ref ELECTOR_CABO_ROLE: EditRole<'static> =
		EditRole::new().name(ELECTOR_CABO).hoist(false).mentionable(false);
	pub static ref COUNCILMAN_ROLE: EditRole<'static> = EditRole::new().name(COUNCILMAN).hoist(false).mentionable(false);
	pub static ref MAYOR_ROLE: EditRole<'static> = EditRole::new().name(MAYOR).hoist(false).mentionable(false);
	pub static ref DEPUTY_ROLE: EditRole<'static> = EditRole::new().name(DEPUTY).hoist(false).mentionable(false);
	pub static ref SENATOR_ROLE: EditRole<'static> = EditRole::new().name(SENATOR).hoist(false).mentionable(false);
	pub static ref GOVERNOR_ROLE: EditRole<'static> = EditRole::new().name(GOVERNOR).hoist(false).mentionable(false);
	pub static ref MINISTER_ROLE: EditRole<'static> = EditRole::new().name(MINISTER).hoist(false).mentionable(false);
	pub static ref VICE_PRESIDENT_ROLE: EditRole<'static> =
		EditRole::new().name(VICE_PRESIDENT).hoist(false).mentionable(false);
	pub static ref PRESIDENT_ROLE: EditRole<'static> = EditRole::new().name(PRESIDENT).hoist(false).mentionable(false);
}

pub fn get_all_roles() -> Vec<(&'static str, &'static EditRole<'static>)> {
	vec![
		(ELECTOR, &ELECTOR_ROLE),
		(ELECTOR_CABO, &ELECTOR_CABO_ROLE),
		(COUNCILMAN, &COUNCILMAN_ROLE),
		(MAYOR, &MAYOR_ROLE),
		(DEPUTY, &DEPUTY_ROLE),
		(SENATOR, &SENATOR_ROLE),
		(GOVERNOR, &GOVERNOR_ROLE),
		(MINISTER, &MINISTER_ROLE),
		(VICE_PRESIDENT, &VICE_PRESIDENT_ROLE),
		(PRESIDENT, &PRESIDENT_ROLE),
	]
}

pub fn get_role_by_level(level: i32) -> Option<&'static EditRole<'static>> {
	match level {
		1..=10 => Some(&ELECTOR_ROLE),
		11..=20 => Some(&ELECTOR_CABO_ROLE),
		21..=30 => Some(&COUNCILMAN_ROLE),
		31..=40 => Some(&MAYOR_ROLE),
		41..=50 => Some(&DEPUTY_ROLE),
		51..=60 => Some(&SENATOR_ROLE),
		61..=70 => Some(&GOVERNOR_ROLE),
		71..=80 => Some(&MINISTER_ROLE),
		81..=90 => Some(&VICE_PRESIDENT_ROLE),
		91..=100 => Some(&PRESIDENT_ROLE),
		_ => None,
	}
}
