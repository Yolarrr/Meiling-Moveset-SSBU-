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
use smash::app::KineticEnergy;
use super::*;
use crate::MARKED_COLORS;

pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_SITUATION_KIND: i32 = 0x17;
pub const STATUS_KIND: i32 = 0xB;

pub const WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT :i32 = 0x100000D1;

pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_NEAR: i32 = 0x200000F9;


unsafe extern "C" fn meiling_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::FT_SET_FINAL_SMASH_LIGHT(agent, true);
        //macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 15.0, 300.0);
    }

    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::ATTACK_ABS(agent, *FIGHTER_DOLLY_ATTACK_ABSOLUTE_KIND_FINAL, 0, 1.0, 361, 80, 1, 0, 0.3, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(agent.module_accessor, true, true);
    }
    
}

unsafe extern "C" fn meiling_final_eff(agent: &mut L2CAgentBase) {

}



unsafe extern "C" fn meiling_final_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    //StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    //println!("is this even working");
    0.into()
}

unsafe extern "C" fn meiling_final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_AIR {
        smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        smash::app::lua_bind::KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_air_start"), 0.0, 1.0, false, 0.0, false, false);


        return fighter.sub_shift_status_main(L2CValue::Ptr(meiling_final_main_loop as *const () as _));
    } else {
        let entryself = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let totalplayers = smash::app::lua_bind::FighterManager::entry_count(singletons::FighterManager());
        let posx = PostureModule::pos_x(fighter.module_accessor);
        let posy = PostureModule::pos_y(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        
        let mut opponentinrange = false;
        
        for i in 0..totalplayers {
            if i != entryself as i32 {
                let loopboma = &mut *sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i as i32));
            
                let enemyx = PostureModule::pos_x(loopboma);
                let enemyy = PostureModule::pos_y(loopboma);
                let distx = enemyx - posx;
                let disty = enemyy - posy;
            
                if ((distx < 20.0 && distx > 0.0 && lr == 1.0) || (distx > -20.0 && distx < 0.0 && lr == -1.0)) && (disty > -3.0 && disty < 20.0) { 
                    opponentinrange = true;
                }
            
                
            }
        }
    
        if opponentinrange {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_NEAR);
            //println!("in range");
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2.into(),false.into());
            return 0.into();
        } else {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_NEAR);
            //println!("not in range");
        }
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL.into(),false.into());
        return 0.into();
    }
}

unsafe extern "C" fn meiling_final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !MotionModule::is_end(fighter.module_accessor) {
        notify_event_msc_cmd!(
            fighter,
            *FIGHTER_KINETIC_ENERGY_ID_MOTION,   //  *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, -1);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
    } else {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());
        } else {
            fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into());
        }
    }
    0.into()
}






////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


unsafe extern "C" fn game_superspecial2start(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_START_CUTIN(agent);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 15.0, 90.0);
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 3, 0.5, 0.5, 0.5, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 40);
            macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04finalstart.nuanmb"), true);
        }
    }
    frame(agent.lua_state_agent, 67.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 45, 100, 100, 30, 6.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(6.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.0, 45, 100, 100, 30, 6.0, 0.0, 8.0, 10.0, Some(0.0), Some(8.0), Some(6.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 111.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        macros::CANCEL_FILL_SCREEN(agent, 0, 3);
    }
}


unsafe extern "C" fn game_superspecial2hit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 83, 0, 0, 0, 1.6, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 36.0);
    macros::FT_MOTION_RATE(agent, 10.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_OBJECT_ID);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 45, 0, 0, 0, 9.0, 0.0, 9.5, 9.5, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        //play sound
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 40.0, 83, 46, 0, 74, 1.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_OBJECT_ID);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::ATTACK_IGNORE_THROW(agent, 0, 0, Hash40::new("throw"), 20.0, 362, 46, 0, 68, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 153.0);
    if macros::is_excute(agent) {
        macros::CANCEL_FILL_SCREEN(agent, 0, 3);
    }
    frame(agent.lua_state_agent, 159.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_superspecial2start(agent: &mut L2CAgentBase) {
    let mut hit1: u32 = 0;
    frame(agent.lua_state_agent, 67.0);
    if macros::is_excute(agent) {
        hit1 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_ground_shockwave"), Hash40::new("throw"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:120.0,y:60.0,z:30.0}, 2.7, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
}

unsafe extern "C" fn effect_superspecial2hit(agent: &mut L2CAgentBase) {
    let mut hit2: u32 = 0;
    let mut hit3: u32 = 0;
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        hit2 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_ground_shockwave"), Hash40::new("throw"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:100.0,y:80.0,z:50.0}, 2.7, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        hit3 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_ground_shockwave"), Hash40::new("throw"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:140.0,y:50.0,z:40.0}, 3.3, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
}

unsafe extern "C" fn sound_superspecial2hit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
    frame(agent.lua_state_agent, 36.0);
    //println!("just making sure");
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_ll"));
    }
}





///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


unsafe extern "C" fn final1dash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_START_CUTIN(agent);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 15.0, 90.0);
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 3, 0.5, 0.5, 0.5, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 40);
            macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04finalstart.nuanmb"), true);
        }
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 270, 30, 30, 30, 6.0, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_level(agent.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 30.0, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 78.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 140.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        macros::CANCEL_FILL_SCREEN(agent, 0, 3);
    }
}

unsafe extern "C" fn final1dash_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura"), Hash40::new("hip"), 1, 0, 0, 0, 0, 0, 1.4, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura"), Hash40::new("handr"), 1, 0, 0, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura"), Hash40::new("handl"), 1, 0, 0, 0, 0, 0, 0.7, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura"), Hash40::new("footr"), 1, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura"), Hash40::new("footl"), 1, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura"), Hash40::new("head"), 1, 0, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura"), Hash40::new("kneel"), 1, 0, 0, 0, 0, 0, 0.9, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_final_aura"), Hash40::new("kneer"), 1, 0, 0, 0, 0, 0, 0.9, true);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_final_aura"), false, true);
    }
    frame(agent.lua_state_agent, 140.0);
    if macros::is_excute(agent) {
        
    }
}

unsafe extern "C" fn final1dash_expr(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}





unsafe extern "C" fn final1hit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        JostleModule::set_status(agent.module_accessor, false);
        macros::SLOW_OPPONENT(agent, 10.0, 12.0);
    }
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        macros::SLOW_OPPONENT(agent, 1000.0, 174.0);
    }
    frame(agent.lua_state_agent, 186.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("throw"), 45.0, 100, 36, 0, 62, 40.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_ENERGY);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 217.0);
    if macros::is_excute(agent) {
        macros::CANCEL_FILL_SCREEN(agent, 0, 3);
    }
    frame(agent.lua_state_agent, 233.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, true);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn final1hit_effect(agent: &mut L2CAgentBase) {
    let mut projeffect: u32 = 0;
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        projeffect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_special_all_up"), Hash40::new("throw"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.05, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
    for i in 0..89 {
        if macros::is_excute(agent) {
            EffectModule::set_scale(agent.module_accessor, projeffect, &Vector3f{x:0.01 + (i as f32 * 0.02),y:0.01 + (i as f32 * 0.02),z:0.01 + (i as f32 * 0.02)});
        }
        wait(agent.lua_state_agent, 2.0);
    }
    frame(agent.lua_state_agent, 186.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_special_all_up"), false, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_dead_light"), Hash40::new("throw"), 1, 0, 0, 0, 0, 0, 1.4, true);
    }
}








unsafe extern "C" fn meiling_final1_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn meiling_final1_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("visual_scene_01"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_set_ground_correct_by_situation(true.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(meiling_final1_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_final1_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !MotionModule::is_end(fighter.module_accessor) {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            fighter.change_status((*FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE02).into(), false.into());
        }
    } 
    else {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());
        } else {
            fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn meiling_final1_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}






unsafe extern "C" fn meiling_final2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);

    0.into()
}

unsafe extern "C" fn meiling_final2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("visual_scene_02"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_set_ground_correct_by_situation(true.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(meiling_final2_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_final2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());
        } else {
            fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn meiling_final2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}



////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////




unsafe extern "C" fn airfinal(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_START_CUTIN(agent);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 20.0, 110.0);
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::FILL_SCREEN_MODEL_COLOR(agent, 0, 3, 0.5, 0.5, 0.5, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 40);
            macros::REQ_FINAL_START_CAMERA(agent, Hash40::new("d04finalstart.nuanmb"), true);
        }
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 367, 30, 30, 30, 6.0, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(7.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 8, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 106.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 107.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    }
    frame(agent.lua_state_agent, 140.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        macros::CANCEL_FILL_SCREEN(agent, 0, 3);
    }
}

unsafe extern "C" fn airfinal_effect(agent: &mut L2CAgentBase) {
    let mut projeffect1 = 0;
    let mut projeffect2 = 0;
    let mut projeffect3 = 0;
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        projeffect1 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_final_aura"), Hash40::new("top"), &Vector3f{x:0.0,y:8.0,z:10.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.0 /* size */, true, 0, 0, 0, 0, 0, true, true) as u32;
        projeffect2 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_final_aura"), Hash40::new("top"), &Vector3f{x:0.0,y:8.0,z:10.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.0 /* size */, true, 0, 0, 0, 0, 0, true, true) as u32;
        projeffect3 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_final_aura"), Hash40::new("top"), &Vector3f{x:0.0,y:8.0,z:10.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.0 /* size */, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
    for i in 0..30 {
        if macros::is_excute(agent) {
            EffectModule::set_scale(agent.module_accessor, projeffect1, &Vector3f{x:0.2 + (i as f32 * 0.07),y:0.2 + (i as f32 * 0.07),z:0.2 + (i as f32 * 0.07)});
            EffectModule::set_scale(agent.module_accessor, projeffect2, &Vector3f{x:0.2 + (i as f32 * 0.07),y:0.2 + (i as f32 * 0.07),z:0.2 + (i as f32 * 0.07)});
            EffectModule::set_scale(agent.module_accessor, projeffect3, &Vector3f{x:0.2 + (i as f32 * 0.07),y:0.2 + (i as f32 * 0.07),z:0.2 + (i as f32 * 0.07)});
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 106.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 107.0);
    if macros::is_excute(agent) {
        EffectModule::kill(agent.module_accessor, projeffect1, false, false);
        EffectModule::kill(agent.module_accessor, projeffect2, false, false);
        EffectModule::kill(agent.module_accessor, projeffect3, false, false);
    }
    frame(agent.lua_state_agent, 140.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        macros::CANCEL_FILL_SCREEN(agent, 0, 3);
    }
}






unsafe extern "C" fn airfinal_bullet(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.2, 366, 20, 70, 60, 17.5, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.2, 9, 20, 70, 50, 11.5, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, f32::NAN, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
}


unsafe extern "C" fn airfinal_bullet2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 14.2, 68, 50, 0, 100, 18.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
}





pub unsafe extern "C" fn airfinal_bullet_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let ownerboma = smash::app::sv_battle_object::module_accessor(owner_id);
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_KEEP as u32, //*GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32,
    );
    0.into()
}

pub unsafe extern "C" fn airfinal_bullet_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let ownerboma = smash::app::sv_battle_object::module_accessor(owner_id);
    let lr = PostureModule::lr(ownerboma);

    let newPos = Vector3f{
        x: PostureModule::pos_x(ownerboma) + (10.0*lr), 
        y: PostureModule::pos_y(ownerboma) + 8.0, 
        z: PostureModule::pos_z(ownerboma)
    };
    PostureModule::set_pos(weapon.module_accessor, &newPos);
    0.into()
}

pub unsafe extern "C" fn airfinal_bullet_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let ownerboma = smash::app::sv_battle_object::module_accessor(owner_id);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("final_meiling"), 0.0, 1.0, false, 0.0, false, false);
    MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_GAME, Hash40::new("game_final_meiling"), -1);

    EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_final_aura"), Hash40::new("top"), &Vector3f{x:0.0,y:8.0,z:10.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 4.8 /* size */, true, 0, 0, 0, 0, 0, true, true) as u32;
    EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_final_aura"), Hash40::new("top"), &Vector3f{x:0.0,y:8.0,z:10.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 4.8 /* size */, true, 0, 0, 0, 0, 0, true, true) as u32;
    EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_final_aura"), Hash40::new("top"), &Vector3f{x:0.0,y:8.0,z:10.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 4.8 /* size */, true, 0, 0, 0, 0, 0, true, true) as u32;

    let life = 53;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT);

    WorkModule::off_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);

    weapon.fastshift(L2CValue::Ptr(airfinal_bullet_main_loop as *const () as _))
}


unsafe extern "C" fn airfinal_bullet_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let lr = PostureModule::lr(owner);  
    let selflr = PostureModule::lr(weapon.module_accessor);
    
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let mut life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let maxlife = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
        WorkModule::inc_int(weapon.module_accessor, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT);
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            0.0
        );
    } else {
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            1.6*selflr,
            0.0
        );
    }

    if life == 4 {
        MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_GAME, Hash40::new("game_final_meiling2"), -1);
        EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_dead_light"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.3 /* size */, true, 0, 0, 0, 0, 0, true, true) as u32;
    }
    if life < 0 {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }

    if !AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
        life = maxlife
    }

    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);


    0.into()
}
pub unsafe extern "C" fn airfinal_bullet_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
pub unsafe extern "C" fn airfinal_bullet_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
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
        .status(Pre, *FIGHTER_STATUS_KIND_FINAL, meiling_final_pre)
        .status(Main, *FIGHTER_STATUS_KIND_FINAL, meiling_final_main)

        .game_acmd("game_superspecial2start_meiling", game_superspecial2start, Default)
        .game_acmd("game_superspecial2_meiling", game_superspecial2hit, Default)
        .effect_acmd("effect_superspecial2start_meiling", effect_superspecial2start, Default)
        .effect_acmd("effect_superspecial2_meiling", effect_superspecial2hit, Default)
        .sound_acmd("sound_superspecial2_meiling", sound_superspecial2hit, Default)

        .game_acmd("game_visualscene01_meiling", final1dash, Default)
        .effect_acmd("effect_visualscene01_meiling", final1dash_effect, Default)
        .expression_acmd("expression_visualscene01_meiling", final1dash_expr, Default)
        .game_acmd("game_visualscene02_meiling", final1hit, Default)
        .effect_acmd("effect_visualscene02_meiling", final1hit_effect, Default)

        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, meiling_final1_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, meiling_final1_main)
        .status(End, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, meiling_final1_end)
        
        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE02, meiling_final2_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE02, meiling_final2_main)
        .status(End, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE02, meiling_final2_end)

        .game_acmd("game_finalairstart_meiling", airfinal, Default)
        .effect_acmd("effect_finalairstart_meiling", airfinal_effect, Default)

        .install();
    Agent::new("dolly_burst")
    .set_costume(costume.to_vec())
        .game_acmd("game_final_meiling", airfinal_bullet, Default)
        .game_acmd("game_final_meiling2", airfinal_bullet2, Default)

        .status(Pre, *WEAPON_DOLLY_BURST_STATUS_KIND_REGULAR, airfinal_bullet_pre)
        .status(Init, *WEAPON_DOLLY_BURST_STATUS_KIND_REGULAR, airfinal_bullet_init)
        .status(Main, *WEAPON_DOLLY_BURST_STATUS_KIND_REGULAR, airfinal_bullet_main)
        .status(End, *WEAPON_DOLLY_BURST_STATUS_KIND_REGULAR, airfinal_bullet_end)
        .install();
}