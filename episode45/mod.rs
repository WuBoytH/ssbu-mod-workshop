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
use smash::lib::L2CValue;

pub const KIND:						i32 = 0x2;
pub const CMD_CAT4:					i32 = 0x23;
pub const CHECK_SPECIAL_COMMAND:	i32 = 0x3C;

const FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND : i32 = 0x200000f0;

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
	unsafe {
		let fighter_kind = fighter.global_table[KIND].get_i32();
		if fighter_kind == *FIGHTER_KIND_WOLF {
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
			fighter.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(wolf_check_special_command as *const () as _));
		}
	}
}

unsafe extern "C" fn wolf_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
	let mut ret = false;
	let cat4 = fighter.global_table[CMD_CAT4].get_i32();
	
	// Side B
	if !ret && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND != 0
	&& WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
		WorkModule::on_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND);
		fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
		ret = true;
	}
	
	// Up B
	if !ret && cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND != 0
	&& WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND) {
		fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
		ret = true;
	}
	
	// Return bool
	ret.into()
}

#[status_script(agent = "wolf", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn wolf_special_s_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND) {
		AttackModule::set_power_up(fighter.module_accessor, 1.5);
		WorkModule::off_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SPECIAL_S_COMMAND)
	}
	original!(fighter)
}

pub fn install() {
	install_agent_init_callback!(
		agent_init
	);
	install_status_scripts!(
		wolf_special_s_start_main,
	);
}

// List from black_calculus & LilyLambda, with help from WuBoy
// All of these can be inputted in the air. Ones with back inputs typically don't work well for characters without autoturnaround

/*
// Ryu/Ken/Terry Command Inputs
	FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND = Quarter Circle Forward, Hadoken [ 236A/B ]
	FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND = Half Circle Forward, Shakunetsu Hadoken [ 41236A/B ]
	FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND = Quarter Circle Back, Tatsumaki Senpukyaku [ 214A/B ]
	FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND = Z input, Shoryuken [ 623A/B ]
	FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND = Charge down -> Up, Rising Tackle [ [2]8A/B ]
	FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND = Dobule Quarter Circle Forward, Buster Wolf [ 236236A/B ]
	FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND = Dobule Quarter Circle Back, Buster Wolf [ 214214A/B ]
	FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND = Half Circle Back -> Forward, Power Geyser [ 21416A/B, 2426A/B, 2146A/B ]
	FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND = Half Circle Forward -> Back, Power Geyser [ 23634A/B, 2624A/B, 2364A/B ]
	FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1 = Quarter circle down, Nata Otoshi Geri [ 632A/B ]

// Kazuya Command Inputs
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6AB = Double tap forward, Left Splits Kick [ 656A ]
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623NB = Crouch Dash [ 623 ]
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_323CATCH = Gates of Hell [ 323Z ]
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A = Wind God Fist [ 623A ]
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT = Electric Wind God Fist [ 623#A ]
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG = Dragon Uppercut / Rage Drive [ 623[A] ]
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG = Rage Drive [ 623[B] ]

// Other
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6 = Double tap forward (no buttons) [ 656 ]
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_4N4 = Double tap backward (no buttons) [ 454 ]

// Numpad Inputs (Don't work)
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_1 = Numpad 1, Kazuya's Stature Smash / Crouch Spin Kick
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_2 = Numpad 2, Kazuya's Neijiri Uraken / Crouch Jab
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_3 = Numpad 3, Kazuya's Tsunami Kick / Tombstone Crusher
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_4 = Numpad 4, Kazuya's Flash Tornado
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6 = Numpad 6, Kazuya's Oni Front Kick
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_7 = Numpad 7, Kazuya's Jump Side Kick
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_8 = Numpad 8, Kazuya's Twin Pistons
	FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_9 = Numpad 9, Kazuya's Roundhouse to Triple Spin Kicks
*/
