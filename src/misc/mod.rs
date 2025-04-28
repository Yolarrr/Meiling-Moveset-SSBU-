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

pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_STATUS_KIND: i32 = 0xA;
pub const STATUS_KIND: i32 = 0xB;
pub const STATUS_FRAME: i32 = 0xE;
pub const STATUS_FRAME_NO_INTERP: i32 = 0xF;

pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND: i32 = 0x200000EE;
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DASH_ATTACK_B: i32 = 0x200000EF;
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_NEAR: i32 = 0x200000F9;
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DAIR_EFFECT: i32 = 0x200000D2;

pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_B_USED: i32 = 0x200000F1;
pub const FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_C_USED: i32 = 0x200000F2;


static mut timerdp: [f32; 8] = [0.0; 8];
static mut checkdp1: [bool; 8] = [false; 8];
static mut checkdp2: [bool; 8] = [false; 8];
static mut timerqc: [f32; 8] = [0.0; 8];
static mut checkqc1: [bool; 8] = [false; 8];
static mut checkqc2: [bool; 8] = [false; 8];
static mut timerdd: [f32; 8] = [0.0; 8];
static mut timerdd2: [f32; 8] = [0.0; 8];
static mut checkdd1: [bool; 8] = [false; 8];
static mut checkdd2: [bool; 8] = [false; 8];
static mut dir: [f32; 8] = [0.0; 8];
static mut cancelb: [bool; 8] = [false; 8];
static mut cancelc: [bool; 8] = [false; 8];
static mut cancelskill: [bool; 8] = [false; 8];
static mut commandbuffertype: [i32; 8] = [0; 8];


unsafe extern "C" fn dolly_frames(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let statuskind = fighter.global_table[STATUS_KIND].get_i32();
        let statusframe = fighter.global_table[STATUS_FRAME].get_f32();

        //if is_meiling(fighter.module_accessor) {	
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ESCAPE_ATTACK);


        if statuskind == *FIGHTER_STATUS_KIND_ATTACK_LW4 && statusframe >= 29.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat_wait"), 0.0, 1.0, false, 0.0, false, false);

            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT_WAIT, false);
        }
        
        if statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK || 
            statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND || 
            statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B || 
            statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK || 
            statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND {
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_DOLLY_STRENGTH_S, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
            } else {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_DOLLY_STRENGTH_W, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH);
            }
        }

        if (statuskind != *FIGHTER_STATUS_KIND_ATTACK_DASH && statuskind != *FIGHTER_STATUS_KIND_ATTACK) {
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DASH_ATTACK_B);
        }

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
        

        let mut commandinputting :i32 = 0;

        let stickx = ControlModule::get_stick_x(fighter.module_accessor);
        let sticky = ControlModule::get_stick_y(fighter.module_accessor);
        let stickangle = ControlModule::get_stick_angle(fighter.module_accessor) * 57.2957786666;
        let facing = PostureModule::lr(fighter.module_accessor);


        //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        
        if (stickx > 0.6 && stickangle > -22.5 && stickangle < 22.5 && facing == 1.0) || (stickx < -0.6 && (stickangle < -157.5 || stickangle > 157.5) && facing == -1.0) 
        && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_f_end") {
            if !checkdp2[entry_id] {
                dir[entry_id] = facing;
            }
            checkdp1[entry_id] = true;
            timerdp[entry_id] = 14.0;
        }

        if checkdp1[entry_id] {
            if (sticky < -0.6 && stickangle >= -112.5 && stickangle <= -67.5) {
                checkdp2[entry_id] = true;
            } else if (stickangle > 30.0 && stickangle < 150.0) {
                checkdp1[entry_id] = false;
                checkdp2[entry_id] = false;
                dir[entry_id] = 0.0;
                timerdp[entry_id] = 0.0;
            }
        }

        if checkdp2[entry_id] && timerdp[entry_id] > 0.0 && ((stickangle > -67.5 && stickangle < 22.5 && facing == 1.0 && dir[entry_id] == facing) || 
        ((stickangle < -112.5 || stickangle > 157.5) && facing == -1.0 && dir[entry_id] == facing)) {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND) || cancelskill[entry_id]) && 
                fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                    commandinputting = 1;
                    fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND.into(), true.into());
                    if StopModule::is_stop(fighter.module_accessor) {
                        commandbuffertype[entry_id] = 1;
                    }
                    timerdp[entry_id] = 0.0;
                    checkdp1[entry_id] = false;
                    checkdp2[entry_id] = false;
                    dir[entry_id] = 0.0;
                }
            }
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND) || cancelskill[entry_id]) && 
                fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                    commandinputting = 1;
                    fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND.into(), true.into());
                    if StopModule::is_stop(fighter.module_accessor) {
                        commandbuffertype[entry_id] = 1;
                    }
                    timerdp[entry_id] = 0.0;
                    checkdp1[entry_id] = false;
                    checkdp2[entry_id] = false;
                    dir[entry_id] = 0.0;
                }
            }
        }

        if timerdp[entry_id] > 0.0 {
            timerdp[entry_id] -= 1.0 ;
        }

        if timerdp[entry_id] <= 0.0 || (statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND && statusframe < 20.0) {
            //timerdp[entry_id] = 13.0;
            checkdp1[entry_id] = false;
            checkdp2[entry_id] = false;
            dir[entry_id] = 0.0;
        }




        ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////



        if (sticky < -0.6 && stickangle >= -112.5 && stickangle <= -67.5) 
        && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_f_end") {
            checkqc1[entry_id] = true;
            dir[entry_id] = facing;
            timerqc[entry_id] = 14.0;
        }

        if checkqc1[entry_id] {
            if (stickangle > -67.5 && stickangle < 22.5) || (stickangle < -112.5 || stickangle > 157.5) {
                checkqc2[entry_id] = true;
            } else if (stickangle > 30.0 && stickangle < 150.0) {
                checkqc1[entry_id] = false;
                checkqc2[entry_id] = false;
                dir[entry_id] = 0.0;
                timerqc[entry_id] = 0.0;
            }
        }


        if checkqc2[entry_id] && timerqc[entry_id] > 0.0 && stickx > 0.6 && stickangle > -22.5 && stickangle < 22.5 { //  -->
            if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) || cancelskill[entry_id]) 
            && commandinputting == 0 && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                    if facing == 1.0 {
                        //println!("qcf right catch");
                        WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                        commandinputting = 2;
                        if statuskind == *FIGHTER_STATUS_KIND_TURN_RUN {
                            PostureModule::set_lr(fighter.module_accessor, facing * -1.0);
                            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
                            if StopModule::is_stop(fighter.module_accessor) {
                                commandbuffertype[entry_id] = 3;
                            }
                        } else {
                            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
                            if StopModule::is_stop(fighter.module_accessor) {
                                commandbuffertype[entry_id] = 2;
                            }
                        }
                        checkqc1[entry_id] = false;
                        checkqc2[entry_id] = false;
                        dir[entry_id] = 0.0;
                        timerqc[entry_id] = 0.0;
                    }
                    else {
                        //println!("qcb left catch");
                        WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                        commandinputting = 3;
                        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
                        if StopModule::is_stop(fighter.module_accessor) {
                            commandbuffertype[entry_id] = 3;
                        }
                        checkqc1[entry_id] = false;
                        checkqc2[entry_id] = false;
                        dir[entry_id] = 0.0;
                        timerqc[entry_id] = 0.0;
                    }
                } else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    if facing == 1.0 {
                        //println!("qcf right spec");
                        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                        commandinputting = 2;
                        if statuskind == *FIGHTER_STATUS_KIND_TURN_RUN {
                            PostureModule::set_lr(fighter.module_accessor, facing * -1.0);
                            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B.into(), true.into());
                            if StopModule::is_stop(fighter.module_accessor) {
                                commandbuffertype[entry_id] = 4;
                            }
                        } else {
                            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
                            if StopModule::is_stop(fighter.module_accessor) {
                                commandbuffertype[entry_id] = 2;
                            }
                        }
                        checkqc1[entry_id] = false;
                        checkqc2[entry_id] = false;
                        dir[entry_id] = 0.0;
                        timerqc[entry_id] = 0.0;
                    }
                    else {
                        //println!("qcb left spec");
                        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                        commandinputting = 3;
                        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B.into(), true.into());
                        if StopModule::is_stop(fighter.module_accessor) {
                            commandbuffertype[entry_id] = 4;
                        }
                        checkqc1[entry_id] = false;
                        checkqc2[entry_id] = false;
                        dir[entry_id] = 0.0;
                        timerqc[entry_id] = 0.0;
                    }
                }
            }
        }
        
        if checkqc2[entry_id] && timerqc[entry_id] > 0.0 && stickx < -0.6 && (stickangle < -157.5 || stickangle > 157.5) {
            if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) || cancelskill[entry_id]) 
            && commandinputting == 0 && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                    if facing == 1.0 {
                        //println!("qcb right catch");
                        WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                        commandinputting = 3;
                        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
                        if StopModule::is_stop(fighter.module_accessor) {
                            commandbuffertype[entry_id] = 3;
                        }
                        checkqc1[entry_id] = false;
                        checkqc2[entry_id] = false;
                        dir[entry_id] = 0.0;
                        timerqc[entry_id] = 0.0;
                    }
                    else {
                        //println!("qcf left catch");
                        WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                        commandinputting = 2;
                        if statuskind == *FIGHTER_STATUS_KIND_TURN_RUN {
                            PostureModule::set_lr(fighter.module_accessor, facing * -1.0);
                            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
                            if StopModule::is_stop(fighter.module_accessor) {
                                commandbuffertype[entry_id] = 3;
                            }
                        } else {
                            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
                            if StopModule::is_stop(fighter.module_accessor) {
                                commandbuffertype[entry_id] = 2;
                            }
                        }
                        checkqc1[entry_id] = false;
                        checkqc2[entry_id] = false;
                        dir[entry_id] = 0.0;
                        timerqc[entry_id] = 0.0;
                    }
                } else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    if facing == 1.0 {
                        //println!("qcb right spec");
                        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                        commandinputting = 3;
                        fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B.into(), true.into());
                        if StopModule::is_stop(fighter.module_accessor) {
                            commandbuffertype[entry_id] = 4;
                        }
                        checkqc1[entry_id] = false;
                        checkqc2[entry_id] = false;
                        dir[entry_id] = 0.0;
                        timerqc[entry_id] = 0.0;
                    }
                    else {
                        //println!("qcf left spec");
                        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                        commandinputting = 2;
                        if statuskind == *FIGHTER_STATUS_KIND_TURN_RUN {
                            PostureModule::set_lr(fighter.module_accessor, facing * -1.0);
                            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B.into(), true.into());
                            if StopModule::is_stop(fighter.module_accessor) {
                                commandbuffertype[entry_id] = 4;
                            }
                        } else {
                            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
                            if StopModule::is_stop(fighter.module_accessor) {
                                commandbuffertype[entry_id] = 2;
                            }
                        }
                        checkqc1[entry_id] = false;
                        checkqc2[entry_id] = false;
                        dir[entry_id] = 0.0;
                        timerqc[entry_id] = 0.0;
                    }
                }
            }
        }



        if timerqc[entry_id] > 0.0 {
            timerqc[entry_id] -= 1.0;
        }


        if timerqc[entry_id] <= 0.0 || (statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK && statusframe < 25.0) || 
        (statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND && statusframe < 8.0) ||
        (statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B && statusframe < 8.0) ||
        (statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK && statusframe < 8.0) {
            checkqc1[entry_id] = false;
            checkqc2[entry_id] = false;
            dir[entry_id] = 0.0;
        }










        //dd
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


        if sticky < -0.6 && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_f_end") {
            checkdd1[entry_id] = true;
            if !checkdd2[entry_id] {
                timerdd[entry_id] = 16.0;
            } else {
                timerdd2[entry_id] = 16.0;
            }
        }

        if checkdd1[entry_id] {
            if sticky >= -0.6 {
                checkdd2[entry_id] = true;
            }
        }

        if checkdd2[entry_id] && timerdd[entry_id] > 0.0 && sticky < -0.6 && commandinputting == 0 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND) || cancelskill[entry_id]) && 
                fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    WorkModule::on_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                    commandinputting = 4;
                    fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE05.into(), true.into());
                    if StopModule::is_stop(fighter.module_accessor) {
                        commandbuffertype[entry_id] = 5;
                    }
                    timerdd[entry_id] = 0.0;
                    checkdd1[entry_id] = false;
                    checkdd2[entry_id] = false;
                }
            }
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                if (WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND) || cancelskill[entry_id]) && 
                fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_COMMAND);
                    commandinputting = 4;
                    fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE05.into(), true.into());
                    if StopModule::is_stop(fighter.module_accessor) {
                        commandbuffertype[entry_id] = 5;
                    }
                    timerdd[entry_id] = 0.0;
                    checkdd1[entry_id] = false;
                    checkdd2[entry_id] = false;
                }
            }
        }

        if timerdd[entry_id] > 0.0 {
            timerdd[entry_id] -= 1.0 ;
        }

        if timerdd2[entry_id] > 0.0 {
            timerdd2[entry_id] -= 1.0 ;
        }

        if timerdd[entry_id] <= 0.0 || (statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND && statusframe < 20.0) {
            if timerdd2[entry_id] > 0.0 && timerdd2[entry_id] < 16.0 {
                timerdd[entry_id] = timerdd2[entry_id];
                timerdd2[entry_id] = 0.0;
                checkdd2[entry_id] = false;
            } else {
            checkdd1[entry_id] = false;
            checkdd2[entry_id] = false;
            }
        }



        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////








        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) && commandinputting == 0  &&
        ((fighter.global_table[STATUS_KIND] == *FIGHTER_STATUS_KIND_DASH && statusframe >= 5.0) || 
        fighter.global_table[STATUS_KIND] == *FIGHTER_STATUS_KIND_RUN) {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                if (stickx > 0.65 && facing == 1.0) || (stickx < -0.65 && facing == -1.0) {
                    //println!("dash atk b");
                    WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DASH_ATTACK_B);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_DASH, false);

                    commandinputting = 12;
                }
            }
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) && ItemModule::get_have_item_size(fighter.module_accessor, 0) != 0 {
                if (stickx > 0.65 && facing == 1.0) || (stickx < -0.65 && facing == -1.0) {
                    //println!("dash atk c");
                    fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), true.into());
                    commandinputting = 13;
                }
            }
        }


        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND) && commandinputting == 0 
        && ItemModule::get_have_item_size(fighter.module_accessor, 0) != 0 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                //println!("c moves");
                if (stickx > 0.65 && facing == 1.0) || (stickx < -0.65 && facing == -1.0) {
                    fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
                    if StopModule::is_stop(fighter.module_accessor) {
                        commandbuffertype[entry_id] = 6;
                    }
                } else if sticky < -0.65 && fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE03.into(), true.into());
                    if StopModule::is_stop(fighter.module_accessor) {
                        commandbuffertype[entry_id] = 7;
                    }
                } else {
                    fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
                    if StopModule::is_stop(fighter.module_accessor) {
                        commandbuffertype[entry_id] = 8;
                    }
                }
            }
        }

        if statuskind == *FIGHTER_STATUS_KIND_ATTACK && MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_13") 
        && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
        || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)) 
        && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) 
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if (stickx > 0.65 && facing == 1.0) || (stickx < -0.65 && facing == -1.0) {
                WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DASH_ATTACK_B);
            } else if sticky < -0.65 {
                WorkModule::set_int(fighter.module_accessor, 2, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DASH_ATTACK_B);
            } else {
                WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DASH_ATTACK_B);
            }
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE04.into(), true.into());
        }






        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            fighter.sub_transition_group_check_air_escape();
        }







        // *******CANCELS*******

        if (statuskind == *FIGHTER_STATUS_KIND_ATTACK || statuskind == *FIGHTER_STATUS_KIND_ATTACK_AIR || statuskind == *FIGHTER_STATUS_KIND_ATTACK_HI3 
            || (statuskind == *FIGHTER_STATUS_KIND_ATTACK_DASH 
            && WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DASH_ATTACK_B) == 0) 
            || statuskind == *FIGHTER_STATUS_KIND_ATTACK_HI4 || statuskind == *FIGHTER_STATUS_KIND_ATTACK_LW3 || statuskind == *FIGHTER_STATUS_KIND_ATTACK_LW4 
            || statuskind == *FIGHTER_STATUS_KIND_ATTACK_S3 || statuskind == *FIGHTER_STATUS_KIND_ATTACK_S4) 
            && commandinputting == 0 
            && (AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
            || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) 
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)) {

            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);

            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR && !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                //println!("a cancellable escape air");
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
                fighter.sub_transition_group_check_air_escape();
            }

            fighter.sub_transition_group_check_ground_special();
            fighter.sub_transition_group_check_air_special();

            cancelb[entry_id] = true;
            cancelc[entry_id] = true;
            cancelskill[entry_id] = true;

        } else if (statuskind == *FIGHTER_STATUS_KIND_SPECIAL_N && (statusframe >= 13.0 || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD))) 
        || (statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING && (statusframe >= 16.0 || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD))) 
        || (statuskind == *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE01 && statusframe >= 16.0) 
        || (statuskind == *FIGHTER_STATUS_KIND_SPECIAL_LW && (statusframe >= 16.0 || AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD))) 
        && commandinputting == 0 {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);

            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR && !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                //println!("b cancellable escape air");
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
                fighter.sub_transition_group_check_air_escape();
            }
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }

            fighter.sub_transition_group_check_ground_special();
            fighter.sub_transition_group_check_air_special();

            cancelb[entry_id] = false;
            cancelc[entry_id] = true;
            cancelskill[entry_id] = true;

        } else if ((statuskind == *FIGHTER_STATUS_KIND_CATCH && statusframe >= 18.0) || (statuskind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND && statusframe >= 17.0) 
        || (statuskind == *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE03 && statusframe >= 19.0) 
        || (statuskind == *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE04 && MotionModule::motion_kind(fighter.module_accessor) == hash40("catch_attack") 
            && (AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) 
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)))) 
        && commandinputting < 5 {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);

            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR && !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                //println!("c cancellable escape air");
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
                fighter.sub_transition_group_check_air_escape();
            }
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            }

            fighter.sub_transition_group_check_ground_special();
            fighter.sub_transition_group_check_air_special();

            cancelb[entry_id] = false;
            cancelc[entry_id] = false;
            cancelskill[entry_id] = true;

        } else {
            cancelb[entry_id] = false;
            cancelc[entry_id] = false;
            cancelskill[entry_id] = false;
        }

        if statuskind == *FIGHTER_STATUS_KIND_ATTACK && //motion_kind maybe?
        AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) ||
        AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        }

        if statuskind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
        }

        if statuskind == *FIGHTER_STATUS_KIND_ATTACK_LW3 &&
        AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) ||
        AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
        }

        if statuskind == *FIGHTER_STATUS_KIND_ESCAPE_AIR && statusframe > 4.0 && statusframe < 30.0 {
            //println!("escape airing");
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            fighter.sub_transition_group_check_ground_special();
            fighter.sub_transition_group_check_air_special();
            fighter.sub_transition_group_check_air_attack();
        }


        if !StopModule::is_stop(fighter.module_accessor) && commandbuffertype[entry_id] > 0 && commandbuffertype[entry_id] < 10 {
            if commandbuffertype[entry_id] == 1 {
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND.into(), true.into());
                commandbuffertype[entry_id] = 0;
            }
            if commandbuffertype[entry_id] == 2 {
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
                commandbuffertype[entry_id] = 0;
            }
            if commandbuffertype[entry_id] == 3 {
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND.into(), true.into());
                commandbuffertype[entry_id] = 0;
            }
            if commandbuffertype[entry_id] == 4 {
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B.into(), true.into());
                commandbuffertype[entry_id] = 0;
            }
            if commandbuffertype[entry_id] == 5 {
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE05.into(), true.into());
                commandbuffertype[entry_id] = 0;
            }
            if commandbuffertype[entry_id] == 6 {
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND.into(), true.into());
                commandbuffertype[entry_id] = 0;
            }
            if commandbuffertype[entry_id] == 7 {
                fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE03.into(), true.into());
                commandbuffertype[entry_id] = 0;
            }
            if commandbuffertype[entry_id] == 8 {
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
                commandbuffertype[entry_id] = 0;
            }
        }




        
        //once per air time stuff

        let situation_kind = fighter.global_table[0x16].get_i32();
        if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&situation_kind)
        || [*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING].contains(&statuskind) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_B_USED);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_C_USED);
        }

        



        // anim fixes

        if statuskind == *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY {
            fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA_STAND.into(), true.into());
        }

        if statuskind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            fighter.change_status(FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE01.into(), true.into());
        }
        

        if ((fighter.global_table[PREV_STATUS_KIND] == *FIGHTER_STATUS_KIND_ATTACK_AIR && statusframe < 3.0) 
        || statuskind == *FIGHTER_STATUS_KIND_ATTACK_AIR) ||
        (fighter.global_table[PREV_STATUS_KIND] == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK && statusframe < 3.0) {
            let projeffect: u32 = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_DAIR_EFFECT) as u32;
            EffectModule::kill(fighter.module_accessor, projeffect, false, false);
        }

        if ((fighter.global_table[PREV_STATUS_KIND] == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_LANDING || fighter.global_table[PREV_STATUS_KIND] == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK) && statusframe < 3.0) {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_vector"), true, true);
        }


        if statuskind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW && statusframe < 12.0 {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_catch"), true, true);
        }
        if statuskind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW && statusframe == 37.0 {
            let target = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_OBJECT_ID);
            let enemyboma = &mut *sv_battle_object::module_accessor(target as u32);
            DamageModule::add_damage(enemyboma, 3.0, 0);
            
        }
        if statuskind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW && statusframe > 47.0 && statusframe <= 109.0 {
            let target = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL2_WORK_INT_TARGET_OBJECT_ID);
            
            let enemyboma = &mut *sv_battle_object::module_accessor(target as u32);

            if statusframe < 109.0 {
                GroundModule::set_collidable(enemyboma, false);
            } else {
                GroundModule::set_collidable(enemyboma, true);
            }
            
            let mut angle = (statusframe - 47.0) * 22.5;
            let mut rotation = Vector3f{x: (angle), y: 0.0, z: 0.0};

            if is_meiling(enemyboma) {
                ModelModule::set_joint_rotate(enemyboma, Hash40::new("rot"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_BEFORE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            } else {
                ModelModule::set_joint_rotate(enemyboma, Hash40::new("hip"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_BEFORE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
            }
        }
    }
}

unsafe extern "C" fn meiling_furafura(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.6);
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
        .game_acmd("game_furafura_meiling", meiling_furafura, Default)
        .on_line(Main, dolly_frames)    
        .install();
}

