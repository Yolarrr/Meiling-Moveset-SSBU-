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
use smash::app;
use super::*;
use crate::MARKED_COLORS;

pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_SITUATION_KIND: i32 = 0x17;
pub const STATUS_KIND: i32 = 0xB;

pub static mut FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET :i32 = 0x4;
pub static mut FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET2 :i32 = 0x4;
pub static mut FIGHTER_DOLLY_GENERATE_ARTICLE_5CBULLET :i32 = 0x4;


//pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT :i32 = 0x200000E9;
//pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_ANGLE :i32 = 0x200000E6;
//pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND: i32 = 0x200000EB;

//pub const WEAPON_FALCO_BLASTER_BULLET_INSTANCE_WORK_ID_INT_6BBULLET_COUNT :i32 = 0x1000000B;

pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT :i32 = 0x100000CC;
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_ANGLE :i32 = 0x100000CD;
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLOAT_6CBULLET_ANGLE :i32 = 0x100000CE;
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_5CBULLET_ANGLE :i32 = 0x100000D0;

pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND: i32 = 0x200000EE;
//pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_NEAR: i32 = 0x200000F0;

pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DASH_ATTACK_B: i32 = 0x200000EF;
pub const FIGHTER_TERRY_COMBO_HIT : i32 = 0x200000F0;
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_NEAR: i32 = 0x200000F9;

pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DAIR_EFFECT: i32 = 0x200000D2;

pub const WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_NUM :i32 = 0x100000CF;
pub const WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT :i32 = 0x100000D1;

pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_B_USED: i32 = 0x200000F1;
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_C_USED: i32 = 0x200000F2;



unsafe extern "C" fn meiling_neutralb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 40, 70, 0, 70, 6.0, 0.0, 8.0, 8.0, Some(0.0), Some(6.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_neutralb_air(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    macros::SET_SPEED_EX(agent, -1.0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 40, 70, 0, 70, 6.0, 0.0, 10.0, 9.0, Some(0.0), Some(8.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_neutralb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        let projeffect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_muzzleflash"), Hash40::new("top"), &Vector3f{x:0.0,y:8.0,z:8.0}, &Vector3f{x:20.0,y:0.0,z:0.0}, 1.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(agent.module_accessor, projeffect, 0.6, 0.94, 1.0);
        macros::LAST_EFFECT_SET_RATE(agent, 0.9);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
}

unsafe extern "C" fn meiling_neutralb_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swim_high_01"));
    }
}

unsafe extern "C" fn meiling_neutralb_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_vector"), true, true);
    return 0.into();
}


unsafe extern "C" fn meiling_neutralb_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}




unsafe extern "C" fn meiling_sideb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    }
}

unsafe extern "C" fn meiling_sideb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn meiling_sideb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    fighter.sub_change_motion_by_situation(Hash40::new("special_f_start").into(), Hash40::new("special_air_f_start").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);

    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_ANGLE);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);

    fighter.sub_shift_status_main(L2CValue::Ptr(meiling_sideb_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_sideb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bulletcount = 0;
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !MotionModule::is_end(fighter.module_accessor) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE) {
                //if (WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT) < 7) {
                    ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET, false, -1);
                    WorkModule::inc_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
                    bulletcount = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);

                    ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET2, false, -1);
                    WorkModule::inc_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
                    bulletcount = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);

                    ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET, false, -1);
                    WorkModule::inc_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
                    bulletcount = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
                    
                    ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET2, false, -1);
                    WorkModule::inc_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
                    bulletcount = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);

                    ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET, false, -1);
                    WorkModule::inc_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
                    bulletcount = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);

                    ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET2, false, -1);
                    WorkModule::inc_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
                    bulletcount = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);

                    ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET, false, -1);
                    WorkModule::inc_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
                    bulletcount = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);


                //} else {
                    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                        //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
                    }
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
                //}
            }
            fighter.sub_change_motion_by_situation(hash40("special_f_start").into(), hash40("special_air_f_start").into(), true.into());
            fighter.sub_set_ground_correct_by_situation(true.into());
        }
        else {
            if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());
            } else {
                fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into());
            }
        }

        if (fighter.global_table[0x16] != *SITUATION_KIND_GROUND) && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_B_USED) {
            let speedy = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            notify_event_msc_cmd!( //prob wrong
                fighter,
                *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                //ENERGY_STOP_RESET_TYPE_AIR,
                0.0,
                -1.5,      //important
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_accel"));

            sv_kinetic_energy::set_accel(fighter.lua_state_agent);

            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }

    }
    else {
        fighter.sub_wait_ground_check_common(false.into());
    }
    0.into()
}

unsafe extern "C" fn meiling_sideb_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_B_USED);
    0.into()
}





























unsafe extern "C" fn meiling_6bbullet_game_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.5, 361, 100, 0, 20, 1.4, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_6bbullet_effect_regular(agent: &mut L2CAgentBase) {
    let bulletnum = WorkModule::get_int(agent.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
    if macros::is_excute(agent) {
        
    }
}




pub unsafe extern "C" fn meiling_6bbullet_start_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let num_bullet = WorkModule::get_int(owner, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);
    let startangle = WorkModule::get_int(owner, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_ANGLE);
    let angle = ((num_bullet) * 10) + 330 + startangle;

    WorkModule::set_int(weapon.module_accessor, angle, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_ANGLE);
    WorkModule::set_int(weapon.module_accessor, num_bullet, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_NUM);

    let mut owner_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let lr = PostureModule::lr(owner);    
    let newPos = Vector3f{x: PostureModule::pos_x(owner) + (6.0 * lr), y: PostureModule::pos_y(owner) + 10.0, z: PostureModule::pos_z(owner)};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    if num_bullet == 0 {
        let projeffect = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(weapon.module_accessor, projeffect, 1.0, 0.0, 0.0);
    }
    if num_bullet == 1 {
        let projeffect = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(weapon.module_accessor, projeffect, 1.0, 0.3, 0.0);
    }
    if num_bullet == 2 {
        let projeffect = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(weapon.module_accessor, projeffect, 0.9, 1.0, 0.0);
    }
    if num_bullet == 3 {
        let projeffect = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(weapon.module_accessor, projeffect, 0.0, 1.0, 0.0);
    }
    if num_bullet == 4 {
        let projeffect = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(weapon.module_accessor, projeffect, 0.0, 1.0, 1.0);
    }
    if num_bullet == 5 {
        let projeffect = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(weapon.module_accessor, projeffect, 0.0, 0.0, 1.0);
    }
    if num_bullet == 6 {
        let projeffect = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rgb(weapon.module_accessor, projeffect, 1.0, 0.0, 0.5);
    }

    0.into()
}

pub unsafe extern "C" fn meiling_6bbullet_start_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_ATTACK as i32,
    );
    0.into()
}

pub unsafe extern "C" fn meiling_6bbullet_start_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let life = 1;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stay"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(meiling_6bbullet_start_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_6bbullet_start_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    
    if life < 0 {
        StatusModule::change_status_force(weapon.module_accessor, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_STAY, false);
        return 0.into();
    }
    0.into()
}


pub unsafe extern "C" fn meiling_6bbullet_move_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32,
    );
    0.into()
}

pub unsafe extern "C" fn meiling_6bbullet_move_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let angle = WorkModule::get_int(weapon.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_ANGLE) as f32;
    let num_bullet = WorkModule::get_int(weapon.module_accessor, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_NUM) as f32;
    
    let lr = PostureModule::lr(weapon.module_accessor);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let num_speed = 2.0 - ((num_bullet - 3.0).abs() * 0.15);
    
    let speed_x = (angle.to_radians()).cos() * num_speed * 0.95;
    let speed_y = (angle.to_radians()).sin() * num_speed * 0.9;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x*lr,
        speed_y
    );

    0.into()
}

pub unsafe extern "C" fn meiling_6bbullet_move_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let life = 50;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    WorkModule::off_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);
    
    if StopModule::is_stop(weapon.module_accessor){
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }

    weapon.fastshift(L2CValue::Ptr(meiling_6bbullet_move_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_6bbullet_move_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let effvanilla = WorkModule::get_int(weapon.module_accessor, *WEAPON_FALCO_BLASTER_BULLET_INSTANCE_WORK_ID_INT_EFFECT_ID);
    effect!(weapon, MA_MSC_EFFECT_REMOVE, effvanilla, 1);

    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x = smash::app::lua_bind::KineticEnergy::get_speed_x(energy_type);
    let mut speed_y = smash::app::lua_bind::KineticEnergy::get_speed_y(energy_type);

    if life == 42 {
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed_x*0.15,
            speed_y*0.15
        );
    }
    if life < 0 {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }

    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
    {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    0.into()
}
pub unsafe extern "C" fn meiling_6bbullet_move_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
pub unsafe extern "C" fn meiling_6bbullet_move_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}











unsafe extern "C" fn meiling_downb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 89, 160, 50, 0, 5.0, 0.0, 5.0, 20.0, Some(0.0), Some(5.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 89, 75, 0, 70, 5.0, 0.0, 10.0, 20.0, Some(0.0), Some(10.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 89, 75, 0, 70, 5.0, 0.0, 9.0, 15.5, Some(0.0), Some(32.0), Some(15.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_downb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_score_aura"), Hash40::new("top"), 17.0, 0.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.7, 1.0, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 1.7);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_vector"), Hash40::new("top"), 17.0, 0.0, 0.0, 270, 0, 0, 4.7, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 0.0, 1.0, 0.0);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
}

unsafe extern "C" fn meiling_downb_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swim_high_01"));
    }
}

unsafe extern "C" fn meiling_downb_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}



unsafe extern "C" fn meiling_downb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING.into(),false.into());
    } else {
        fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
        fighter.sub_change_motion_by_situation(Hash40::new("special_f_start").into(), Hash40::new("special_air_f_start").into(), false.into());
        fighter.sub_set_ground_correct_by_situation(true.into());

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);

        WorkModule::set_int(fighter.module_accessor, -30, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_ANGLE);
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_6BBULLET_COUNT);

        return fighter.sub_shift_status_main(L2CValue::Ptr(meiling_sideb_main_loop as *const () as _));
    }
    return 0.into();
}

unsafe extern "C" fn meiling_downb_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_vector"), true, true);
    return 0.into();
}

unsafe extern "C" fn meiling_downb_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_vector"), true, true);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_B_USED);
    return 0.into();
}




unsafe extern "C" fn meiling_qcf_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);

    0.into()
}

unsafe extern "C" fn meiling_qcf_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(),false.into());
    }
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_f_attack"), 0.0, 1.0, false, 0.0, false, false);
    } else {        
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_f_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(meiling_qcf_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_qcf_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    if situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND {
        return 0.into();
    }
    return 0.into();
}

unsafe extern "C" fn meiling_qcf_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_vector"), true, true);
    return 0.into();
}

unsafe extern "C" fn meiling_qcf_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_vector"), true, true);
    return 0.into();
}


unsafe extern "C" fn meiling_qcf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 10.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(14.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 8.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(31.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 10.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(14.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 8.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(31.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 60, 100, 0, 75, 10.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 60, 100, 0, 75, 8.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(31.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_qcf_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_score_aura"), Hash40::new("top"), 10.0, 10.0, 0.0, 90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 2.7);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        //for _ in 0..4 {
            macros::EFFECT(agent, Hash40::new("sys_vector"), Hash40::new("top"), 10.0, 10.0, 0.0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1000.0, 0.7, 0.0);
            macros::LAST_EFFECT_SET_RATE(agent, 1.9);
        //}
    } //macros::LAST_EFFECT_SET_RATE
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_score_aura"), Hash40::new("top"), 10.0, 10.0, 0.0, 90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 2.7);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        //for _ in 0..4 {
            macros::EFFECT(agent, Hash40::new("sys_vector"), Hash40::new("top"), 10.0, 10.0, 0.0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 0.7, 1.0, 0.0);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        //}
    } //macros::LAST_EFFECT_SET_RATE
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_score_aura"), Hash40::new("top"), 10.0, 10.0, 0.0, 90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 2.7);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        //for _ in 0..4 {
            macros::EFFECT(agent, Hash40::new("sys_vector"), Hash40::new("top"), 10.0, 10.0, 0.0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.7, 0.0);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        //}
    } //macros::LAST_EFFECT_SET_RATE
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
}

unsafe extern "C" fn meiling_qcf_heavy(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 12.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(14.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 9.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(37.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 12.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(14.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 9.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(37.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 12.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(14.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 9.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(37.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 84.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 12.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(14.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 366, 10, 10, 10, 9.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(37.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 91.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 60, 100, 0, 80, 12.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 100, 0, 80, 9.0, 0.0, 8.5, 20.0, Some(0.0), Some(8.5), Some(37.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 3.0, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_qcf_heavy_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_score_aura"), Hash40::new("top"), 10.0, 10.0, 0.0, 90, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 2.7);
    }
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        //for _ in 0..4 {
            let beameffect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_vector"), Hash40::new("top"), &Vector3f{x:0.0,y:10.0,z:10.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 3.5, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(agent.module_accessor, beameffect, 1.0, 0.8, 0.0);
            EffectModule::set_rate(agent.module_accessor, beameffect, 1.3);
        //}
    } //macros::LAST_EFFECT_SET_RATE
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
    frame(agent.lua_state_agent, 68.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_score_aura"), Hash40::new("top"), 10.0, 10.0, 0.0, 90, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 2.7);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        //for _ in 0..4 {
            let beameffect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_vector"), Hash40::new("top"), &Vector3f{x:10.0,y:10.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 3.5, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(agent.module_accessor, beameffect, 0.5, 1.0, 0.0);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        //}
    } //macros::LAST_EFFECT_SET_RATE
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_score_aura"), Hash40::new("top"), 10.0, 10.0, 0.0, 90, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 2.7);
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        //for _ in 0..4 {
            let beameffect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_vector"), Hash40::new("top"), &Vector3f{x:10.0,y:10.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 3.5, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(agent.module_accessor, beameffect, 0.5, 1.0, 0.0);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        //}
    } //macros::LAST_EFFECT_SET_RATE
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
    frame(agent.lua_state_agent, 82.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_score_aura"), Hash40::new("top"), 10.0, 10.0, 0.0, 90, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 2.7);
    }
    frame(agent.lua_state_agent, 84.0);
    if macros::is_excute(agent) {
        //for _ in 0..4 {
            let beameffect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_vector"), Hash40::new("top"), &Vector3f{x:10.0,y:10.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 3.5, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(agent.module_accessor, beameffect, 0.5, 1.0, 0.0);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        //}
    } //macros::LAST_EFFECT_SET_RATE
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
    frame(agent.lua_state_agent, 89.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_score_aura"), Hash40::new("top"), 10.0, 10.0, 0.0, 90, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.7);
        macros::LAST_EFFECT_SET_RATE(agent, 2.7);
    }
    frame(agent.lua_state_agent, 91.0);
    if macros::is_excute(agent) {
        //for _ in 0..4 {
            let beameffect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_vector"), Hash40::new("top"), &Vector3f{x:10.0,y:10.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 3.5, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(agent.module_accessor, beameffect, 0.5, 1.0, 0.0);
            macros::LAST_EFFECT_SET_RATE(agent, 1.3);
        //}
    } //macros::LAST_EFFECT_SET_RATE
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_vector"), false, true);
    }
}



unsafe extern "C" fn meiling_qcf_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
}

unsafe extern "C" fn meiling_qcf_heavy_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 63.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
    frame(agent.lua_state_agent, 77.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
    frame(agent.lua_state_agent, 84.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
    frame(agent.lua_state_agent, 91.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
}





unsafe extern "C" fn meiling_qcb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 40, 70, 0, 60, 3.5, 0.0, 11.0, 8.0, Some(0.0), Some(5.5), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 40, 70, 0, 60, 10, 0.0, 7.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_WEAPON, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 10.0, 0.0, 7.0, 8.0, 0.0, 0.0, 0.0, 0.1, 0.1, 1.0, false, 0.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_qcb_eff(agent: &mut L2CAgentBase) {
    let lr = PostureModule::lr(agent.module_accessor);
    let mut projeffect: u32 = 0;
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        projeffect = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_final_aura2"), Hash40::new("handr"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 3.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        WorkModule::set_int(agent.module_accessor, projeffect as i32, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DAIR_EFFECT);
    }
    wait(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        projeffect = WorkModule::get_int(agent.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DAIR_EFFECT) as u32;
        EffectModule::kill(agent.module_accessor, projeffect, false, false);
    }
}

unsafe extern "C" fn meiling_qcb_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_special_sb03"));
    }
}





unsafe extern "C" fn meiling_dp_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);

    0.into()
}

unsafe extern "C" fn meiling_dp_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(),false.into());
    }
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_attack"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(meiling_dp_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_dp_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    if situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND {
        return 0.into();
    }
    return 0.into();
}

unsafe extern "C" fn meiling_dp_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}


unsafe extern "C" fn meiling_dp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 83, 70, 0, 64, 3.5, 0.0, 15.5, 4.5, Some(0.0), Some(16.5), Some(8.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 75, 70, 0, 64, 3.2, 0.0, 11.0, 5.5, Some(0.0), Some(11.0), Some(6.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 17.0, 83, 70, 0, 64, 3.5, 0.0, 19.0, 8.0, Some(0.0), Some(21.5), Some(9.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_dp_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 9.0, 18.0, 0.0, 15.0, 0.0, 0.0, 4.5, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_COLOR(agent, 10.0, 0.1, 0.1);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.1);
            macros::EFFECT(agent, Hash40::new("sys_piyo"), Hash40::new("top"), 9.0, 18.0, 0.0, 15.0, 0.0, 0.0, 4.3, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_COLOR(agent, 10.0, 0.1, 0.1);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_ground_shockwave"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_piyo"), false, true);
    }
}

unsafe extern "C" fn meiling_dp_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
    }
}












unsafe extern "C" fn meiling_neutralc(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    }
}

unsafe extern "C" fn meiling_neutralc_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 83, 73, 0, 60, 3.5, 0.0, 16.0, 5.5, Some(0.0), Some(16.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
}

unsafe extern "C" fn meiling_neutralc_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
}




unsafe extern "C" fn meiling_neutralc_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn meiling_neutralc_main(fighter: &mut L2CFighterCommon) -> L2CValue {	
        fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());

        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_set_ground_correct_by_situation(true.into());

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);

        fighter.sub_shift_status_main(L2CValue::Ptr(meiling_neutralc_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_neutralc_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE) {
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_5CBULLET_ANGLE);
            ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_5CBULLET, false, 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
        fighter.sub_set_ground_correct_by_situation(true.into());
    }
    else {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());
        } else {
            fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into());
        }
    }

    if (fighter.global_table[0x16] != *SITUATION_KIND_GROUND) && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_C_USED) {
        let speedy = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        notify_event_msc_cmd!(
            fighter,
            *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0,
            -1.5,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_accel"));
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    0.into()
}

unsafe extern "C" fn meiling_neutralc_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}



unsafe extern "C" fn meiling_downc(agent: &mut L2CAgentBase) {
    StatusModule::set_situation_kind(agent.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
    StatusModule::set_keep_situation_air(agent.module_accessor, true);

    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    }
}

unsafe extern "C" fn meiling_downc_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swimattack_high"));
    }
}


unsafe extern "C" fn meiling_downc_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    
    0.into()
}


unsafe extern "C" fn attack_hi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_hi"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_hi4_main_loop as *const () as _))
}
  
unsafe extern "C" fn attack_hi4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    if !MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE) {
            WorkModule::set_int(fighter.module_accessor, 30, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_5CBULLET_ANGLE);
            ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_5CBULLET, false, 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
    } else {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR { 
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
    }
    return 0.into();
}


unsafe extern "C" fn meiling_downc_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}















#[skyline::hook(offset = 0x33bdc30)]
unsafe extern "C" fn normal_weapon_hit_handler(vtable: u64, weapon: *mut smash::app::Weapon, log: u32) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_object_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_object_id as u32);
    let status_kind = StatusModule::status_kind(boma);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let color = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

    if ((*weapon).battle_object.kind == *WEAPON_KIND_MARIO_FIREBALL as u32) && is_meiling(owner_boma) {
        *(weapon as *mut bool).add(0x90) = true;
    }
    call_original!(vtable, weapon, log)
}

/* pub fn install() {
    skyline::install_hooks!(normal_weapon_hit_handler);
} */


unsafe extern "C" fn meiling_5cbullet_game_regular(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 100, 30, 30, 0, 2.5, 0.0, 2.0, 2.0, Some(0.0), Some(14.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 2, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 361, 20, 0, 70, 0.1, 0.0, 2.0, 2.0, Some(0.0), Some(14.0), Some(-2.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_5cbullet_effect_regular(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_shield_damage3"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn meiling_5cbullet_game_finish(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 72, 10, 0, 80, 2.5, 0.0, 2.0, 2.0, Some(0.0), Some(14.0), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 20.0, 361, 20, 0, 70, 0.1, 0.0, 2.0, 2.0, Some(0.0), Some(14.0), Some(-2.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}





pub unsafe extern "C" fn meiling_5cbullet_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
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

pub unsafe extern "C" fn meiling_5cbullet_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let mut downc_offset = Vector3f{x:0.0,y:0.0,z:0.01};
    let lr = PostureModule::lr(owner);
    PostureModule::set_lr(weapon.module_accessor, lr);

    let mut angle: f32 = 0.0;

    if WorkModule::get_int(owner, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_5CBULLET_ANGLE) != 0 {
        downc_offset = Vector3f{x:(-2.0 * lr),y:6.0,z:0.01};
        angle = -10.0;
    }
    let newPos = Vector3f{
        x: PostureModule::pos_x(owner) + (6.0*lr) + downc_offset.x, 
        y: PostureModule::pos_y(owner) + 1.0 + downc_offset.y, 
        z: PostureModule::pos_z(owner) + downc_offset.z
    };
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x: (angle / 2.0), y: 0.0, z: 0.0}, 0);

    0.into()
}

pub unsafe extern "C" fn meiling_5cbullet_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);

    let life = 52;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT);

    WorkModule::off_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);

    weapon.fastshift(L2CValue::Ptr(meiling_5cbullet_main_status_loop as *const () as _))
}


unsafe extern "C" fn meiling_5cbullet_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let lr = PostureModule::lr(owner);  
    let selflr = PostureModule::lr(weapon.module_accessor);
    
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let mut life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let maxlife = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    let mut angle: f32 = 0.0;
    if WorkModule::get_int(owner, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_5CBULLET_ANGLE) != 0 {
        angle = 10.0;
    }

    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x = smash::app::lua_bind::KineticEnergy::get_speed_x(energy_type);
    let mut speed_y = smash::app::lua_bind::KineticEnergy::get_speed_y(energy_type);

    if life == 50 {
        if selflr == -1.0 {
            angle = 180.0 - angle;
        }
        speed_x = (angle.to_radians()).cos()*1.2;
        speed_y = (angle.to_radians()).sin()*1.2;
    } else if (life <= 34 && life > 29) || life < 9 {
        speed_x *= 0.81;
        speed_y *= 0.81;
    }
    
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );



    if life > maxlife {
        life = maxlife;
    }

    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        WorkModule::inc_int(weapon.module_accessor, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("finish"), 0.0, 1.0, false, 0.0, false, false);

    }

    if life < 0 || WorkModule::get_int(weapon.module_accessor, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT) >= 2 {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }

    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);


    0.into()
}
pub unsafe extern "C" fn meiling_5cbullet_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
pub unsafe extern "C" fn meiling_5cbullet_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}




















unsafe extern "C" fn meiling_6c(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    }
}

unsafe extern "C" fn meiling_6c_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 83, 73, 0, 60, 3.5, 0.0, 16.0, 5.5, Some(0.0), Some(16.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
}




unsafe extern "C" fn meiling_6c_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn meiling_6c_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_set_ground_correct_by_situation(true.into());

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);

    fighter.sub_shift_status_main(L2CValue::Ptr(meiling_6c_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_6c_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_WAVE, false, 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
        fighter.sub_set_ground_correct_by_situation(true.into());
    } else {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            fighter.change_status((*FIGHTER_STATUS_KIND_FALL).into(), false.into());
        } else {
            fighter.change_status((*FIGHTER_STATUS_KIND_WAIT).into(), false.into());
        }
    }

    if (fighter.global_table[0x16] != *SITUATION_KIND_GROUND) && !WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_C_USED) {
        let speedy = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
        notify_event_msc_cmd!(
            fighter,
            *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0,
            -1.5,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_accel"));
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    0.into()
}

unsafe extern "C" fn meiling_6c_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_C_USED);
    0.into()
}








#[skyline::hook(offset = 0x33e1a70)]
unsafe extern "C" fn wave_init(vtable: u64, weapon: *mut app::Weapon, something: u64, something_2: f32) {
    original!()(vtable, weapon, something, something_2);
    let module_accessor = (*weapon).battle_object.module_accessor;
    let owner_object_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_object_id as u32);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    let color = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

    if !WorkModule::is_flag(module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR) && is_meiling(owner_boma) {
        *(weapon as *mut bool).add(0x90) = true;
        //println!("hook true");
    } else {
        //println!("hook false");
    }
}


#[skyline::hook(offset = 0x33e1fa4, inline)]
unsafe extern "C" fn wave_on_hit(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[24].x.as_ref() as *mut BattleObjectModuleAccessor;
    let owner_object_id = WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_object_id as u32);
    let owner_kind = utility::get_kind(&mut *owner_boma);
    if is_meiling(owner_boma) {
        /* let effect = if VarModule::is_flag(module_accessor, dolly_wave::status::flag::FINAL_HIT) {
            Hash40::new("effect_hitstrong")
        }
        else {
            Hash40::new("effect_hitstrong_last")
        }; */
        /* MotionAnimcmdModule::call_script_single(
            module_accessor,
            *WEAPON_ANIMCMD_EFFECT,
            effect,
            -1
        ); */
        *ctx.registers[19].w.as_mut() = 0;
    }
    /* else {
        //original!()(ctx);
    } */
}

/* pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x33e1fa4).nop();
    skyline::install_hooks!(
        wave_init,
        wave_on_hit
    );
} */




unsafe extern "C" fn terry_wave_game_shoot(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.2, 66, 50, 0, 70, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 3, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}



pub unsafe extern "C" fn meiling_6cbullet_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let ownerboma = smash::app::sv_battle_object::module_accessor(owner_id);
    let weaponboma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR_PASSABLE_OFF as u32,   //KEEP goes through stage
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32,
    );
    0.into()
}

pub unsafe extern "C" fn meiling_6cbullet_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let ownerboma = smash::app::sv_battle_object::module_accessor(owner_id);
    let weaponboma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let lr = PostureModule::lr(owner);    
    let newPos = Vector3f{x: PostureModule::pos_x(owner) + (6.0 * lr), y: PostureModule::pos_y(owner) + 10.0, z: PostureModule::pos_z(owner)};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    let projeffect = EffectModule::req_follow(weapon.module_accessor, Hash40::new("sys_shield_damage3"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 0.8, true, 0, 0, 0, 0, 0, true, true) as u32;

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        1.5*lr,
        0
    );


    0.into()
}

pub unsafe extern "C" fn meiling_6cbullet_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let ownerboma = smash::app::sv_battle_object::module_accessor(owner_id);
    let weaponboma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_GAME, Hash40::new("game_shoot"), -1);

    WorkModule::on_flag(weapon.module_accessor, *WEAPON_DOLLY_WAVE_INSTANCE_WORK_ID_FLAG_TYPE_AIR);

    let life = 60;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT);

    if PostureModule::lr(weapon.module_accessor) == 1.0 {
        WorkModule::set_float(weapon.module_accessor, 0.0, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLOAT_6CBULLET_ANGLE);
    } else {
        WorkModule::set_float(weapon.module_accessor, 180.0, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLOAT_6CBULLET_ANGLE);
    }

    WorkModule::off_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);

    if StopModule::is_stop(weapon.module_accessor){
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }

    weapon.fastshift(L2CValue::Ptr(meiling_6cbullet_main_status_loop as *const () as _))
}


unsafe extern "C" fn meiling_6cbullet_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let entryowner = WorkModule::get_int(owner, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let totalplayers = smash::app::lua_bind::FighterManager::entry_count(singletons::FighterManager());
    let posx = PostureModule::pos_x(weapon.module_accessor);
    let posy = PostureModule::pos_y(weapon.module_accessor);
    let lr = PostureModule::lr(weapon.module_accessor);
    let currentangle = WorkModule::get_float(weapon.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLOAT_6CBULLET_ANGLE);
    
    let mut loopentry: i32 = -1;
    let mut loopdist: f32 = 0.0;

    let mut angle = 0.0;
    if lr == -1.0 { angle = 180.0; }
    let mut enemyx = 0.0;
    let mut enemyy = 0.0;

    
    for i in 0..totalplayers {
        if i != entryowner as i32 {
            let loopboma = &mut *sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i as i32));

            let mut pos = Vector3f{x:0.0,y:0.0,z:0.0};
            ModelModule::joint_global_position(loopboma, Hash40::new("rot"), &mut pos, false);
            
		    let dsquared: f32 = (pos.x - posx).powf(2.0) + (pos.y - posy).powf(2.0);
		    let d = dsquared.sqrt();

            if (d < loopdist || loopdist == 0.0) && (((pos.x - posx) > 0.0 && lr > 0.0) || ((pos.x - posx) < 0.0 && lr < 0.0)) { 
                loopdist = d;
                loopentry = i;
                enemyx = pos.x;
                enemyy = pos.y;
            }

            
        }
	} 
    
    let pos_diff_y = enemyy - posy;
    let mut pos_diff_x = enemyx - posx;
    if pos_diff_x == 0.0 { pos_diff_x = 0.01; }
    
    if (loopentry != -1) {
        if pos_diff_y > 0.0 {
            angle = (pos_diff_y).atan2(pos_diff_x).to_degrees();
        }
        else {
            angle = 360.0 - (-pos_diff_y).atan2(pos_diff_x).to_degrees();
        }
        
    }

    if angle >= 265.0 { angle -= 360.0 }
    if currentangle <= (angle - 2.0) { 
        angle = currentangle + 2.0; 
    } else if currentangle >= (angle + 2.0) { 
        angle = currentangle - 2.0; 
    } else { 
        angle = currentangle; 
    }
    
    if angle > 85.0 && lr == 1.0 { angle = 85.0; }
    if angle < -85.0 && lr == 1.0 { angle = -85.0; }
    if angle < 95.0 && angle > 0.0 && lr == -1.0 { angle = 95.0; }
    if angle > -95.0 && angle < 0.0 && lr == -1.0 { angle = -95.0; }
    

    WorkModule::set_float(weapon.module_accessor, angle, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLOAT_6CBULLET_ANGLE);

    let speed = 1.4;
    let speed_x = (angle.to_radians()).cos()*speed;
    let speed_y = (angle.to_radians()).sin()*speed;
    
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );




    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let mut life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let maxlife = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);

    if life > maxlife {
        life = maxlife;
    }

    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        WorkModule::inc_int(weapon.module_accessor, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT);
    }

    
    if life < 0 || WorkModule::get_int(weapon.module_accessor, WEAPON_DOLLY_INSTANCE_WORK_ID_INT_6CBULLET_HIT) >= 3 {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }

    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}
pub unsafe extern "C" fn meiling_6cbullet_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
pub unsafe extern "C" fn meiling_6cbullet_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}








unsafe extern "C" fn meiling_ddb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 83, 20, 50, 70, 4.5, 0.0, 0.5, -19.0, Some(0.0), Some(0.5), Some(24.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_ddb_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        let ddeff = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:5.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 2.7, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_scale(agent.module_accessor, ddeff, &Vector3f{x:1.0,y:1.0,z:0.2});
        EffectModule::set_rgb(agent.module_accessor, ddeff, 1.0, 0.8, 0.0);
    }
}

unsafe extern "C" fn meiling_ddc(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 83, 20, 50, 70, 4.5, 0.0, 0.5, -19.0, Some(0.0), Some(0.5), Some(24.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn meiling_ddc_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        let ddeff = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:5.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 2.7, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_scale(agent.module_accessor, ddeff, &Vector3f{x:1.0,y:1.0,z:0.2});
        EffectModule::set_rgb(agent.module_accessor, ddeff, 1.0, 0.8, 0.0);
    }
}

unsafe extern "C" fn meiling_ddb_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
    }
}

unsafe extern "C" fn meiling_ddc_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_s"));
    }
}




unsafe extern "C" fn meiling_dd_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);

    0.into()
}

unsafe extern "C" fn meiling_dd_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("visual_scene_05"), 0.0, 1.0, false, 0.0, false, false);
    } else {        
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(meiling_dd_main_loop as *const () as _))
}

unsafe extern "C" fn meiling_dd_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    if situation_kind == *SITUATION_KIND_AIR && prev_situation_kind == *SITUATION_KIND_GROUND {
        return 0.into();
    }
    return 0.into();
}

unsafe extern "C" fn meiling_dd_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}










unsafe extern "C" fn game_specialhi1_meiling(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 98, 110, 110, 30, 4.0, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 75, 110, 110, 30, 4.0, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(6.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 6.0, Some(0.0), Some(3.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(3.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, Some(0.0), Some(3.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 83, 65, 0, 90, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(agent.module_accessor, 1.5);
    }
}

unsafe extern "C" fn sound_sideb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
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
        .game_acmd("game_specialn_meiling", meiling_neutralb, Default)          //neutral b
        .game_acmd("game_specialairn_meiling", meiling_neutralb_air, Default)
        .effect_acmd("effect_specialn_meiling", meiling_neutralb_eff, Default)
        .effect_acmd("effect_specialairn_meiling", meiling_neutralb_eff, Default)
        .sound_acmd("sound_specialn_meiling", meiling_neutralb_sound, Default)
        .sound_acmd("sound_specialairn_meiling", meiling_neutralb_sound, Default)
        //.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, meiling_neutralb_exec)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, meiling_neutralb_end)


        .game_acmd("game_speciallwend_meiling", meiling_downb, Default)          //down b
        .effect_acmd("effect_speciallwend_meiling", meiling_downb_eff, Default)
        .sound_acmd("sound_speciallwend_meiling", meiling_downb_sound, Default)
        .expression_acmd("expression_speciallwend_meiling", meiling_downb_exp, Default)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, meiling_downb_main)
        .status(Exit, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING, meiling_downb_exit)
        .status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING, meiling_downb_end)
        
        .game_acmd("game_specialsfend_meiling", meiling_qcf, Default)            //qcf
        .effect_acmd("effect_specialsfend_meiling", meiling_qcf_eff, Default)
        .game_acmd("game_specialsfattack_meiling", meiling_qcf_heavy, Default)
        .effect_acmd("effect_specialsfattack_meiling", meiling_qcf_heavy_eff, Default)
        .sound_acmd("sound_specialsfend_meiling", meiling_qcf_sound, Default)
        .sound_acmd("sound_specialsfattack_meiling", meiling_qcf_heavy_sound, Default)
        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, meiling_qcf_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, meiling_qcf_main)
        .status(Exit, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, meiling_qcf_exit)
        .status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, meiling_qcf_end)
        
        .game_acmd("game_specialsfstart_meiling", meiling_sideb, Default)        //side b f
        .game_acmd("game_specialairsfstart_meiling", meiling_sideb, Default)
        .sound_acmd("sound_specialsfstart_meiling", sound_sideb, Default)
        .sound_acmd("sound_specialairsfstart_meiling", sound_sideb, Default)
        //.effect_acmd("effect_specialsfstart", meiling_sideb_eff, Default)
        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE01, meiling_sideb_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE01, meiling_sideb_main)
        .status(End, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE01, meiling_sideb_end)

        .game_acmd("game_specialsbattack_meiling", meiling_qcb, Default)            //qcb
        .effect_acmd("effect_specialsbattack_meiling", meiling_qcb_eff, Default)
        .game_acmd("game_specialsbattackw_meiling", meiling_qcb, Default)
        .effect_acmd("effect_specialsbattackw_meiling", meiling_qcb_eff, Default)
        .sound_acmd("sound_specialsbattack_meiling", meiling_qcb_sound, Default)
        .sound_acmd("sound_specialsbattackw_meiling", meiling_qcb_sound, Default)

        .game_acmd("game_specialairlw_meiling", meiling_dp, Default)            //dp
        .effect_acmd("effect_specialairlw_meiling", meiling_dp_eff, Default)
        .sound_acmd("sound_specialairlw_meiling", meiling_dp_sound, Default)
        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, meiling_dp_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, meiling_dp_main)
        //.status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, meiling_dp_end)

        .game_acmd("game_catch_meiling", meiling_neutralc, Default)          //neutral c
        .effect_acmd("effect_catch_meiling", meiling_neutralc_eff, Default)
        .sound_acmd("sound_catch_meiling", meiling_neutralc_sound, Default)
        .status(Pre, *FIGHTER_STATUS_KIND_CATCH, meiling_neutralc_pre)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH, meiling_neutralc_main)
        .status(End, *FIGHTER_STATUS_KIND_CATCH, meiling_neutralc_end)
        
        .game_acmd("game_specialairlwrise_meiling", meiling_6c, Default)          //6c
        .effect_acmd("effect_specialairlwrise_meiling", meiling_6c_eff, Default)
        .sound_acmd("sound_specialairlwrise_meiling", meiling_downc_sound, Default)
        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, meiling_6c_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, meiling_6c_main)
        .status(End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, meiling_6c_end)

        .game_acmd("game_throwhi_meiling", meiling_downc, Default)              //down c
        .sound_acmd("sound_throwhi_meiling", meiling_downc_sound, Default)
        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE03, meiling_downc_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE03, attack_hi4_main)
        .status(End, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE03, meiling_downc_end)

        .game_acmd("game_catchwait_meiling", meiling_ddb, Default)              //dd
        .effect_acmd("effect_catchwait_meiling", meiling_ddb_eff, Default)
        .game_acmd("game_visualscene05_meiling", meiling_ddc, Default)
        .effect_acmd("effect_visualscene05_meiling", meiling_ddc_eff, Default)
        .sound_acmd("sound_catchwait_meiling", meiling_ddb_sound, Default)
        .sound_acmd("sound_visualscene05_meiling", meiling_ddc_sound, Default)
        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE05, meiling_dd_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE05, meiling_dd_main)
        .status(End, *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE05, meiling_dd_end)

        .game_acmd("game_specialhi1_meiling", game_specialhi1_meiling, Default)

        .install();



    Agent::new("dolly_6bbullet")
    .set_costume(costume.to_vec())
        .game_acmd("game_fly", meiling_6bbullet_game_regular, Default)
        .effect_acmd("effect_fly", meiling_6bbullet_effect_regular, Default)
        .effect_acmd("effect_stay", meiling_6bbullet_effect_regular, Default)
        .status(Pre, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_FLY, meiling_6bbullet_start_pre)
        .status(Init, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_FLY, meiling_6bbullet_start_init)
        .status(Main, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_FLY, meiling_6bbullet_start_main)
        .status(Pre, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_pre)
        .status(Init, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_init)
        .status(Main, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_main)
        .status(Exec, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_exec)
        .status(End, *WEAPON_FOX_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_end)
        .install();
    Agent::new("dolly_6bbullet2")
    .set_costume(costume.to_vec())
        .game_acmd("game_fly", meiling_6bbullet_game_regular, Default)
        .effect_acmd("effect_fly", meiling_6bbullet_effect_regular, Default)
        .effect_acmd("effect_stay", meiling_6bbullet_effect_regular, Default)
        .status(Pre, *WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_FLY, meiling_6bbullet_start_pre)
        .status(Init, *WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_FLY, meiling_6bbullet_start_init)
        .status(Main, *WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_FLY, meiling_6bbullet_start_main)
        .status(Pre, *WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_pre)
        .status(Init, *WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_init)
        .status(Main, *WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_main)
        .status(Exec, *WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_exec)
        .status(End, *WEAPON_FALCO_BLASTER_BULLET_STATUS_KIND_STAY, meiling_6bbullet_move_end)
        .install();

    Agent::new("dolly_6cbullet") //5c
    .set_costume(costume.to_vec())
        .game_acmd("game_regular", meiling_5cbullet_game_regular, Default)
        .game_acmd("game_finish", meiling_5cbullet_game_finish, Default)
        .effect_acmd("effect_regular", meiling_5cbullet_effect_regular, Default)
        .status(Pre, *WN_LINK_BOOMERANG_STATUS_KIND_START, meiling_5cbullet_pre)
        .status(Init, *WN_LINK_BOOMERANG_STATUS_KIND_START, meiling_5cbullet_init)
        .status(Main, *WN_LINK_BOOMERANG_STATUS_KIND_START, meiling_5cbullet_main)
        .status(Exec, *WN_LINK_BOOMERANG_STATUS_KIND_START, meiling_5cbullet_exec)
        .status(End, *WN_LINK_BOOMERANG_STATUS_KIND_START, meiling_5cbullet_end)
        .install();
        
    Agent::new("dolly_wave") //6c
    .set_costume(costume.to_vec())
        .game_acmd("game_shoot", terry_wave_game_shoot, Default)
        .status(Pre, *WEAPON_DOLLY_WAVE_STATUS_KIND_NORMAL, meiling_6cbullet_pre)
        .status(Init, *WEAPON_DOLLY_WAVE_STATUS_KIND_NORMAL, meiling_6cbullet_init)
        .status(Main, *WEAPON_DOLLY_WAVE_STATUS_KIND_NORMAL, meiling_6cbullet_main)
        .status(Exec, *WEAPON_DOLLY_WAVE_STATUS_KIND_NORMAL, meiling_6cbullet_exec)
        .status(End, *WEAPON_DOLLY_WAVE_STATUS_KIND_NORMAL, meiling_6cbullet_end)
        .install();

    //skyline::install_hooks!(normal_weapon_hit_handler);
    //skyline::install_hooks!(normal_weapon_hit_handler, wave_init);

    //let _ = skyline::patching::Patch::in_text(0x33e1fa4).nop();
    skyline::install_hooks!(
        wave_init,
        wave_on_hit,
        normal_weapon_hit_handler
    );

    unsafe {
        FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET += smashline::clone_weapon("fox", *WEAPON_KIND_FOX_BLASTER_BULLET, "dolly", "6bbullet", false);
        FIGHTER_DOLLY_GENERATE_ARTICLE_6BBULLET2 += smashline::clone_weapon("falco", *WEAPON_KIND_FALCO_BLASTER_BULLET, "dolly", "6bbullet2", false);
        FIGHTER_DOLLY_GENERATE_ARTICLE_5CBULLET += smashline::clone_weapon("link", *WEAPON_KIND_LINK_BOOMERANG, "dolly", "6cbullet", false);
        //FIGHTER_DOLLY_GENERATE_ARTICLE_5CBULLET += smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "dolly", "6cbullet", false);
    }

}