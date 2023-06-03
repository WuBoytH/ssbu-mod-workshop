use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*
};

#[fighter_frame( agent = FIGHTER_KIND_LUCARIO )]
fn lucario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma1 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        let boma2 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
		
		if (PostureModule::lr(boma1) == PostureModule::lr(boma2)){
			AttackModule::set_power_up(fighter.module_accessor, 2.0);
		} else {
			AttackModule::set_power_up(fighter.module_accessor, 1.0);
		}
		
	}
}

pub fn install() {
	smashline::install_agent_frames!(
		lucario_frame
    );
}