// https://stackoverflow.com/a/50334049
extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::borrow::Borrow;
use std::collections::HashMap;
// requires the rand crate
use std::fs::File;
use std::io::Result;
// use std::io::Read;
use std::io::Write;

use rand::distributions::uniform::SampleUniform;
use rand::Rng;
// use std::io::prelude::*;
use rand::seq::SliceRandom;
use serde::Deserialize;
use strum::AsStaticRef;

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

#[derive(Clone, AsRefStr)]
enum WeaponCategory {
    Weapon,
    Turret,
}

impl Default for WeaponCategory {
    fn default() -> Self { WeaponCategory::Weapon }
}

#[derive(AsStaticStr, Deserialize, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Deserialize, Clone)]
struct Limit<T: Clone + SampleUniform> {
    min: T,
    max: T,
}

impl<T: Clone + SampleUniform> Limit<T> {
    fn gen(&self) -> T {
        let mut prng = rand::thread_rng();
        prng.gen_range(self.min.clone(), self.max.clone())
    }
}

#[derive(Deserialize, Clone)]
struct SizeData {
    // Limits
    weapon_range: Limit<i64>,
    rate_of_fire: Limit<f64>,
    // Note: Max of min must be less than min of max
    damage_min: Limit<i64>,
    damage_max: Limit<i64>,
    damage_flat_max: i64,
    speed: Limit<i64>,
    rot_max: Limit<i64>,
    rot_accel: Limit<i64>,
    reload_time: Limit<i64>,
    hull_value: Limit<i64>,

    // Other parameters
    modifiers: Vec<String>,
    weapon_art: Vec<String>,
    turret_art: Option<Vec<String>>,
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
    gun_type: Vec<String>,
    gun_method: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    page_id: u32,
    target_path: String,
    prodwares: Vec<String>,
    factions: Vec<String>,
    art: Art,
    general_modifiers: Modifiers,
    size: HashMap<String, SizeData>,
}

#[derive(Default)]
struct Weapon {
    weapon_xml: String,
    bullet_xml: String,
    bullet_macroname: String,
    ware_xml: String,
    weapon_component: String,

    icon_name: String,
    laser: bool,
    rate_of_fire: f64,

    damage_min: i64,
    damage_max: i64,
    damage_flat: i64,
    speed: i64,
    weapon_range: i64,

    impact_effect: String,
    muzzle_effect: String,
    weapon_system: String,
    category: WeaponCategory,

    rot_max: i64,
    rot_accel: i64,
    reload_time: i64,
    hull_value: i64,

    price_min: i64,
    price_avg: i64,
    price_max: i64,

    factions_ware: String,

    prodware_one_name: String,
    prodware_one_qty: i64,
    prodware_two_name: String,
    prodware_two_qty: i64,
    prodware_three_name: String,
    prodware_three_qty: i64,
    compname: String,
    macroname: String,
    tname: String,
}

fn replace_string(template: &mut String, word: &str, txt: &str) -> () {
    let word_start = template.find(word).unwrap_or(template.len());
    let word_end = word_start + word.len();

    template.replace_range(word_start..word_end, &txt.to_string());
}

fn choose_random(choosefrom: &Vec<String>) -> String {
    choosefrom
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

fn to_macro_name(name: &String) -> String {
    name.replace(" ", "_").to_lowercase()
}

fn main() -> Result<()> {
    //parsing the "words" as stuff between whitespace
    // Note: Config is currently hard-coded at compile-time
    let toml_str = include_str!("Config.toml");
    let conf: Config = toml::from_str(toml_str.as_ref()).unwrap();
    let art = &conf.art;
    let gen_mod = &conf.general_modifiers;
    // tfiles
    let page_id = conf.page_id;
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
    let target_path = conf.target_path.clone();
    // Note: currently hard-coded at compile-time
    let path_tfile = target_path.clone() + "tfiles.xml";

    let mut ware_file_string = " ".to_string();

    let weapon_template = include_str!("../templates/xml_read_weapon.xml");
    let bullet_template = include_str!("../templates/xml_read_bullet.xml");
    let ware_template = include_str!("../templates/xml_read_ware.xml");
    let mut index_template: String = include_str!("../templates/xml_read_index.xml").to_string();
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

    let mut weapon_vec: Vec<Weapon> = vec![];// TODO remove when possible
    // wep
    let class = vec![WeaponCategory::Weapon, WeaponCategory::Turret];
    let sizes = vec![Size::Small, Size::Medium, Size::Large, Size::ExtraLarge, ];
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
    let empty_str: Vec<String> = vec![];
    for _ in min_range..macro_count {
        let size = sizes.choose(&mut prng).unwrap();
        let size_str = size.as_static().to_string();
        let size_conf: &SizeData = conf.size[&size_str].borrow();
        let weapon_art = &size_conf.weapon_art;
        let turret_art = size_conf.turret_art.as_ref().unwrap_or(&empty_str);

        let damage_min = size_conf.damage_max.gen();
        let damage_max = size_conf.damage_max.gen();
        // TODO is this intended behavior of min?
        let damage_flat = prng.gen_range(damage_max, size_conf.damage_flat_max);
        let rate_of_fire = size_conf.rate_of_fire.gen();
        let dps = ((damage_min + damage_max) as f64 / 2f64 + damage_flat as f64) * rate_of_fire;
        print!(
            "
                {}: {:?}",
            size.as_static(),
            dps
        );
        let name = choose_random(if dps >= 300f64 {
            &gen_mod.strong
        } else {
            &gen_mod.weak
        }) + &choose_random(&size_conf.modifiers)
            + &choose_random(&gen_mod.gun_type)
            + &choose_random(&gen_mod.gun_method);
        let laser = false;
        let category = if turret_art.is_empty() {
            WeaponCategory::Weapon
        } else {
            class.choose(&mut prng).unwrap().clone()
        };
        let wep = Weapon {
            laser,
            category: category.clone(),
            compname: if !laser {
                choose_random(&size_conf.projectiles)
            } else {
                choose_random(&size_conf.beams)
            },
            weapon_component: match &category {
                WeaponCategory::Weapon => weapon_art,
                WeaponCategory::Turret => turret_art
            }.choose(&mut prng).unwrap().to_string(),
            tname: name.clone(),
            bullet_macroname: to_macro_name((name.clone() + "bullet_macro").borrow()),
            macroname: to_macro_name((name.clone() + "macro").borrow()),
            weapon_xml: weapon_template.to_string(),
            bullet_xml: bullet_template.to_string(),
            ware_xml: ware_template.to_string(),
            icon_name: choose_random(&art.icons).to_string(),
            price_min: prng.gen_range((dps * 100f64) as i64, (dps * 200f64) as i64),
            price_avg: prng.gen_range((dps * 200f64) as i64, (dps * 300f64) as i64),
            price_max: prng.gen_range((dps * 300f64) as i64, (dps * 400f64) as i64),
            damage_min,
            damage_max,
            damage_flat,
            rate_of_fire,
            weapon_range: size_conf.weapon_range.gen(),
            reload_time: size_conf.reload_time.gen(),

            rot_max: size_conf.rot_max.gen(),
            rot_accel: size_conf.rot_accel.gen(),
            hull_value: size_conf.hull_value.gen(),
            speed: size_conf.speed.gen(),

            impact_effect: choose_random(&art.effects.impact).to_string(),
            muzzle_effect: choose_random(&art.effects.muzzle).to_string(),

            weapon_system: "weapon_standard".to_string(),
            prodware_one_name: choose_random(&conf.prodwares),
            prodware_one_qty: prng.gen_range(10, 240),
            prodware_two_name: choose_random(&conf.prodwares),
            prodware_two_qty: prng.gen_range(10, 240),
            prodware_three_name: choose_random(&conf.prodwares),
            prodware_three_qty: prng.gen_range(10, 240),
            factions_ware: choose_random(&conf.factions),
        };
        weapon_vec.push(wep);
    }

    for i in min_range..macro_count {
        wep_name_id += 100;
        wep_base_id += 100;
        wep_short_id += 100;
        wep_desc_id += 100;
        let wep = &mut weapon_vec[i];
        let weapon_xml = &mut wep.weapon_xml;
        for word in weapon_xml.clone().split_whitespace() {
            match word {
                "macroname" => replace_string(
                    weapon_xml,
                    word,
                    &wep.macroname,
                ),
                "bulletmacroname" => replace_string(
                    weapon_xml,
                    word,
                    &wep.bullet_macroname,
                ),
                "compname" => {
                    replace_string(weapon_xml, word, &wep.weapon_component)
                }
                "pageident" => replace_string(weapon_xml, word, &page_id.to_string()),
                "nameid" => {
                    wep_name_id += 1;
                    replace_string(weapon_xml, word, &wep_name_id.to_string());
                    t_string.push_str(&format!(
                        "
                    <t id={:?}>{}</t>",
                        &wep_name_id.to_string(),
                        wep.tname
                    ))
                }
                "baseid" => {
                    wep_base_id += 1;
                    replace_string(weapon_xml, word, &wep_base_id.to_string());
                    t_string.push_str(&format!(
                        "
                    <t id={:?}>{}</t>",
                        &wep_base_id.to_string(),
                        wep.tname
                    ))
                }
                "shortid" => {
                    wep_short_id += 1;
                    replace_string(weapon_xml, word, &wep_short_id.to_string());
                    t_string.push_str(&format!(
                        "
                    <t id={:?}>{}</t>",
                        &wep_short_id.to_string(),
                        wep.tname
                    ))
                }
                "descid" => {
                    wep_desc_id += 1;
                    replace_string(weapon_xml, word, &wep_desc_id.to_string());
                    t_string.push_str(&format!(
                        "
                    <t id={:?}>{}</t>",
                        &wep_desc_id.to_string(),
                        wep.tname
                    ))
                }
                "rotmax" => replace_string(weapon_xml, word, &wep.rot_max.to_string()),
                "rotacc" => replace_string(weapon_xml, word, &wep.rot_accel.to_string()),
                "reloadtime" => {
                    replace_string(weapon_xml, word, &wep.reload_time.to_string())
                }
                "hullvalue" => {
                    replace_string(weapon_xml, word, &wep.hull_value.to_string())
                }
                "weaponorturret" => replace_string(
                    weapon_xml,
                    word,
                    &wep.category.as_ref().to_lowercase(),
                ),

                _ => (),
            }
        }

        // overwrite weapon file
        let weapon_result_path = format!("{}{}{}{}{}", target_path, macroname, weapon, i, xml);
        let mut file = File::create(&weapon_result_path)?;
        file.write_all(
            weapon_xml
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

        let ware_xml = &mut wep.ware_xml;
        for word in ware_xml.clone().split_whitespace() {
            match word {
                "macroname" => replace_string(
                    ware_xml,
                    word,
                    &wep.macroname,
                ),
                "pageident" => replace_string(ware_xml, word, &page_id.to_string()),
                "nameid" => {
                    ware_name_id += 1;
                    replace_string(ware_xml, word, &ware_name_id.to_string())
                }
                "baseid" => {
                    ware_base_id += 1;
                    replace_string(ware_xml, word, &ware_base_id.to_string())
                }
                "shortid" => {
                    ware_short_id += 1;
                    replace_string(ware_xml, word, &ware_short_id.to_string())
                }
                "descid" => {
                    ware_desc_id += 1;
                    replace_string(ware_xml, word, &ware_desc_id.to_string())
                }
                "minprice" => replace_string(
                    ware_xml,
                    word,
                    &wep.price_min.to_string(),
                ),
                "avgprice" => replace_string(
                    ware_xml,
                    word,
                    &wep.price_avg.to_string(),
                ),
                "maxprice" => replace_string(
                    ware_xml,
                    word,
                    &wep.price_max.to_string(),
                ),
                "classgroup" => replace_string(
                    ware_xml,
                    word,
                    &(wep.category.as_ref().to_string().to_lowercase() + "s")),
                "weaponorturret" => replace_string(
                    ware_xml,
                    word,
                    &wep.category.as_ref().to_lowercase(),
                ),

                "prodwareone" => replace_string(
                    ware_xml,
                    word,
                    &wep.prodware_one_name.to_string(),
                ),
                "prodwaretwo" => replace_string(
                    ware_xml,
                    word,
                    &wep.prodware_two_name,
                ),
                "prodwarethree" => replace_string(
                    ware_xml,
                    word,
                    &wep.prodware_three_name,
                ),
                "prodwareone_amount" => replace_string(
                    ware_xml,
                    word,
                    &wep.prodware_one_qty.to_string(),
                ),
                "prodwaretwo_amount" => replace_string(
                    ware_xml,
                    word,
                    &wep.prodware_two_qty.to_string(),
                ),
                "prodwarethree_amount" => replace_string(
                    ware_xml,
                    word,
                    &wep.prodware_three_qty.to_string(),
                ),
                "factions_ware" => replace_string(
                    ware_xml,
                    word,
                    &wep.factions_ware.to_string(),
                ),
                "wareid" => replace_string(
                    ware_xml,
                    word,
                    &wep.macroname,
                ),

                _ => (),
            }
        }
        // append ware string
        ware_file_string.push_str(
            "
        ",
        );
        ware_file_string.push_str(ware_xml);

        // index entries
        let newentry = format!(
            "{}{}{}{}{}",
            new_entry_line,
            &wep.macroname,
            value,
            format!("{}{}{}", macroname, weapon, i),
            closing
        );
        let bulletnewentry = format!(
            "{}{}{}{}{}",
            new_entry_line,
            &wep.bullet_macroname,
            value,
            format!("{}{}{}", macroname, bullet, i),
            closing
        );
        index_template.push_str(&newentry);
        index_template.push_str(&bulletnewentry);

        let bullet_xml = &mut wep.bullet_xml;
        for word in bullet_xml.clone().split_whitespace() {
            match word {
                "bulletmacroname" => replace_string(
                    bullet_xml,
                    word,
                    &wep.bullet_macroname,
                ),
                "iconname" => replace_string(
                    bullet_xml,
                    word,
                    &wep.icon_name,
                ),
                "laserbool" => replace_string(
                    bullet_xml,
                    word,
                    &(wep.laser as i32).to_string(),
                ),
                "rateoffire" => replace_string(
                    bullet_xml,
                    word,
                    &wep.rate_of_fire.to_string(),
                ),
                "mindamage" => replace_string(
                    bullet_xml,
                    word,
                    &wep.damage_min.to_string(),
                ),
                "maxdamage" => replace_string(
                    bullet_xml,
                    word,
                    &wep.damage_max.to_string(),
                ),
                "flatdamage" => replace_string(
                    bullet_xml,
                    word,
                    &wep.damage_flat.to_string(),
                ),
                "speedvalue" => {
                    replace_string(bullet_xml, word, &wep.speed.to_string())
                }
                "lifetimevalue" => replace_string(
                    bullet_xml,
                    word,
                    &wep.weapon_range.to_string(),
                ),
                "impactname" => replace_string(
                    bullet_xml,
                    word,
                    &wep.impact_effect,
                ),
                "launchname" => replace_string(
                    bullet_xml,
                    word,
                    &wep.muzzle_effect,
                ),
                "weaponsystem" => replace_string(
                    bullet_xml,
                    word,
                    &wep.weapon_system,
                ),

                "componentname" => replace_string(bullet_xml, word, &wep.compname),
                _ => (),
            };
        }

        // overwrite bullet file
        let bullet_result_path = format!("{}{}{}{}{}", target_path, macroname, bullet, i, xml);
        let mut file = File::create(&bullet_result_path)?;
        file.write_all(
            bullet_xml
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
    let mut indexfile = File::create(target_path + "index.xml")?;
    indexfile.write_all(index_template.as_bytes())?;
    // tfile
    let mut tfile = File::create(path_tfile)?;
    tfile.write_all(t_string.as_bytes())?;
    Ok(())
}

