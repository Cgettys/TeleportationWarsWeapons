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
fn main() -> Result<()> {
    //parsing the "words" as stuff between whitespace

    // tfiles
    let page_id = 12345678;
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

    let path_weapon = "C:/Users/alby/Desktop/rust_targets/xml_read_weapon.xml";
    let path_bullet = "C:/Users/alby/Desktop/rust_targets/xml_read_bullet.xml";
    let path_ware = "C:/Users/alby/Desktop/rust_targets/xml_read_ware.xml";
    let path_index = "C:/Users/alby/Desktop/rust_targets/xml_read_index.xml";
    let path_tfile = "C:/Users/alby/Desktop/rust_targets/tfiles.xml";

    let mut ware_file_string = " ".to_string();

    let unwrapped_weapon = fs::read_to_string(path_weapon).unwrap();
    let unwrapped_bullet = fs::read_to_string(path_bullet).unwrap();
    let unwrapped_ware = fs::read_to_string(path_ware).unwrap();
    let mut unwrapped_index = fs::read_to_string(path_index).unwrap();
    let mut t_string = " ".to_string();
    // string crap
    let weapon = "_weapon_";
    let bullet = "_bullet_";
    let ware = "_ware_";
    let target_path = "C:/Users/alby/Desktop/rust_targets/";
    let xml = ".xml";
    // <entry name="tpwareq_pulse_xlarge_proj_bullet_macro" value="extensions\tpwar_release\equipment\weapons\macros\tpwar_bullet_macros" />
    let new_entry_line = "
    <entry name=\"";
    let value = "\" value=\"extensions\\tpwar_release\\equipment\\weapons\\macros\\";
    let closing = "\"  />";
    // end string crap

    let macro_count = 20;
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
    let profile = vec!["small_gun", "medium_gun", "large_gun", "xl_gun"];
    let mut turret_m_art = vec![
        "turret_arg_m_beam_01_mk1",
        "turret_arg_m_beam_02_mk1",
        "turret_par_m_beam_01_mk1",
        "turret_par_m_beam_02_mk1",
        "turret_tel_m_beam_01_mk1",
        "turret_tel_m_beam_02_mk1",
        "turret_arg_m_gatling_01_mk1",
        "turret_arg_m_gatling_02_mk1",
        "turret_arg_m_plasma_01_mk1",
        "turret_arg_m_plasma_02_mk1",
        "turret_kha_m_beam_01_mk1",
        "turret_par_m_gatling_01_mk1",
        "turret_par_m_gatling_02_mk1",
        "turret_par_m_plasma_01_mk1",
        "turret_par_m_plasma_02_mk1",
        "turret_tel_m_gatling_01_mk1",
        "turret_tel_m_gatling_02_mk1",
        "turret_tel_m_plasma_01_mk1",
        "turret_tel_m_plasma_02_mk1",
        "turret_arg_m_laser_01_mk1",
        "turret_arg_m_laser_02_mk1",
        "turret_arg_m_shotgun_01_mk1",
        "turret_arg_m_shotgun_02_mk1",
        "turret_par_m_laser_01_mk1",
        "turret_par_m_laser_02_mk1",
        "turret_par_m_shotgun_01_mk1",
        "turret_par_m_shotgun_02_mk1",
        "turret_tel_m_laser_01_mk1",
        "turret_tel_m_laser_02_mk1",
        "turret_tel_m_shotgun_01_mk1",
        "turret_tel_m_shotgun_02_mk1",
        "turret_xen_m_laser_02_mk1",
        "turret_xen_m_laser_01_mk1",
        "turret_xen_m_laser_02_mk1",
    ];
    let mut turret_l_art = vec![
        "turret_arg_l_beam_01_mk1",
        "turret_par_l_beam_01_mk1",
        "turret_tel_l_beam_01_mk1",
        "turret_arg_l_plasma_01_mk1",
        "turret_par_l_plasma_01_mk1",
        "turret_tel_l_plasma_01_mk1",
        "turret_arg_l_laser_01_mk1",
        "turret_par_l_laser_01_mk1",
        "turret_tel_l_laser_01_mk1",
        "turret_xen_l_laser_01_mk1",
    ];
    // wep
    let class = vec!["weapon", "turret"];
    let mut weapon_l_art = vec![
        "weapon_arg_l_destroyer_01_mk1",
        "weapon_par_l_destroyer_01_mk1",
        "weapon_tel_l_destroyer_01_mk1",
        "weapon_arg_l_plasma_01_mk1",
        "weapon_par_l_railgun_01_mk1",
        "weapon_tel_l_beam_01_mk1",
        "weapon_gen_l_laser_01_mk1",
    ];
    let mut weapon_m_art = vec![
        "weapon_arg_m_ion_01_mk1",
        "weapon_arg_m_ion_01_mk2",
        "weapon_gen_m_beam_01_mk1",
        "weapon_gen_m_beam_01_mk2",
        "weapon_gen_m_gatling_01_mk1",
        "weapon_gen_m_gatling_01_mk2",
        "weapon_gen_m_plasma_01_mk1",
        "weapon_gen_m_plasma_01_mk2",
        "weapon_par_m_railgun_01_mk1",
        "weapon_par_m_railgun_01_mk2",
        "weapon_tel_m_charge_01_mk1",
        "weapon_tel_m_charge_01_mk2",
        "weapon_gen_m_laser_01_mk1",
        "weapon_gen_m_laser_01_mk2",
        "weapon_gen_m_shotgun_01_mk1",
        "weapon_gen_m_shotgun_01_mk2",
        "weapon_kha_m_laser_01_mk1",
        "weapon_xen_m_laser_01_mk1",
    ];
    let mut weapon_s_art = vec![
        "weapon_arg_s_ion_01_mk1",
        "weapon_arg_s_ion_01_mk2",
        "weapon_gen_s_beam_01_mk1",
        "weapon_gen_s_beam_01_mk2",
        "weapon_gen_s_gatling_01_mk1",
        "weapon_gen_s_gatling_01_mk2",
        "weapon_gen_s_plasma_01_mk1",
        "weapon_gen_s_plasma_01_mk2",
        "weapon_par_s_railgun_01_mk1",
        "weapon_par_s_railgun_01_mk2",
        "weapon_tel_s_charge_01_mk1",
        "weapon_tel_s_charge_01_mk2",
        "weapon_gen_s_laser_01_mk1",
        "weapon_gen_s_laser_01_mk2",
        "weapon_gen_s_shotgun_01_mk1",
        "weapon_gen_s_shotgun_01_mk2",
        "weapon_kha_s_laser_01_mk1",
        "weapon_xen_s_laser_01_mk1",
    ];
    // bullet comps
    let mut bullet_l_art = vec!["bullet_gen_l_laser_01_mk1", "bullet_xen_l_laser_01_mk1"];
    let mut bullet_m_beam_art = vec![
        "bullet_gen_m_beam_01_mk1",
        "bullet_gen_m_beam_01_mk2",
        "bullet_gen_m_beam_01_mk1",
        "bullet_kha_m_beam_01",
        "bullet_gen_m_beam_01_mk1",
    ];
    let mut bullet_s_beam_art = vec![
        "bullet_gen_s_beam_01_mk1",
        "bullet_gen_s_beam_01_mk2",
        "bullet_gen_s_beam_01_mk1",
        "bullet_kha_s_beam_01",
    ];
    let mut bullet_m_art = vec![
        "bullet_arg_m_ion_01_mk1",
        "bullet_arg_m_ion_01_mk2",
        "bullet_gen_m_gatling_01_mk1",
        "bullet_gen_m_gatling_01_mk2",
        "bullet_gen_m_laser_01_mk1",
        "bullet_gen_m_laser_01_mk2",
        "bullet_gen_m_plasma_01_mk1",
        "bullet_gen_m_plasma_01_mk2",
        "bullet_gen_m_shotgun_01_mk1",
        "bullet_gen_m_shotgun_01_mk2",
        "bullet_gen_m_gatling_01_mk1",
        "bullet_gen_m_laser_01_mk1",
        "bullet_gen_m_plasma_01_mk1",
        "bullet_gen_m_shotgun_01_mk1",
        "bullet_par_m_railgun_01_mk1",
        "bullet_par_m_railgun_01_mk2",
        "bullet_tel_m_charge_01_mk1",
        "bullet_tel_m_charge_01_mk2",
        "bullet_xen_m_laser_01_mk1",
        "bullet_xen_m_laser_01_mk1",
    ];
    let mut bullet_s_art = vec![
        "bullet_arg_s_ion_01_mk1",
        "bullet_arg_s_ion_01_mk2",
        "bullet_gen_s_gatling_01_mk1",
        "bullet_gen_s_gatling_01_mk2",
        "bullet_gen_s_laser_01_mk1",
        "bullet_gen_s_laser_01_mk2",
        "bullet_gen_s_plasma_01_mk1",
        "bullet_gen_s_plasma_01_mk2",
        "bullet_gen_s_shotgun_01_mk1",
        "bullet_gen_s_shotgun_01_mk2",
        "bullet_gen_s_gatling_01_mk1",
        "bullet_gen_s_laser_01_mk1",
        "bullet_gen_s_plasma_01_mk1",
        "bullet_gen_s_shotgun_01_mk1",
        "bullet_par_s_railgun_01_mk1",
        "bullet_par_s_railgun_01_mk2",
        "bullet_tel_s_charge_01_mk1",
        "bullet_tel_s_charge_01_mk2",
        "bullet_xen_s_laser_01_mk1",
        "bullet_xen_s_laser_01_mk1",
    ];
    // icons
    let mut wep_icons = vec![
        "weapon_empbomb_mk1",
        "weapon_bomb_mk1",
        "weapon_ion_mk1",
        "weapon_ion_mk2",
        "weapon_ion_mk1",
        "weapon_ion_mk2",
        "weapon_laser_mk1",
        "weapon_beam_mk1",
        "weapon_beam_mk2",
        "weapon_gatling_mk1",
        "weapon_gatling_mk2",
        "weapon_laser_mk1",
        "weapon_laser_mk2",
        "weapon_plasma_mk1",
        "weapon_plasma_mk2",
        "weapon_shotgun_mk1",
        "weapon_shotgun_mk2",
        "weapon_beam_mk1",
        "weapon_beam_mk2",
        "weapon_gatling_mk1",
        "weapon_gatling_mk2",
        "weapon_laser_mk1",
        "weapon_laser_mk2",
        "weapon_plasma_mk1",
        "weapon_plasma_mk2",
        "weapon_shotgun_mk1",
        "weapon_shotgun_mk2",
        "weapon_beam_mk1",
        "weapon_gatling_mk1",
        "weapon_laser_mk1",
        "weapon_plasma_mk1",
        "weapon_shotgun_mk1",
        "weapon_beam_mk1",
        "weapon_gatling_mk1",
        "weapon_laser_mk1",
        "weapon_plasma_mk1",
        "weapon_shotgun_mk1",
        "weapon_beam_mk2",
        "weapon_beam_mk1",
        "weapon_railgun_mk1",
        "weapon_railgun_mk2",
        "weapon_railgun_mk1",
        "weapon_railgun_mk2",
        "weapon_handlaser_mk1",
        "weapon_repairlaser_mk1",
        "weapon_charge_mk1",
        "weapon_charge_mk2",
        "weapon_charge_mk1",
        "weapon_charge_mk2",
        "weapon_laser_mk1",
        "weapon_laser_mk1",
        "weapon_laser_mk1",
        "weapon_laser_mk1",
        "weapon_beam_mk1",
    ];
    //effects
    let mut impact_vec = vec![
        "impact_arg_m_ion_01_mk1",
        "impact_arg_m_ion_01_mk1",
        "impact_arg_s_ion_01_mk1",
        "impact_arg_s_ion_01_mk1",
        "impact_gen_l_laser_01_mk1",
        "impact_gen_m_beam_01_mk1",
        "impact_gen_m_beam_01_mk1",
        "impact_gen_m_gatling_01_mk1",
        "impact_gen_m_gatling_01_mk1",
        "impact_gen_m_laser_01_mk1",
        "impact_gen_m_laser_01_mk1",
        "impact_gen_m_plasma_01_mk1",
        "impact_gen_m_plasma_01_mk1",
        "impact_gen_m_shotgun_01_mk1",
        "impact_gen_m_shotgun_01_mk1",
        "impact_gen_s_beam_01_mk1",
        "impact_gen_s_beam_01_mk1",
        "impact_gen_s_gatling_01_mk1",
        "impact_gen_s_gatling_01_mk1",
        "impact_gen_s_laser_01_mk1",
        "impact_gen_s_laser_01_mk1",
        "impact_gen_s_plasma_01_mk1",
        "impact_gen_s_plasma_01_mk1",
        "impact_gen_s_shotgun_01_mk1",
        "impact_gen_s_shotgun_01_mk1",
        "impact_gen_m_beam_01_mk1",
        "impact_gen_m_gatling_01_mk1",
        "impact_gen_m_laser_01_mk1",
        "impact_gen_m_plasma_01_mk1",
        "impact_gen_m_shotgun_01_mk1",
        "impact_gen_m_beam_01_mk1",
        "impact_gen_s_gatling_01_mk1",
        "impact_gen_s_laser_01_mk1",
        "impact_gen_s_plasma_01_mk1",
        "impact_gen_s_shotgun_01_mk1",
        "impact_gen_m_beam_01_mk1",
        "impact_gen_s_beam_01_mk1",
        "impact_par_m_railgun_01_mk1",
        "impact_par_m_railgun_01_mk1",
        "impact_par_s_railgun_01_mk1",
        "impact_par_s_railgun_01_mk1",
        "impact_gen_spacesuit_laser_01_mk1",
        "impact_spacesuit_repair_01_mk1",
        "impact_tel_m_charge_01_mk1",
        "impact_tel_m_charge_01_mk1",
        "impact_tel_s_charge_01_mk1",
        "impact_tel_s_charge_01_mk1",
        "impact_xen_m_laser_01_mk1",
        "impact_xen_m_laser_01_mk1",
        "impact_xen_s_laser_01_mk1",
        "impact_xen_m_laser_01_mk1",
        "impact_xen_s_laser_01_mk1",
    ];
    let mut launch_vec = vec![
        "muzzle_arg_m_ion_01_mk1",
        "muzzle_arg_m_ion_01_mk1",
        "muzzle_arg_s_ion_01_mk1",
        "muzzle_arg_s_ion_01_mk1",
        "muzzle_gen_l_laser_01_mk1",
        "muzzle_gen_m_beam_01_mk1",
        "muzzle_gen_m_beam_01_mk1",
        "muzzle_gen_m_gatling_01_mk1",
        "muzzle_gen_m_gatling_01_mk1",
        "muzzle_gen_m_laser_01_mk1",
        "muzzle_gen_m_laser_01_mk1",
        "muzzle_gen_m_plasma_01_mk1",
        "muzzle_gen_m_plasma_01_mk1",
        "muzzle_gen_m_shotgun_01_mk1",
        "muzzle_gen_m_shotgun_01_mk1",
        "muzzle_gen_s_beam_01_mk1",
        "muzzle_gen_s_beam_01_mk1",
        "muzzle_gen_s_gatling_01_mk1",
        "muzzle_gen_s_gatling_01_mk1",
        "muzzle_gen_s_laser_01_mk1",
        "muzzle_gen_s_laser_01_mk1",
        "muzzle_gen_s_plasma_01_mk1",
        "muzzle_gen_s_plasma_01_mk1",
        "muzzle_gen_s_shotgun_01_mk1",
        "muzzle_gen_s_shotgun_01_mk1",
        "muzzle_gen_l_beam_01_mk1",
        "muzzle_gen_m_gatling_01_mk1",
        "muzzle_gen_m_laser_01_mk1",
        "muzzle_gen_m_plasma_01_mk1",
        "muzzle_gen_m_shotgun_01_mk1",
        "muzzle_gen_m_beam_01_mk1",
        "muzzle_gen_s_gatling_01_mk1",
        "muzzle_gen_s_laser_01_mk1",
        "muzzle_gen_s_plasma_01_mk1",
        "muzzle_gen_s_shotgun_01_mk1",
        "muzzle_gen_m_beam_01_mk1",
        "muzzle_gen_s_beam_01_mk1",
        "muzzle_par_m_railgun_01_mk1",
        "muzzle_par_m_railgun_01_mk1",
        "muzzle_par_s_railgun_01_mk1",
        "muzzle_par_s_railgun_01_mk1",
        "muzzle_gen_spacesuit_laser_01_mk1",
        "muzzle_gen_s_repair_01_mk1",
        "muzzle_tel_m_charge_01_mk1",
        "muzzle_tel_m_charge_01_mk1",
        "muzzle_tel_s_charge_01_mk1",
        "muzzle_tel_s_charge_01_mk1",
        "muzzle_xen_m_laser_01_mk1",
        "muzzle_xen_m_laser_01_mk1",
        "muzzle_xen_s_laser_01_mk1",
        "muzzle_xen_m_laser_01_mk1",
        "muzzle_xen_s_laser_01_mk1",
        "wpn_flare_launcher",
    ];
    // wares
    let mut prodwares_vec = vec![
        "advancedcomposites",
        "advancedelectronics",
        "antimattercells",
        "antimatterconverters",
        "claytronics",
        "dronecomponents",
        "energycells",
        "engineparts",
        "fieldcoils",
        "foodrations",
        "graphene",
        "hullparts",
        "majadust",
        "majasnails",
        "meat",
        "medicalsupplies",
        "microchips",
        "missilecomponents",
        "nostropoil",
        "plasmaconductors",
        "quantumtubes",
        "refinedmetals",
        "scanningarrays",
        "shieldcomponents",
        "siliconwafers",
        "smartchips",
        "sojabeans",
        "sojahusk",
        "spacefuel",
        "spaceweed",
        "spices",
        "sunriseflowers",
        "superfluidcoolant",
        "swampplant",
        "teladianium",
        "turretcomponents",
        "water",
        "weaponcomponents",
        "wheat",
    ];
    // factions
    let mut factions_vec = vec![
        "khaakpox",
        "xenonmatrix",
        "atlas",
        "sovsyn",
        "cartel",
        "agi",
        "goner",
        "wholefood",
        "ledda",
        "telunion",
        "aldrin",
        "cantera",
        "toride",
        "nmmc",
        "atreus",
        "jonferson",
        "aquariuscorp",
        "ptni",
        "strongarms",
        "plutarch",
        "albionenergy",
        "terracorp",
        "franton",
        "chow",
        "beryll",
        "terrapmc",
        "otas",
        "buccaneers",
        "reivers",
        "terrans",
        "nolimits",
        "uguras",
        "heretics",
        "yaki",
        "usc",
        "heartofalbion",
        "sonraenergy",
    ];
    let mut small_mod = vec![
        "Alpha ", "Small ", "75mm ", "100mm ", "Light ", "Tiny ", "limited ",
    ];
    let mut medium_mod = vec!["150mm ", "200mm ", "Medium ", "Standard ", "Beta "];
    let mut large_mod = vec!["300mm ", "400mm ", "Large ", "Gamma ", "Heavy "];

    let mut weak_mod = vec![
        "Rusty ",
        "Weak ",
        "Damaged ",
        "Diminished ",
        "Fragile ",
        "Flawed ",
        "Impulse ",
    ];
    let mut strong_mod = vec!["Advanced ", "Phased ", "Array ", "Fragmentation "];
    let mut gun_type = vec!["Particle ", "Plasma ", "Ion ", "Artillery ", "Concussion "];
    let mut gun_method = vec![
        "Launcher ",
        "Driver ",
        "Railgun ",
        "Emitter ",
        "Ray ",
        "Cannon ",
        "Generator ",
        "Projector ",
    ];

    let mut profile_vec = vec![];

    for i in min_range..macro_count {
        profile_vec.push(profile.choose(&mut rand::thread_rng()));
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
    let mut irng = rand::thread_rng();
    turret_m_art.shuffle(&mut irng);
    turret_l_art.shuffle(&mut irng);
    weapon_l_art.shuffle(&mut irng);
    weapon_m_art.shuffle(&mut irng);
    weapon_s_art.shuffle(&mut irng);
    bullet_l_art.shuffle(&mut irng);
    bullet_m_beam_art.shuffle(&mut irng);
    bullet_s_beam_art.shuffle(&mut irng);
    bullet_m_art.shuffle(&mut irng);
    bullet_s_art.shuffle(&mut irng);
    wep_icons.shuffle(&mut irng);
    impact_vec.shuffle(&mut irng);
    launch_vec.shuffle(&mut irng);
    small_mod.shuffle(&mut irng);
    medium_mod.shuffle(&mut irng);
    large_mod.shuffle(&mut irng);
    weak_mod.shuffle(&mut irng);
    strong_mod.shuffle(&mut irng);
    gun_type.shuffle(&mut irng);
    gun_method.shuffle(&mut irng);
    let mut prng = rand::thread_rng();

    for i in min_range..macro_count {
        wep_vec.push(unwrapped_weapon.clone());
        bul_vec.push(unwrapped_bullet.clone());
        ware_vec.push(unwrapped_ware.clone());

        // INVARIANT!! = all lists must be the same size as they are indexed across time and space to be with their friends
        match profile_vec[i].unwrap() {
            &"small_gun" => {
                class_vec.push("weapons".to_string());
                weaponorturret_vec.push("weapon".to_string());
                laserbool_vec.push(0);
                if laserbool_vec[i] == 0 {
                    compname_vec.push(bullet_s_art[i].to_string())
                } else {
                    compname_vec.push(bullet_s_beam_art[i].to_string())
                }

                wep_comp_vec.push(
                    weapon_s_art
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .to_string(),
                );
                weaponrange_vec.push(prng.gen_range(2, 10));
                iconname_vec.push(wep_icons[i].to_string());
                rateoffire_vec.push(prng.gen_range(0.5, 5.0));
                mindamage_vec.push(prng.gen_range(10, 50));
                maxdamage_vec.push(prng.gen_range(50, 100));
                flatdamage_vec.push(prng.gen_range(maxdamage_vec[i], 100));
                speed_vec.push(prng.gen_range(500, 2000));
                weaponrange_vec.push(prng.gen_range(2, 9));
                impactname_vec.push(impact_vec[i].to_string());
                launchname_vec.push(launch_vec[i].to_string());
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
            &"medium_gun" => {
                weaponorturret_vec.push(class.choose(&mut rand::thread_rng()).unwrap().to_string());
                laserbool_vec.push(0);
                if laserbool_vec[i] == 0 {
                    compname_vec.push(bullet_s_art[i].to_string())
                } else {
                    compname_vec.push(bullet_s_beam_art[i].to_string())
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
                iconname_vec.push(wep_icons[i].to_string());
                rateoffire_vec.push(prng.gen_range(0.5, 5.0));
                mindamage_vec.push(prng.gen_range(10, 50));
                maxdamage_vec.push(prng.gen_range(50, 100));
                flatdamage_vec.push(prng.gen_range(maxdamage_vec[i], 100));
                speed_vec.push(prng.gen_range(500, 2000));
                weaponrange_vec.push(prng.gen_range(2, 9));
                impactname_vec.push(impact_vec[i].to_string());
                launchname_vec.push(launch_vec[i].to_string());
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
            &"large_gun" => {
                weaponorturret_vec.push(class.choose(&mut rand::thread_rng()).unwrap().to_string());
                laserbool_vec.push(0);
                if laserbool_vec[i] == 0 {
                    compname_vec.push(bullet_s_art[i].to_string())
                } else {
                    compname_vec.push(bullet_s_beam_art[i].to_string())
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
                iconname_vec.push(wep_icons[i].to_string());
                rateoffire_vec.push(prng.gen_range(0.5, 5.0));
                mindamage_vec.push(prng.gen_range(10, 50));
                maxdamage_vec.push(prng.gen_range(50, 100));
                flatdamage_vec.push(prng.gen_range(maxdamage_vec[i], 100));
                speed_vec.push(prng.gen_range(500, 2000));
                weaponrange_vec.push(prng.gen_range(2, 9));
                impactname_vec.push(impact_vec[i].to_string());
                launchname_vec.push(launch_vec[i].to_string());
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
            &"xl_gun" => {
                weaponorturret_vec.push(class.choose(&mut rand::thread_rng()).unwrap().to_string());
                laserbool_vec.push(0);
                if laserbool_vec[i] == 0 {
                    compname_vec.push(bullet_s_art[i].to_string())
                } else {
                    compname_vec.push(bullet_s_beam_art[i].to_string())
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
                iconname_vec.push(wep_icons[i].to_string());
                rateoffire_vec.push(prng.gen_range(10.0, 50.0));
                mindamage_vec.push(prng.gen_range(1000, 5000));
                maxdamage_vec.push(prng.gen_range(5000, 50000));
                flatdamage_vec.push(prng.gen_range(maxdamage_vec[i], 50000));
                speed_vec.push(prng.gen_range(500, 2000));
                weaponrange_vec.push(prng.gen_range(2, 9));
                impactname_vec.push(impact_vec[i].to_string());
                launchname_vec.push(launch_vec[i].to_string());
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

fn replace_string(template: &mut String, word: &str, txt: &str) -> () {
    let word_start = template.find(word).unwrap_or(template.len());
    let word_end = word_start + word.len();

    template.replace_range(word_start..word_end, &txt.to_string());
}

fn choose_random(choosefrom: &mut Vec<&str>) -> String {
    let returnthing = choosefrom
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    return returnthing;
}
