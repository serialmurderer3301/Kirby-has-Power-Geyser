#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

//mod asriel;
//mod joe;
mod kirby;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    //asriel::install();
	//joe::install();
	kirby::install();
	smashline::whitelist_kirby_copy_article(*smash::lib::lua_const::FIGHTER_KIND_DOLLY, *smash::lib::lua_const::WEAPON_KIND_DOLLY_BURST);
}