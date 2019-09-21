// requires the rand crate
use std::fs;
use std::fs::File;
use std::io::Result;
// use std::io::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

// use std::io::Read;
use std::io::Write;
// use std::fs::OpenOptions;
/* 

TODO: 
    - heat
    - hull multiple
    - reload
    - do an actual damage balance
    - better effects/sounds
    - more namegen words
    - maybe read from an extraction for the lists
    - move the file generation around 
*/


use serde_derive::Deserialize;
use std::iter::Map;
enum Profile {
    Small,
    Medium,
    Large,
    ExtraLarge
}
#[derive(Deserialize)]
struct ProfileData {
    modifiers: Vec<String>,
    weapons: Vec<String>,
    turrets: Vec<String>,
    beams: Vec<String>,
    projectiles: Vec<String>,
}

#[derive(Deserialize)]
struct Effects {
    impact: Vec<String>,
    muzzle: Vec<String>,
}
#[derive(Deserialize)]
struct Art {
    icons: Vec<String>,
    effects: Effects,
}
#[derive(Deserialize)]
struct Modifiers {
    weak: Vec<String>,
    strong: Vec<String>,
    guntype: Vec<String>,
    gunmethod: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    page_id: u32,
    template_path: String,
    target_path: String,
    prodwares: Vec<String>,
    factions: Vec<String>,
    art: Art,
    general_modifiers: modifiers,
    small: ProfileData,
    medium: ProfileData,
    large: ProfileData,
    extralarge: ProfileData
}
struct Ship<'a> {
    weapon: &'a str,
    bullet: &'a str,
    bullet_macroname: &'a str,
    ware: &'a str,
    weapon_component: &'a str,

    icon_name: &'a str,
    laser: bool,
    rate_of_fire: f64,

    damage_min: i32,
    damage_max: i32,
    damage_flat: i32,
    speed: i32,
    weapon_range: i32,
    speed_vec: i32,

    impact_effect: &'a str,
    muzzle_effect: &'a str,
    weapon_system: &'a str,
    weapon_or_turret: &'a str,

    rot_max: i32,
    rot_accel: i32,
    reload_time: i32,
    hull_value: i32,

    price_min: i32,
    price_avg: i32,
    price_max: i32,

    factions_ware: &'a str,
    class: &'a str,

    prodware_one_name: &'a str,
    prodware_one_qty: i32,
    prodware_two_name: &'a str,
    prodware_two_qty: i32,
    prodware_three_name: &'a str,
    prodware_three_qty: i32,
    compname: &'a str,
    macroname: &'a str,
    tname: &'a str,
}
fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn replace_string(template: &mut String, word: &str, txt: &str) -> () {
    let word_start = template.find(word).unwrap_or(template.len());
    let word_end = word_start + word.len();

    template.replace_range(word_start..word_end, &txt.to_string());
}

fn choose_random(choosefrom: &Vec<String>) -> String {
    let returnthing = choosefrom
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    return returnthing;
}
fn main() -> Result<()> {
    //parsing the "words" as stuff between whitespace
    let toml_str = fs::read_to_string("Config.toml").unwrap();
    let conf = toml_str.parse::<Config>().unwrap();
    let art = conf.art;
    // tfiles
    let page_id = conf.general.page_id;
    let mut wep_name_id = 1;
    let mut wep_base_id = 2;
    let mut wep_short_id = 3;
    let mut wep_desc_id = 4;

    let mut ware_name_id = 1;
    let mut ware_base_id = 2;
    let mut ware_short_id = 3;
    let mut ware_desc_id = 4;
    //search terms

    let macroname = "macroname";

    // paths -> options -> strings
    let target_path = conf.target_path;
    let templates_path = conf.templates_path;
    let path_index = templates_path+"xml_read_index.xml";
    let path_tfile = target_path+"tfiles.xml";

    let mut ware_file_string = " ".to_string();

    let unwrapped_weapon = read_file(templates_path+"xml_read_weapon.xml");
    let unwrapped_bullet = read_file(templates_path+"xml_read_bullet.xml");
    let unwrapped_ware =   read_file(templates_path+"xml_read_ware.xml");
    let mut unwrapped_index = read_file(path_index);
    let mut t_string = " ".to_string();
    // string crap
    let weapon = "_weapon_";
    let bullet = "_bullet_";
    let ware = "_ware_";
    let xml = ".xml";
    // <entry name="tpwareq_pulse_xlarge_proj_bullet_macro" value="extensions\tpwar_release\equipment\weapons\macros\tpwar_bullet_macros" />
    let new_entry_line = "
    <entry name=\"";
    let value = "\" value=\"extensions\\tpwar_release\\equipment\\weapons\\macros\\";
    let closing = "\"  />";
    // end string crap

    let macro_count = 500;
    let min_range = 0;

    let mut wep_vec = vec![];
    let mut bul_vec = vec![];
    let mut ware_vec = vec![];
    let mut bullet_macroname_vec: Vec<String> = vec![];
    let mut wep_comp_vec: Vec<String> = vec![];

    let mut iconname_vec: Vec<String> = vec![];
    let mut laserbool_vec: Vec<i32> = vec![];
    let mut rateoffire_vec: Vec<f64> = vec![];
    let mut mindamage_vec: Vec<i32> = vec![];
    let mut maxdamage_vec: Vec<i32> = vec![];
    let mut flatdamage_vec: Vec<i32> = vec![];
    let mut speed_vec: Vec<i32> = vec![];
    let mut weaponrange_vec: Vec<i32> = vec![];
    let mut impactname_vec: Vec<String> = vec![];
    let mut launchname_vec: Vec<String> = vec![];
    let mut weaponsystem_vec: Vec<String> = vec![];
    let mut weaponorturret_vec: Vec<String> = vec![];

    let mut rotmax_vec: Vec<i32> = vec![];
    let mut rotacc_vec: Vec<i32> = vec![];
    let mut reloadtime_vec: Vec<i32> = vec![];
    let mut hullvalue_vec: Vec<i32> = vec![];

    let mut minprice_vec: Vec<i64> = vec![];
    let mut avgprice_vec: Vec<i64> = vec![];
    let mut maxprice_vec: Vec<i64> = vec![];

    let mut factions_ware_vec: Vec<String> = vec![];
    let mut class_vec: Vec<String> = vec![];

    let mut prodwareone_vec: Vec<String> = vec![];
    let mut prodwaretwo_vec: Vec<String> = vec![];
    let mut prodwarethree_vec: Vec<String> = vec![];
    let mut prodwareone_amount_vec: Vec<i32> = vec![];
    let mut prodwaretwo_amount_vec: Vec<i32> = vec![];
    let mut prodwarethree_amount_vec: Vec<i32> = vec![];

    let mut compname_vec: Vec<String> = vec![];
    let mut macroname_vec: Vec<String> = vec![];
    let mut tname_vec: Vec<String> = vec![];
    // profile
    let mut turret_m_art = art.medium.turrets;
    let mut turret_l_art = art.large.turrets;
    // wep
    let class = vec!["weapon", "turret"];
    let mut weapon_l_art = art.large.weapons;
    let mut weapon_m_art = art.medium.weapons;
    let mut weapon_s_art  = art.small.weapons;
    // bullet comps
    let mut bullet_l_art = art.large.projectiles;
    let mut bullet_m_beam_art = art.medium.beams;
    let mut bullet_s_beam_art = art.small.beams;
    let mut bullet_m_art = art.medium.projectiles;
    let mut bullet_s_art = art.small.projectiles;
    // icons
    let mut wep_icons = art.icons;
    //effects
    let mut impact_vec = art.effects.impact;
    let mut launch_vec = art.effects.muzzle;
    // wares
    let mut prodwares_vec = conf.general.prodwares;
    // factions
    let mut factions_vec = conf.general.factions;
    let mut small_mod = conf.small.modifiers;
    let mut medium_mod = conf.medium.modifiers;
    let mut large_mod = conf.large.modifiers;

    let mut weak_mod = conf.general_modifiers.weak;
    let mut strong_mod = conf.general_modifiers.weak;
    let mut gun_type = conf.general_modifiers.guntype;
    let mut gun_method = conf.general_modifiers.gunmethod;
    let profiles = vec![Profile::Small, Profile::Medium, Profile::Large, Profile::ExtraLarge,];
    let mut profile_vec = vec![];

    for i in min_range..macro_count {
        profile_vec.push(profiles.choose(&mut rand::thread_rng()));
        print!(
            "
        {:?}profile_vec !!!!!!!!!!!",
            profile_vec[i]
        );
    }

    // Particle
    // Cannon
    // Impulse
    // Ray
    // Emitter
    // Mass
    // Driver
    // Particle
    // Accelerator
    // Cannon
    // Plasma
    // Gun
    // Electro
    // Magnetic
    // Particle
    // Cannon
    // Energy
    // Bolt
    // Chaingun
    // Frag
    // Bomb
    // Launcher
    // High
    // Energy
    // Plasma
    // Thrower
    // Ion
    // Disruptor
    // Phased
    // Repeater
    // Gun
    // Plasma
    // Burst
    // Generator
    // Pulsed
    // Beam
    // Emitter
    // Concussion
    // Impulse
    // Generator
    // Gamma
    // Ray
    // Ion
    // Pulse
    // Generator
    // Ion
    // Shard
    // Railgun
    // Matter
    // Antimatter
    // Launcher
    // Phased
    // Shockwave
    // Generator
    // Cluster
    // Flak
    // Array
    // Flak
    // Artillery
    // Array
    // Fusion
    // Beam
    // Cannon
    // Gauss
    // Cannon
    // Incendiary
    // Bomb
    // Launcher
    // Ion
    // Cannon
    // Phased
    // Array
    // Laser
    // Cannon
    // Photon
    // Artillery
    // Array
    // Photon
    // Pulse
    // Cannon
    // Point
    // Singularity
    // Projector
    // Starburst
    // Shockwave
    // Cannon
    // Tri
    // Beam

    //
    //
    //
    //
    // 800mm
    // Quad
    // Blaster
    // Hellhound
    // Fixed
    // Turreted
    // Plaron
    // Triphammers
    // Graviton

    // Radar
    // Heavy
    // Hail
    // Thunderhead

    let mut prng = rand::thread_rng();

    for i in min_range..macro_count {
        wep_vec.push(unwrapped_weapon.clone());
        bul_vec.push(unwrapped_bullet.clone());
        ware_vec.push(unwrapped_ware.clone());

        // INVARIANT!! = all lists must be the same size as they are indexed across time and space to be with their friends
        match profile_vec[i].unwrap() {
            Profile::Small => {
                class_vec.push("weapons".to_string());
                weaponorturret_vec.push("weapon".to_string());
                laserbool_vec.push(0);
                if laserbool_vec[i] == 0 {
                    compname_vec.push(choose_random(&mut bullet_s_art).to_string())
                } else {
                    compname_vec.push(choose_random(&mut bullet_s_beam_art).to_string())
                  
                }

                wep_comp_vec.push(
                    weapon_s_art
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string(),
                );
                weaponrange_vec.push(prng.gen_range(2, 10));
                iconname_vec.push(choose_random(&mut wep_icons).to_string());
                rateoffire_vec.push(prng.gen_range(0.5, 5.0));
                mindamage_vec.push(prng.gen_range(10, 50));
                maxdamage_vec.push(prng.gen_range(50, 100));
                flatdamage_vec.push(prng.gen_range(maxdamage_vec[i], 100));
                speed_vec.push(prng.gen_range(500, 2000));
                weaponrange_vec.push(prng.gen_range(2, 9));
                impactname_vec.push(choose_random(&mut impact_vec).to_string());
                launchname_vec.push(choose_random(&mut launch_vec).to_string());
                weaponsystem_vec.push("weapon_standard".to_string());

                rotmax_vec.push(prng.gen_range(10, 240));
                rotacc_vec.push(prng.gen_range(10, 240));
                reloadtime_vec.push(prng.gen_range(1, 10));
                hullvalue_vec.push(prng.gen_range(500, 2000));

                let dps = ((mindamage_vec[i] + maxdamage_vec[i]) / 2 + flatdamage_vec[i]) as f64
                    * rateoffire_vec[i];
                print!(
                    "
                small: {:?}",
                    dps
                );
                minprice_vec
                    .push(prng.gen_range((dps * 100 as f64) as i64, (dps * 200 as f64) as i64));
                avgprice_vec
                    .push(prng.gen_range((dps * 200 as f64) as i64, (dps * 300 as f64) as i64));
                maxprice_vec
                    .push(prng.gen_range((dps * 300 as f64) as i64, (dps * 400 as f64) as i64));
                let name = choose_random(if dps >= 300 as f64 {
                    &mut strong_mod
                } else {
                    &mut weak_mod
                }) + &choose_random(&mut small_mod)
                    + &choose_random(&mut gun_type)
                    + &choose_random(&mut gun_method);
                tname_vec.push(name.clone());
                bullet_macroname_vec.push(name.clone() + "bullet_" + "macro");
                macroname_vec.push(name + "macro");

                prodwareone_vec.push(choose_random(&mut prodwares_vec));
                prodwaretwo_vec.push(choose_random(&mut prodwares_vec));
                prodwarethree_vec.push(choose_random(&mut prodwares_vec));
                prodwareone_amount_vec.push(prng.gen_range(10, 240));
                prodwaretwo_amount_vec.push(prng.gen_range(10, 240));
                prodwarethree_amount_vec.push(prng.gen_range(10, 240));

                factions_ware_vec.push(choose_random(&mut factions_vec));
            }
            Profile::Medium => {
                weaponorturret_vec.push(class.choose(&mut rand::thread_rng()).unwrap().to_string());
                laserbool_vec.push(0);
                if laserbool_vec[i] == 0 {
                    compname_vec.push(choose_random(&mut bullet_m_art).to_string())
                } else {
                    compname_vec.push(choose_random(&mut bullet_m_beam_art).to_string())
                }
               
                if weaponorturret_vec[i] == "weapon".to_string() {
                    wep_comp_vec.push(
                        weapon_m_art
                            .choose(&mut rand::thread_rng())
                            .unwrap()
                            .to_string(),
                    );
                    class_vec.push("weapons".to_string());
                } else {
                    wep_comp_vec.push(
                        turret_m_art
                            .choose(&mut rand::thread_rng())
                            .unwrap()
                            .to_string(),
                    );
                    class_vec.push("turrets".to_string());
                }

                weaponrange_vec.push(prng.gen_range(4, 10));
                iconname_vec.push(choose_random(&mut wep_icons).to_string());
                rateoffire_vec.push(prng.gen_range(0.5, 5.0));
                mindamage_vec.push(prng.gen_range(10, 50));
                maxdamage_vec.push(prng.gen_range(50, 100));
                flatdamage_vec.push(prng.gen_range(maxdamage_vec[i], 100));
                speed_vec.push(prng.gen_range(500, 2000));
                weaponrange_vec.push(prng.gen_range(2, 9));
                impactname_vec.push(choose_random(&mut impact_vec).to_string());
                launchname_vec.push(choose_random(&mut launch_vec).to_string());
                weaponsystem_vec.push("weapon_standard".to_string());
                rotmax_vec.push(prng.gen_range(10, 240));
                rotacc_vec.push(prng.gen_range(10, 240));
                reloadtime_vec.push(prng.gen_range(1, 10));
                hullvalue_vec.push(prng.gen_range(500, 2000));
                let dps = ((mindamage_vec[i] + maxdamage_vec[i]) / 2 + flatdamage_vec[i]) as f64
                    * rateoffire_vec[i];
                print!(
                    "
                medium: {:?}",
                    dps
                );
                minprice_vec
                    .push(prng.gen_range((dps * 100 as f64) as i64, (dps * 200 as f64) as i64));
                avgprice_vec
                    .push(prng.gen_range((dps * 200 as f64) as i64, (dps * 300 as f64) as i64));
                maxprice_vec
                    .push(prng.gen_range((dps * 300 as f64) as i64, (dps * 400 as f64) as i64));
                let name = choose_random(if dps >= 300 as f64 {
                    &mut strong_mod
                } else {
                    &mut weak_mod
                }) + &choose_random(&mut medium_mod)
                    + &choose_random(&mut gun_type)
                    + &choose_random(&mut gun_method);
                tname_vec.push(name.clone());
                bullet_macroname_vec.push(name.clone() + "bullet_" + "macro");
                macroname_vec.push(name + "macro");

                prodwareone_vec.push(choose_random(&mut prodwares_vec));
                prodwaretwo_vec.push(choose_random(&mut prodwares_vec));
                prodwarethree_vec.push(choose_random(&mut prodwares_vec));
                prodwareone_amount_vec.push(prng.gen_range(10, 240));
                prodwaretwo_amount_vec.push(prng.gen_range(10, 240));
                prodwarethree_amount_vec.push(prng.gen_range(10, 240));

                factions_ware_vec.push(choose_random(&mut factions_vec));
            }
            Profile::Large => {
                weaponorturret_vec.push(class.choose(&mut rand::thread_rng()).unwrap().to_string());
                laserbool_vec.push(0);
                if laserbool_vec[i] == 0 {
                    compname_vec.push(choose_random(&mut bullet_m_art).to_string())
                } else {
                    compname_vec.push(choose_random(&mut bullet_m_beam_art).to_string())
                }
               
                if weaponorturret_vec[i] == "weapon".to_string() {
                    wep_comp_vec.push(
                        weapon_l_art
                            .choose(&mut rand::thread_rng())
                            .unwrap()
                            .to_string(),
                    );
                    class_vec.push("weapons".to_string());
                } else {
                    wep_comp_vec.push(
                        turret_l_art
                            .choose(&mut rand::thread_rng())
                            .unwrap()
                            .to_string(),
                    );
                    class_vec.push("turrets".to_string());
                }
                wep_comp_vec.push(
                    weapon_l_art
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string(),
                );
                weaponrange_vec.push(prng.gen_range(6, 12));
                iconname_vec.push(choose_random(&mut wep_icons).to_string());
                rateoffire_vec.push(prng.gen_range(0.5, 5.0));
                mindamage_vec.push(prng.gen_range(10, 50));
                maxdamage_vec.push(prng.gen_range(50, 100));
                flatdamage_vec.push(prng.gen_range(maxdamage_vec[i], 100));
                speed_vec.push(prng.gen_range(500, 2000));
                weaponrange_vec.push(prng.gen_range(2, 9));
                impactname_vec.push(choose_random(&mut impact_vec).to_string());
                launchname_vec.push(choose_random(&mut launch_vec).to_string());
                weaponsystem_vec.push("weapon_standard".to_string());
                rotmax_vec.push(prng.gen_range(10, 240));
                rotacc_vec.push(prng.gen_range(10, 240));
                reloadtime_vec.push(prng.gen_range(1, 10));
                hullvalue_vec.push(prng.gen_range(500, 2000));
                let dps = ((mindamage_vec[i] + maxdamage_vec[i]) / 2 + flatdamage_vec[i]) as f64
                    * rateoffire_vec[i];
                print!(
                    "
                large: {:?}",
                    dps
                );
                minprice_vec
                    .push(prng.gen_range((dps * 100 as f64) as i64, (dps * 200 as f64) as i64));
                avgprice_vec
                    .push(prng.gen_range((dps * 200 as f64) as i64, (dps * 300 as f64) as i64));
                maxprice_vec
                    .push(prng.gen_range((dps * 300 as f64) as i64, (dps * 400 as f64) as i64));
                let name = choose_random(if dps >= 300 as f64 {
                    &mut strong_mod
                } else {
                    &mut weak_mod
                }) + &choose_random(&mut large_mod)
                    + &choose_random(&mut gun_type)
                    + &choose_random(&mut gun_method);
                tname_vec.push(name.clone());
                bullet_macroname_vec.push(name.clone() + "bullet_" + "macro");
                macroname_vec.push(name + "macro");

                prodwareone_vec.push(choose_random(&mut prodwares_vec));
                prodwaretwo_vec.push(choose_random(&mut prodwares_vec));
                prodwarethree_vec.push(choose_random(&mut prodwares_vec));
                prodwareone_amount_vec.push(prng.gen_range(10, 240));
                prodwaretwo_amount_vec.push(prng.gen_range(10, 240));
                prodwarethree_amount_vec.push(prng.gen_range(10, 240));

                factions_ware_vec.push(choose_random(&mut factions_vec));
            }
            Profile::Large => {
                weaponorturret_vec.push(class.choose(&mut rand::thread_rng()).unwrap().to_string());
                laserbool_vec.push(0);
                if laserbool_vec[i] == 0 {
                    compname_vec.push(choose_random(&mut bullet_m_art).to_string())
                } else {
                    compname_vec.push(choose_random(&mut bullet_m_beam_art).to_string())
                }
                if weaponorturret_vec[i] == "weapon".to_string() {
                    wep_comp_vec.push(
                        weapon_l_art
                            .choose(&mut rand::thread_rng())
                            .unwrap()
                            .to_string(),
                    );
                    class_vec.push("weapons".to_string());
                } else {
                    wep_comp_vec.push(
                        turret_l_art
                            .choose(&mut rand::thread_rng())
                            .unwrap()
                            .to_string(),
                    );
                    class_vec.push("turrets".to_string());
                }
                wep_comp_vec.push(
                    weapon_l_art
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string(),
                );
                weaponrange_vec.push(prng.gen_range(8, 14));
                iconname_vec.push(choose_random(&mut wep_icons).to_string());
                rateoffire_vec.push(prng.gen_range(10.0, 50.0));
                mindamage_vec.push(prng.gen_range(1000, 5000));
                maxdamage_vec.push(prng.gen_range(5000, 50000));
                flatdamage_vec.push(prng.gen_range(maxdamage_vec[i], 50000));
                speed_vec.push(prng.gen_range(500, 2000));
                weaponrange_vec.push(prng.gen_range(2, 9));
                impactname_vec.push(choose_random(&mut impact_vec).to_string());
                launchname_vec.push(choose_random(&mut launch_vec).to_string());
                weaponsystem_vec.push("weapon_standard".to_string());
                rotmax_vec.push(prng.gen_range(10, 240));
                rotacc_vec.push(prng.gen_range(10, 240));
                reloadtime_vec.push(prng.gen_range(1, 10));
                hullvalue_vec.push(prng.gen_range(500, 2000));

                let dps = ((mindamage_vec[i] + maxdamage_vec[i]) / 2 + flatdamage_vec[i]) as f64
                    * rateoffire_vec[i];
                print!(
                    "
                xl: {:?}",
                    dps
                );
                minprice_vec
                    .push(prng.gen_range((dps * 100 as f64) as i64, (dps * 200 as f64) as i64));
                avgprice_vec
                    .push(prng.gen_range((dps * 200 as f64) as i64, (dps * 300 as f64) as i64));
                maxprice_vec
                    .push(prng.gen_range((dps * 300 as f64) as i64, (dps * 400 as f64) as i64));
                let name = choose_random(if dps >= 300 as f64 {
                    &mut strong_mod
                } else {
                    &mut weak_mod
                }) + &choose_random(&mut large_mod)
                    + &choose_random(&mut gun_type)
                    + &choose_random(&mut gun_method);
                tname_vec.push(name.clone());
                bullet_macroname_vec.push(name.clone() + "bullet_" + "macro");
                macroname_vec.push(name + "macro");

                prodwareone_vec.push(choose_random(&mut prodwares_vec));
                prodwaretwo_vec.push(choose_random(&mut prodwares_vec));
                prodwarethree_vec.push(choose_random(&mut prodwares_vec));
                prodwareone_amount_vec.push(prng.gen_range(10, 240));
                prodwaretwo_amount_vec.push(prng.gen_range(10, 240));
                prodwarethree_amount_vec.push(prng.gen_range(10, 240));

                factions_ware_vec.push(choose_random(&mut factions_vec));
            }
            _ => print!(
                "ERROR - no profile match!!!
            "
            ),
        }
    }

    for i in min_range..macro_count {
        wep_name_id += 100;
        wep_base_id += 100;
        wep_short_id += 100;
        wep_desc_id += 100;

        let mut mutable_template = wep_vec[i].clone();
        for word in wep_vec[i].split_whitespace() {
            match word {
                "macroname" => replace_string(
                    &mut mutable_template,
                    word,
                    &macroname_vec[i].replace(" ", "_").to_lowercase(),
                ),
                "bulletmacroname" => replace_string(
                    &mut mutable_template,
                    word,
                    &bullet_macroname_vec[i].replace(" ", "_").to_lowercase(),
                ),
                "compname" => {
                    replace_string(&mut mutable_template, word, &wep_comp_vec[i].to_string())
                }
                "pageident" => replace_string(&mut mutable_template, word, &page_id.to_string()),
                "nameid" => {
                    wep_name_id += 1;
                    replace_string(&mut mutable_template, word, &wep_name_id.to_string());
                    t_string.push_str(&format!(
                        "
                    <t id={:?}>{}</t>",
                        &wep_name_id.to_string(),
                        tname_vec[i]
                    ))
                }
                "baseid" => {
                    wep_base_id += 1;
                    replace_string(&mut mutable_template, word, &wep_base_id.to_string());
                    t_string.push_str(&format!(
                        "
                    <t id={:?}>{}</t>",
                        &wep_base_id.to_string(),
                        tname_vec[i]
                    ))
                }
                "shortid" => {
                    wep_short_id += 1;
                    replace_string(&mut mutable_template, word, &wep_short_id.to_string());
                    t_string.push_str(&format!(
                        "
                    <t id={:?}>{}</t>",
                        &wep_short_id.to_string(),
                        tname_vec[i]
                    ))
                }
                "descid" => {
                    wep_desc_id += 1;
                    replace_string(&mut mutable_template, word, &wep_desc_id.to_string());
                    t_string.push_str(&format!(
                        "
                    <t id={:?}>{}</t>",
                        &wep_desc_id.to_string(),
                        tname_vec[i]
                    ))
                }
                "rotmax" => replace_string(&mut mutable_template, word, &rotmax_vec[i].to_string()),
                "rotacc" => replace_string(&mut mutable_template, word, &rotacc_vec[i].to_string()),
                "reloadtime" => {
                    replace_string(&mut mutable_template, word, &reloadtime_vec[i].to_string())
                }
                "hullvalue" => {
                    replace_string(&mut mutable_template, word, &hullvalue_vec[i].to_string())
                }
                "weaponorturret" => replace_string(
                    &mut mutable_template,
                    word,
                    &weaponorturret_vec[i].to_string(),
                ),

                _ => (),
            }
        }

        // overwrite weapon file
        let weapon_result_path = format!("{}{}{}{}{}", target_path, macroname, weapon, i, xml);
        let mut file = File::create(&weapon_result_path)?;
        file.write_all(
            mutable_template
                .replace("  \"", "\"")
                .replace("\"  ", "\"")
                .replace("\"{ ", "\"{")
                .replace(" }\"", "}\"")
                .replace(" , ", ",")
                .as_bytes(),
        )?;
        print!("{:?}", weapon_result_path);

        // ware

        ware_name_id += 100;
        ware_base_id += 100;
        ware_short_id += 100;
        ware_desc_id += 100;

        let mut mutable_ware_template = ware_vec[i].clone();
        for word in ware_vec[i].split_whitespace() {
            match word {
                "macroname" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &macroname_vec[i].replace(" ", "_").to_lowercase(),
                ),
                "pageident" => {
                    replace_string(&mut mutable_ware_template, word, &page_id.to_string())
                }
                "nameid" => {
                    ware_name_id += 1;
                    replace_string(&mut mutable_ware_template, word, &ware_name_id.to_string())
                }
                "baseid" => {
                    ware_base_id += 1;
                    replace_string(&mut mutable_ware_template, word, &ware_base_id.to_string())
                }
                "shortid" => {
                    ware_short_id += 1;
                    replace_string(&mut mutable_ware_template, word, &ware_short_id.to_string())
                }
                "descid" => {
                    ware_desc_id += 1;
                    replace_string(&mut mutable_ware_template, word, &ware_desc_id.to_string())
                }
                "minprice" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &minprice_vec[i].to_string(),
                ),
                "avgprice" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &avgprice_vec[i].to_string(),
                ),
                "maxprice" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &maxprice_vec[i].to_string(),
                ),
                "classgroup" => {
                    replace_string(&mut mutable_ware_template, word, &class_vec[i].to_string())
                }
                "weaponorturret" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &weaponorturret_vec[i].to_string(),
                ),

                "prodwareone" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &prodwareone_vec[i].to_string(),
                ),
                "prodwaretwo" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &prodwaretwo_vec[i].to_string(),
                ),
                "prodwarethree" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &prodwarethree_vec[i].to_string(),
                ),
                "prodwareone_amount" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &prodwareone_amount_vec[i].to_string(),
                ),
                "prodwaretwo_amount" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &prodwaretwo_amount_vec[i].to_string(),
                ),
                "prodwarethree_amount" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &prodwarethree_amount_vec[i].to_string(),
                ),
                "factions_ware" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &factions_ware_vec[i].to_string(),
                ),
                "wareid" => replace_string(
                    &mut mutable_ware_template,
                    word,
                    &macroname_vec[i].replace(" ", "_").to_lowercase(),
                ),

                _ => (),
            }
        }
        // append ware string
        ware_file_string.push_str(
            "
        ",
        );
        ware_file_string.push_str(&mutable_ware_template);

        // index entries
        let newentry = format!(
            "{}{}{}{}{}",
            new_entry_line,
            &macroname_vec[i].replace(" ", "_").to_lowercase(),
            value,
            format!("{}{}{}", macroname, weapon, i),
            closing
        );
        let bulletnewentry = format!(
            "{}{}{}{}{}",
            new_entry_line,
            &bullet_macroname_vec[i].replace(" ", "_").to_lowercase(),
            value,
            format!("{}{}{}", macroname, bullet, i),
            closing
        );
        unwrapped_index.push_str(&newentry);
        unwrapped_index.push_str(&bulletnewentry);

        let mut mutable_bul_template = bul_vec[i].clone();
        for word in bul_vec[i].split_whitespace() {
            match word {
                "bulletmacroname" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &bullet_macroname_vec[i].replace(" ", "_").to_lowercase(),
                ),
                "iconname" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &iconname_vec[i].to_string(),
                ),
                "laserbool" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &laserbool_vec[i].to_string(),
                ),
                "rateoffire" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &rateoffire_vec[i].to_string(),
                ),
                "mindamage" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &mindamage_vec[i].to_string(),
                ),
                "maxdamage" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &maxdamage_vec[i].to_string(),
                ),
                "flatdamage" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &flatdamage_vec[i].to_string(),
                ),
                "speedvalue" => {
                    replace_string(&mut mutable_bul_template, word, &speed_vec[i].to_string())
                }
                "lifetimevalue" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &weaponrange_vec[i].to_string(),
                ),
                "impactname" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &impactname_vec[i].to_string(),
                ),
                "launchname" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &launchname_vec[i].to_string(),
                ),
                "weaponsystem" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &weaponsystem_vec[i].to_string(),
                ),

                "componentname" => replace_string(
                    &mut mutable_bul_template,
                    word,
                    &compname_vec[i].to_string(),
                ),
                _ => (),
            };
        }

        // overwrite bullet file
        let bullet_result_path = format!("{}{}{}{}{}", target_path, macroname, bullet, i, xml);
        let mut file = File::create(&bullet_result_path)?;
        file.write_all(
            mutable_bul_template
                .replace("  \"", "\"")
                .replace("\"  ", "\"")
                .as_bytes(),
        )?;
        print!("{:?}", bullet_result_path);
    }
    let ware_result_path = format!("{}{}{}{}", target_path, macroname, ware, xml);
    let mut file = File::create(&ware_result_path)?;
    file.write_all(
        ware_file_string
            .replace("  \"", "\"")
            .replace("\"  ", "\"")
            .replace("\"{ ", "\"{")
            .replace(" }\"", "}\"")
            .replace(" , ", ",")
            .as_bytes(),
    )?;
    print!("{:?}", ware_result_path);
    // overwrite index
    let mut indexfile = File::create(path_index)?;
    indexfile.write_all(unwrapped_index.as_bytes())?;
    // tfile
    let mut tfile = File::create(path_tfile)?;
    tfile.write_all(t_string.as_bytes())?;
    Ok(())
}

