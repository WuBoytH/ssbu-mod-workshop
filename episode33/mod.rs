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

#[acmd_script( agent = "lucina", script = "expression_attacklw3", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_expression_attacklw3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6);
		ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, false, *ATTACH_ITEM_GROUP_ALL as u8);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 12);
		ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(agent.module_accessor, true, *ATTACH_ITEM_GROUP_ALL as u8);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        lucina_expression_attacklw3,
    );
}
