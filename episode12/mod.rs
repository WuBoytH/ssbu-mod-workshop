use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_ELIGHT )]
fn elight_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		
		if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s3s") || 
		MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3") || 
		MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_lw3") {
			
			if MotionModule::frame(fighter.module_accessor) > 10.0 && MotionModule::frame(fighter.module_accessor) < 20.0 {
				
				if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
					
					fighter.change_status(FIGHTER_STATUS_KIND_JUMP.into(), false.into());
					
				}
				
			}
			
		} else if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b") || 
					MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_f") || 
					MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") || 
					MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") || 
					MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {

			// Removes 20 frames of endlag by allowing a cancel if the current frame is 20 frames or fewer away from the normal cancel frame
			if FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)), false) - MotionModule::frame(fighter.module_accessor) <= 20.0 {
				
				CancelModule::enable_cancel(fighter.module_accessor);
				
			}
			
		}
		
	}
	
}

#[acmd_script( agent = "elight", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn elight_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 3.0);
	// motion_rate originally 1.0
    macros::FT_MOTION_RATE(agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 20, 0, 15, 2.5, 0.0, 8.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 20, 0, 15, 2.5, 0.0, 8.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 180, 15, 0, 15, 2.5, 0.0, 8.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 180, 15, 0, 15, 2.5, 0.0, 8.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 7.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 7.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 7.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 7.0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        elight_attack11,
    );
	smashline::install_agent_frames!(
		elight_frame,
    );
}
