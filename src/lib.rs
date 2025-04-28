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
    clippy::borrow_interior_mutable_const,
	unused_parens,
	unused_mut,
	dead_code
)]

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

mod normals;
mod aerials;
mod specials;
mod misc;
mod finalsmash;
//mod params;
//mod articles;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const FIGHTER_NAME: &str = "dolly";
    const MARKER_FILE: &str = "meiling.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }
	
	param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(),(hash40("param_private"), hash40("super_special_damage"), 10000.0));
	param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(),(hash40("param_private"), hash40("super_special_rate"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(),(hash40("param_private"), hash40("super_special_hp_min"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(),(hash40("param_private"), hash40("super_special_hp_max"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(),(hash40("param_private"), hash40("scale_z"), 0.01));
	param_config::update_int_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(),(hash40("param_super_special2"), hash40("hit_stop_frame_fighter"), 0));
	param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(),(hash40("param_special_s"), hash40("attack_stop_cliff_offset_y"), 0.0));
	param_config::update_int_2(-*WEAPON_KIND_DOLLY_WAVE, marked_slots.clone(),(hash40("param_wave"), hash40("lifetime_ground_w"), 120));
	param_config::update_int_2(-*WEAPON_KIND_DOLLY_WAVE, marked_slots.clone(),(hash40("param_wave"), hash40("lifetime_ground_s"), 120));
	param_config::update_int_2(-*WEAPON_KIND_DOLLY_WAVE, marked_slots.clone(),(hash40("param_wave"), hash40("lifetime_air_w"), 120));
	param_config::update_int_2(-*WEAPON_KIND_DOLLY_WAVE, marked_slots.clone(),(hash40("param_wave"), hash40("lifetime_air_s"), 120));
	param_config::update_int_2(-*WEAPON_KIND_FOX_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet"), hash40("life"), 22));
	param_config::update_float_2(-*WEAPON_KIND_FOX_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet"), hash40("brake"), 0.0));
	param_config::update_float_2(-*WEAPON_KIND_FOX_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet"), hash40("base_offset_x"), 8.2));
	param_config::update_float_2(-*WEAPON_KIND_FOX_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet"), hash40("scale_max"), 3.0));
	param_config::update_float_2(-*WEAPON_KIND_FOX_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet"), hash40("scale_mul"), 11.25));
	param_config::update_int_2(-*WEAPON_KIND_FOX_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet"), hash40("is_penetration"), 0));
	param_config::update_float_2(-*WEAPON_KIND_FOX_BLASTER_BULLET, marked_slots.clone(),(hash40("map_collision_6bbullet"), hash40("up"), 1.0));
	param_config::update_float_2(-*WEAPON_KIND_FOX_BLASTER_BULLET, marked_slots.clone(),(hash40("map_collision_6bbullet"), hash40("down"), -1.0));
	param_config::update_float_2(-*WEAPON_KIND_FOX_BLASTER_BULLET, marked_slots.clone(),(hash40("map_collision_6bbullet"), hash40("half_width"), 10.0));
	param_config::update_int_2(-*WEAPON_KIND_FALCO_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet2"), hash40("life"), 22));
	param_config::update_float_2(-*WEAPON_KIND_FALCO_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet2"), hash40("brake"), 0.0));
	param_config::update_float_2(-*WEAPON_KIND_FALCO_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet2"), hash40("base_offset_x"), 8.2));
	param_config::update_float_2(-*WEAPON_KIND_FALCO_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet2"), hash40("scale_max"), 3.0));
	param_config::update_float_2(-*WEAPON_KIND_FALCO_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet2"), hash40("scale_mul"), 11.25));
	param_config::update_int_2(-*WEAPON_KIND_FALCO_BLASTER_BULLET, marked_slots.clone(),(hash40("param_6bbullet2"), hash40("is_penetration"), 0));
	param_config::update_float_2(-*WEAPON_KIND_FALCO_BLASTER_BULLET, marked_slots.clone(),(hash40("map_collision_6bbullet2"), hash40("up"), 1.0));
	param_config::update_float_2(-*WEAPON_KIND_FALCO_BLASTER_BULLET, marked_slots.clone(),(hash40("map_collision_6bbullet2"), hash40("down"), -1.0));
	param_config::update_float_2(-*WEAPON_KIND_FALCO_BLASTER_BULLET, marked_slots.clone(),(hash40("map_collision_6bbullet2"), hash40("half_width"), 10.0));
    //param_config::update_float_2(-*WEAPON_KIND_MEWTWO_SHADOWBALL, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_shadowball"), hash40("min_speed"), 4.0));

    if lowest_color == -1 {
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };

    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_meiling");

	the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
            ui_chara_id: smash::hash40("ui_chara_meiling"),
            clone_from_ui_chara_id: None,
            name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("meiling")), 
            fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(0x1226f4b6cd),
            fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(0x1226f4b6cd),
            ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_series_touhou")),
            //ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(0x13599d69f8),
            fighter_type: the_csk_collection_api::Hash40Type::Overwrite(0x1353795179),
            alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x0), 
            exhibit_year: the_csk_collection_api::ShortType::Overwrite(2002),
            exhibit_day_order: the_csk_collection_api::IntType::Overwrite(81109), 
            ext_skill_page_num: the_csk_collection_api::SignedByteType::Overwrite(3), 
            is_img_ext_skill_page0: the_csk_collection_api::BoolType::Overwrite(false), 
			is_img_ext_skill_page1: the_csk_collection_api::BoolType::Overwrite(false), 
			is_img_ext_skill_page2: the_csk_collection_api::BoolType::Overwrite(false),
            skill_list_order: the_csk_collection_api::SignedByteType::Optional(Some(75)), 
            disp_order: the_csk_collection_api::SignedByteType::Optional(Some(75)), 
            save_no: the_csk_collection_api::SignedByteType::Overwrite(75), 
			chara_count: the_csk_collection_api::SignedByteType::Overwrite(1), 
            can_select: the_csk_collection_api::BoolType::Overwrite(true), 
			is_usable_soundtest: the_csk_collection_api::BoolType::Overwrite(true), 
			is_called_pokemon: the_csk_collection_api::BoolType::Overwrite(false), 
			is_mii: the_csk_collection_api::BoolType::Overwrite(false), 
			is_boss: the_csk_collection_api::BoolType::Overwrite(false), 
			is_hidden_boss: the_csk_collection_api::BoolType::Overwrite(false), 
			is_dlc: the_csk_collection_api::BoolType::Overwrite(false), 
			is_patch: the_csk_collection_api::BoolType::Overwrite(false), 
            is_plural_message: the_csk_collection_api::BoolType::Overwrite(false), 
			is_plural_narration: the_csk_collection_api::BoolType::Overwrite(false), 
			is_article: the_csk_collection_api::BoolType::Overwrite(false), 
            extra_flags: the_csk_collection_api::IntType::Overwrite(0), 
            //unk_0x112b7bb52a: the_csk_collection_api::BoolType::Overwrite(false),
            has_multiple_face: the_csk_collection_api::BoolType::Overwrite(false),
            result_pf0: the_csk_collection_api::BoolType::Overwrite(true), 
			result_pf1: the_csk_collection_api::BoolType::Overwrite(true), 
			result_pf2: the_csk_collection_api::BoolType::Overwrite(true), 
			color_num: the_csk_collection_api::UnsignedByteType::Overwrite(color_num as u8), 
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(std::collections::HashMap::from([
				(0x915C075DE /* c00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9B3B77E6A /* c01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9825F64F7 /* c02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x924286F43 /* c03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9E18F51CD /* c04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x947F85A79 /* c05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9761040E4 /* c06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9D0674B50 /* c07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9E48F9289 /* n00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x942F8993D /* n01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9731083A0 /* n02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9D5678814 /* n03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x910C0B69A /* n04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9B6B7BD2E /* n05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9875FA7B3 /* n06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x92128AC07 /* n07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9F873561A /* c00_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x95E045DAE /* c01_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x96FEC4733 /* c02_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9C99B4C87 /* c03_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x90C3C7209 /* c04_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9AA4B79BD /* c05_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x99BA36320 /* c06_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x93DD46894 /* c07_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x11895f00fc, the_csk_collection_api::UnsignedByteType::Overwrite(lowest_color as _)),
			])), 
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(std::collections::HashMap::from([
				(0x1337FC912E /* characall_label_c00 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_meiling"))), 
				(0x1340FBA1B8 /* characall_label_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13D9F2F002 /* characall_label_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13AEF5C094 /* characall_label_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1330915537 /* characall_label_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13479665A1 /* characall_label_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13DE9F341B /* characall_label_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13A998048D /* characall_label_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B8B13E500 /* characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1BFC14D596 /* characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B651D842C /* characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B121AB4BA /* characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B8C7E2119 /* characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1BFB79118F /* characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B62704035 /* characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B157770A3 /* characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				//(0x160ab9eb98, the_csk_collection_api::Hash40Type::Overwrite(0xC629A3E1A /* ui_chara_ike */)),
			])), 
            shop_item_tag: the_csk_collection_api::Hash40Type::Overwrite(0x0), 
        },
    );

	the_csk_collection_api::add_chara_layout_db_entry_info(the_csk_collection_api::CharacterLayoutDatabaseEntry {
		ui_layout_id: smash::hash40("ui_chara_meiling_00"), 
		ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_meiling")),
		chara_color: the_csk_collection_api::UnsignedByteType::Optional(Some(0)), 
		eye_0_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(1)), 
		eye_1_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(1)), 
		eye_2_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(1)), 
		eye_0_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-20.0)), 
		eye_0_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(184.0)), 
		eye_0_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(44.0)), 
		eye_1_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(194.0)), 
		eye_1_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_flash_info_pos_x: the_csk_collection_api::FloatType::Optional(Some(-3.0)), 
		eye_flash_info_pos_y: the_csk_collection_api::FloatType::Optional(Some(2.0)), 
		chara_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(20.0)), 
		chara_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-65.0)), 
		chara_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.1)), 
		chara_1_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(20.0)), 
		chara_1_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-60.0)), 
		chara_1_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.5)), 
		chara_1_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_1_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(8.0)), 
		chara_1_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-34.0)), 
		chara_1_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.42)), 
		chara_1_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-40.0)), 
		chara_1_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.43)), 
		chara_1_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		//chara_3_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(130.0)), 
		//chara_3_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(-300.0)), 
		chara_3_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(50.0)), 
		chara_3_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(-100.0)), 
		//chara_3_0_scale: the_csk_collection_api::FloatType::Optional(Some(0.75)), 
		chara_3_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		//chara_3_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(190.0)), 
		//chara_3_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-300.0)), 
		chara_3_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(80.0)), 
		chara_3_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-100.0)), 
		//chara_3_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.75)), 
		chara_3_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		//chara_3_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		//chara_3_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_3_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), //working
		chara_3_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		//chara_3_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_3_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.1)), 
		//chara_3_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(100.0)), 
		//chara_3_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-200.0)), 
		chara_3_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(40.0)), 
		chara_3_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-70.0)), 
		//chara_3_3_scale: the_csk_collection_api::FloatType::Optional(Some(0.68)), 
		chara_3_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.1)), 
		//chara_3_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(32.0)), 
		//chara_3_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-32.0)), 
		chara_3_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(12.0)), 
		chara_3_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-12.0)), 
		//chara_3_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		chara_3_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.9)), 
		//chara_3_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(140.0)), 
		//chara_3_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(-340.0)), 
		chara_3_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(50.0)), 
		chara_3_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(-120.0)), 
		//chara_3_5_scale: the_csk_collection_api::FloatType::Optional(Some(0.7)), 
		chara_3_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		//chara_3_6_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		//chara_3_6_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_3_6_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_3_6_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		//chara_3_6_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_3_6_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		//chara_3_7_offset_x: the_csk_collection_api::FloatType::Optional(Some(100.0)), 
		//chara_3_7_offset_y: the_csk_collection_api::FloatType::Optional(Some(-240.0)), 
		chara_3_7_offset_x: the_csk_collection_api::FloatType::Optional(Some(40.0)), 
		chara_3_7_offset_y: the_csk_collection_api::FloatType::Optional(Some(-80.0)), 
		//chara_3_7_scale: the_csk_collection_api::FloatType::Optional(Some(0.7)), 
		chara_3_7_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		chara_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_select_icon_list_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_select_icon_list_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_select_icon_list_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_7_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_7_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		spirits_eye_visible: the_csk_collection_api::BoolType::Optional(Some(true)), 
		..std::default::Default::default()
	});

	//the_csk_collection_api::add_chara_layout_db_entry_info(the_csk_collection_api::CharacterLayoutDatabaseEntry {

	/* the_csk_collection_api::add_bgm_db_entry_info(&the_csk_collection_api::BgmDatabaseRootEntry {
		ui_bgm_id: smash::hash40("ui_bgm_meiling_victory"),
    	clone_from_ui_bgm_id: None,
    	stream_set_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("set_meiling_victory")),
    	rarity: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("bgm_rarity_0")),
    	record_type: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("record_none")),
    	ui_gametitle_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_gametitle_none")),
    	ui_gametitle_id_1: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_gametitle_none")),
    	ui_gametitle_id_2: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_gametitle_none")),
    	ui_gametitle_id_3: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_gametitle_none")),
    	ui_gametitle_id_4: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_gametitle_none")),
    	name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("meiling_victory")),
    	save_no: the_csk_collection_api::ShortType::Overwrite(-1),
    	test_disp_order: the_csk_collection_api::ShortType::Overwrite(-1),
    	menu_value: the_csk_collection_api::IntType::Overwrite(-1),
    	jp_region: the_csk_collection_api::BoolType::Overwrite(false),
    	other_region: the_csk_collection_api::BoolType::Overwrite(false),
    	possessed: the_csk_collection_api::BoolType::Overwrite(false),
    	prize_lottery: the_csk_collection_api::BoolType::Overwrite(false),
    	shop_price: the_csk_collection_api::UnsignedIntType::Overwrite(0),
    	count_target: the_csk_collection_api::BoolType::Overwrite(false),
    	menu_loop: the_csk_collection_api::UnsignedByteType::Overwrite(0),
    	is_selectable_stage_make: the_csk_collection_api::BoolType::Overwrite(false),
    	is_selectable_movie_edit: the_csk_collection_api::BoolType::Overwrite(false),
    	is_selectable_original: the_csk_collection_api::BoolType::Overwrite(false),
    	is_dlc: the_csk_collection_api::BoolType::Overwrite(false),
    	is_patch: the_csk_collection_api::BoolType::Overwrite(false),
    	dlc_ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	dlc_mii_hat_motif_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	dlc_mii_body_motif_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	unk_0x0e6b57e593: the_csk_collection_api::BoolType::Overwrite(false)
	});

	the_csk_collection_api::add_assigned_info_entry_info(&the_csk_collection_api::AssignedInfoEntry {
		info_id: smash::hash40("info_meiling_victory"),
		clone_from_info_id: None,
		stream_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("stream_meiling_victory")),
		condition: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("sound_condition_none")),
		condition_process: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("sound_condition_process_add")),
		start_frame: the_csk_collection_api::IntType::Overwrite(0),
		change_fadein_frame: the_csk_collection_api::IntType::Overwrite(0),
		change_start_delay_frame: the_csk_collection_api::IntType::Overwrite(0),
		change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
		change_stop_delay_frame: the_csk_collection_api::IntType::Overwrite(0),
		menu_change_fadein_frame: the_csk_collection_api::IntType::Overwrite(0),
		menu_change_start_delay_frame: the_csk_collection_api::IntType::Overwrite(0),
		menu_change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
		menu_change_stop_delay_frame: the_csk_collection_api::IntType::Overwrite(0)
	});

	the_csk_collection_api::add_stream_property_entry_info(&the_csk_collection_api::StreamPropertyEntry {
		stream_id: smash::hash40("stream_meiling_victory"),
		clone_from_stream_id: None,
		data_name0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("meiling_victory")),
		data_name1: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		data_name2: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		data_name3: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		data_name4: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		loop_track: the_csk_collection_api::UnsignedByteType::Overwrite(0),
		end_point: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		fadeout_frame: the_csk_collection_api::UnsignedShortType::Overwrite(0),
		start_point_suddendeath: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		start_point_transition: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		start_point0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		start_point1: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		start_point2: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		start_point3: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("")),
		start_point4: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new(""))
	});

	the_csk_collection_api::add_stream_set_entry_info(&the_csk_collection_api::StreamSetEntry {
		stream_set_id: smash::hash40("set_meiling_victory"),
    	clone_from_stream_set_id: None,
    	special_category: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info0: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("info_meiling_victory")),
    	info1: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info2: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info3: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info4: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info5: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info6: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info7: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info8: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info9: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info10: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info11: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info12: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info13: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info14: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
    	info15: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("")),
	});

	the_csk_collection_api::add_new_bgm_property_entry(the_csk_collection_api::BgmPropertyEntry {
		stream_name: smash::hash40("stream_meiling_victory"),
		loop_start_ms: the_csk_collection_api::UnsignedIntType::Overwrite(0),
		loop_start_sample: the_csk_collection_api::UnsignedIntType::Overwrite(0),
		loop_end_ms: the_csk_collection_api::UnsignedIntType::Overwrite(10000),
		loop_end_sample: the_csk_collection_api::UnsignedIntType::Overwrite(4800000),
		duration_ms: the_csk_collection_api::UnsignedIntType::Overwrite(10000),
		duration_sample: the_csk_collection_api::UnsignedIntType::Overwrite(4800000)
	}); */

/* 
pub struct BgmPropertyEntry {
    /// Hashed name of the BGM stream.
    pub stream_name: Hash40,

    /// Beginning of the BGM stream's loop measured in milliseconds.
    pub loop_start_ms: u32,

    /// Beginning of the BGM stream's loop measured in samples.
    pub loop_start_sample: u32,

    /// End of the BGM stream's loop measured in milliseconds.
    pub loop_end_ms: u32,

    /// End of the BGM stream's loop measured in samples.
    pub loop_end_sample: u32,

    /// Duration of the BGM stream measured in milliseconds.
    pub duration_ms: u32,

    /// Duration of the BGM stream measured in samples.
    #[brw(pad_after = 4)]
    pub duration_sample: u32,
}

	 */

	//the_csk_collection_api::set_fighter_jingle(smash::hash40("ui_chara_meiling"), "meiling_victory");


	

	normals::install();
	aerials::install();
	specials::install();
	misc::install();
	finalsmash::install();
}

pub unsafe fn is_meiling(boma: *mut BattleObjectModuleAccessor) -> bool {
	let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
	crate::MARKED_COLORS[color as usize]

	//let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
	//return smash::app::utility::get_kind(&mut *boma) == *FIGHTER_KIND_DOLLY && color >= 90 && color <= 99;
}

#[skyline::main(name = "smashline_test")]
pub fn main() {
	unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
    
	/* normals::install();
	aerials::install();
	specials::install();
	misc::install();
	finalsmash::install(); */
}