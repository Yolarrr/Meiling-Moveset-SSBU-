use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};
use super::*;
use crate::MARKED_COLORS;

const FIGHTER_TERRY_COMBO_HIT : i32 = 0x200000F0;

pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DAIR_EFFECT: i32 = 0x200000D2;

unsafe extern "C" fn dolly_game_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 62, 80, 0, 20, 4.5, 0.0, 6.0, 7.0, Some(0.0), Some(8.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 62, 80, 0, 20, 4.5, 0.0, 6.0, 7.0, Some(0.0), Some(8.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}



unsafe extern "C" fn meiling_fair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        let mut speed_x = smash::app::sv_kinetic_energy::get_speed_x(agent.lua_state_agent);
        let mut speed_y = smash::app::sv_kinetic_energy::get_speed_y(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 1.7, 0.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 15.0);
    
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 55, 70, 0, 70, 3.5, 0.0, 6.0, 4.7, Some(0.0), Some(10.7), Some(10.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }

}



unsafe extern "C" fn meiling_bair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::REVERSE_LR(agent);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 1.7, 0.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 15.0);
    
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 55, 70, 0, 70, 3.5, 0.0, 6.0, -4.7, Some(0.0), Some(10.7), Some(-10.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }

}



unsafe extern "C" fn meiling_upair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::SET_SPEED_EX(agent, 0.15, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 10.0);
    
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 88, 80, 30, 0, 3.7, 0.0, 11.3, 6.2, Some(0.0), Some(13.6), Some(9.2), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 367, 80, 30, 0, 3.7, 0.0, 14.7, 10.7, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 282, 50, 0, 40, 4, 0.0, 4.0, 4.0, Some(0.0), Some(9.5), Some(9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 282, 50, 0, 40, 3, 0.0, 10.0, 9.5, Some(0.0), Some(13.3), Some(8.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 0.3);
    frame(agent.lua_state_agent, 48.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}



unsafe extern "C" fn meiling_dair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 12.0);
    
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 79, 160, 30, 0, 4.0, 0.0, 0.3, -7.0, Some(0.0), Some(1.0), Some(6.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 75, 160, 30, 0, 3.0, 0.0, 2.0, 6.0, Some(0.0), Some(4.5), Some(8.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 11.0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 60, 55, 0, 77, 4.5, 0.0, 14.0, 6.0, Some(0.0), Some(17.2), Some(10.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_dair_effect(agent: &mut L2CAgentBase) {
    let mut projeffect: u32 = 0;
    frame(agent.lua_state_agent, 4.0);
    for i in 0..23 {
        if macros::is_excute(agent) {
            projeffect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_status_defense_up"), Hash40::new("throw"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.18, true, 0, 0, 0, 0, 0, true, true) as u32;
            WorkModule::set_int(agent.module_accessor, projeffect as i32, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DAIR_EFFECT);
        }
        wait(agent.lua_state_agent, 1.0);
        EffectModule::kill(agent.module_accessor, projeffect, false, false);
    }
}



unsafe extern "C" fn dolly_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let mut isaerial = false;
        let xpos = ControlModule::get_stick_x(fighter.module_accessor);
        let ypos = ControlModule::get_stick_y(fighter.module_accessor);
        
                if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n") && 
                    (AttackModule::is_infliction_status(fighter.module_accessor, (*COLLISION_KIND_MASK_HIT)) ||
                    AttackModule::is_infliction_status(fighter.module_accessor, (*COLLISION_KIND_MASK_SHIELD))) &&
                    ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && 
                    (((xpos > -0.5 && xpos < 0.5) && (ypos > 0.5)) || ((xpos > -0.5 && xpos < 0.5) && (ypos < -0.5)) || 
                    (((xpos > 0.5) && (ypos > -0.5 && ypos < 0.5)) && (PostureModule::lr(fighter.module_accessor) == 1.0)) ||
                    (((xpos < -0.5) && (ypos > -0.5 && ypos < 0.5)) && (PostureModule::lr(fighter.module_accessor) == -1.0))) {
                        if ((xpos > -0.5 && xpos < 0.5) && (ypos > 0.5)) { ControlModule::set_attack_air_kind(fighter.module_accessor,*FIGHTER_COMMAND_ATTACK_AIR_KIND_HI); }
                        if ((xpos > -0.5 && xpos < 0.5) && (ypos < -0.5)) { ControlModule::set_attack_air_kind(fighter.module_accessor,*FIGHTER_COMMAND_ATTACK_AIR_KIND_LW); }
                        if ((((xpos > 0.5) && (ypos > -0.5 && ypos < 0.5)) && (PostureModule::lr(fighter.module_accessor) == 1.0)) ||
                        (((xpos < -0.5) && (ypos > -0.5 && ypos < 0.5)) && (PostureModule::lr(fighter.module_accessor) == -1.0))) { 
                            ControlModule::set_attack_air_kind(fighter.module_accessor,*FIGHTER_COMMAND_ATTACK_AIR_KIND_F); 
                        }
                        isaerial = true;
                        WorkModule::on_flag(fighter.module_accessor, FIGHTER_TERRY_COMBO_HIT);
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
                }
            
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_TERRY_COMBO_HIT) && MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_n") && isaerial
            {
                        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(),false.into());
                        WorkModule::off_flag(fighter.module_accessor, FIGHTER_TERRY_COMBO_HIT);
            }
        
        if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_n") {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_TERRY_COMBO_HIT);
        }
    }
}


unsafe extern "C" fn meiling_nair_land(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.8);
}

unsafe extern "C" fn meiling_fair_land(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 2.0);
}

unsafe extern "C" fn meiling_upair_land(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 3.4);
}

unsafe extern "C" fn meiling_dair_land(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 2.0);
}



unsafe extern "C" fn empty_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {

    }
}

unsafe extern "C" fn sound_nair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_swing_s"));
    }
}

unsafe extern "C" fn sound_fair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_swing_m"));
    }
}

unsafe extern "C" fn sound_uair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_swing_s"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_swing_m"));
    }
}

unsafe extern "C" fn sound_dair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_swing_s"));
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_swing_s"));
    }
}



pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }

    Agent::new("dolly")
    .set_costume(costume.to_vec())
        .game_acmd("game_attackairn_meiling", dolly_game_attackairn, Default)
        .game_acmd("game_attackairf_meiling", meiling_fair, Default)
        .game_acmd("game_attackairb_meiling", meiling_bair, Default)
        .game_acmd("game_attackairhi_meiling", meiling_upair, Default)
        .game_acmd("game_attackairlw_meiling", meiling_dair, Default)

        .game_acmd("game_landingairn_meiling", meiling_nair_land, Default)
        .game_acmd("game_landingairf_meiling", meiling_fair_land, Default)
        //.game_acmd("game_landingairb_meiling", meiling_bair, Default)
        .game_acmd("game_landingairhi_meiling", meiling_upair_land, Default)
        .game_acmd("game_landingairlw_meiling", meiling_dair_land, Default)

        .effect_acmd("effect_attackairn_meiling", empty_eff, Default)
        .effect_acmd("effect_attackairf_meiling", empty_eff, Default)
        .effect_acmd("effect_attackairb_meiling", empty_eff, Default)
        .effect_acmd("effect_attackairhi_meiling", empty_eff, Default)
        .effect_acmd("effect_attackairlw_meiling", meiling_dair_effect, Default)

        .sound_acmd("sound_attackairn_meiling", sound_nair, Default)
        .sound_acmd("sound_attackairf_meiling", sound_fair, Default)
        .sound_acmd("sound_attackairb_meiling", sound_fair, Default)
        .sound_acmd("sound_attackairhi_meiling", sound_uair, Default)
        .sound_acmd("sound_attackairlw_meiling", sound_dair, Default)

        .on_line(Main, dolly_frame)
        .install();
}
