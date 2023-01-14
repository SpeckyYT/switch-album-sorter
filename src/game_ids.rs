use std::fs::read_to_string;
use cached::proc_macro::cached;
use cached::UnboundCache;
use serde_json::{self, Value};

#[cached]
fn get_game_ids_json_file() -> Vec<[String; 2]> {
    match read_to_string("game_ids.json") {
        Ok(file) => {
            match serde_json::from_str::<Value>(&file) {
                Ok(json) => {
                    match json {
                        Value::Array(array) => {
                            let mut output = vec![];
                            match &array[..] {
                                [Value::String(id), Value::String(game_name)] => {
                                    output.push([id.clone(), game_name.clone()]);
                                }
                                _ => {}
                            }
                            output
                        }
                        Value::Object(object) => {
                            let mut output = vec![];
                            for (id, game_name) in object {
                                if let Value::String(game_name) = game_name {
                                    output.push([id, game_name]);
                                }
                            }
                            output
                        }
                        _ => vec![],
                    }
                }
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    }
}

#[cached(
    type = "UnboundCache<String, Option<String>>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ id.to_string() }"#
)]
pub fn get_game_name(id: &str) -> Option<String> {
    let files = get_game_ids_json_file();

    let output = files.iter()
    .map(|[ id, game_name ]| [ id.as_str(), game_name.as_str() ])
    .chain(GAME_IDS)
    .find_map(|[game_id, game_name]| {
        if id == game_id {
            Some(game_name.clone())
        } else {
            None
        }
    })
    .map(|str| str.to_string());

    output
}

macro_rules! game_ids {
    ($($($game_id:literal)+ => $game_name:literal)*) => {
        pub const GAME_IDS: [[&str; 2]; [$($($game_id,)+)*].len()] = [
            $(
                $(
                    [ $game_id, $game_name ],
                )+
            )*
        ];
    }
}

game_ids!(
    "D8E7C0AAD8A3BD026A180757B047D272"
    => "3 out of 10"

    "83E7DB95268C5F43C3DEE7171F4FA3A8"
    => "A Short Hike"

    "02CB906EA538A35643C1E1484C4B947D"
    => "Animal Crossing New Horizons"

    "96794F2558A5B75BC0DBCC69AA0119EE"
    => "Super Bomberman R"

    "2998809A95ABA90F0C3D05F4CAB1384D"
    => "Boomerang Fu"

    "2B08E10BF731D9ACC704BC8E92D161E7"
    => "Burnout Paradise Remastered"
    
    "0564D94A44647B1CC545932794E57271"
    => "Cake Bash"

    "EAD9CC001D10718B5D012E03CB11DB29"
    => "Car Driving School Simulator"

    "75A32021BE3512D7AA96B2D72F764411"
    => "Celeste"

    "EFD0FBB3F99D2E0C38D3D370E0E8C474"
    => "Chess Ultra"

    "79127164CA4A94F74187421B24A3AA8D"
    => "Clubhouse Games 51 Worldwide Classics"

    "D30E1C2C7666B65122781E72D3DC0467"
    => "Cuphead"

    "B2AA81D02C39303EBD04D8D7F246F34A"
    => "Deltarune"

    "856C45B9690EEBA345D65FB4CE2A29EB"
    => "Fez"

    "604C76706F52F5A7C9EB068576404348"
    => "Hotshot Racing"

    "336DB1DA8BDC3BF38ED8609901964A6B"
    => "Kirby and the Forgotten Land"

    "0C015090E6C5E3F06D97FEDE95458758"
    => "Luigi's Mansion 3"
    
    "9600BAE614E6833B1A261F5FB229CDBA"
    => "Mario + Rabbids Kingdom Battle"
    
    "4E3C1F870D1997912EA61E1657E1AE11"
    => "Mario & Sonic at the Olympic Games Tokyo 2020"

    "16851BE00BC6068871FE49D98876D6C5"
    => "Mario Kart 8 Deluxe"
    
    "E99362CFFB666A5E0A01AB378C44C507"
    => "Moving Out"

    "4A3C965CA3E534C39F98E517C8AE2522"
    => "Need for Speed Hot Pursuit"

    "33B8CC310F76D76B17C6DB0011A75C8A"
    => "New Super Mario Bros U Deluxe"

    "5F006BF2CE7EC39D3B861FA6F150B381"
    => "Octodad - Dadliest Catch"

    "7D6F548625BC0EB94C235FFE679A3299"
    => "Overcooked! 2"

    "39432935800814FC356D7F44D2FB9357"
    => "PICROSS S MEGA DRIVE & Master System edition"

    "E0526A8155CFFE505A1FFDF7F2C82AB0"
    => "Puyo Puyo Tetris 2"

    "638E7E1EEC4CD8A239243633C0345A07"
    => "Ring Fit Adventure"

    "94ADEF489DD19F1CD57AE194CF403C6C"
    => "Scribblenauts Mega Pack"

    "9465A26C8319F79D230D2D555FA4E03E"
    => "SpongeBob SquarePants Battle for Bikini Bottom - Rehydrated"

    "70D37874B56AA7B458E05479CF0E8FDB"
    => "Spyro Reignited Trilogy"

    "45998392CB216ABF034EB6F98E399A06"
    => "Super Mario 3D All-Stars"

    "4E551BEEBAD303591E38565E64373519"
    => "Super Mario 3D World + Bowser's Fury"

    "2796B9E2CD7B9B87D175AEAAC2B2E588"
    => "Super Mario 64"

    "8AEDFF741E2D23FBED39474178692DAF"
    => "Super Mario Odyssey"

    "099ECEEF904DB62AEE3A76A3137C241B"
    => "Super Mario Party"

    "E73E4976CD1992F1CF99AC0053695FD6"
    => "Super Monkey Ball Banana Blitz"

    "465FC248781ABFD962B56E27C73ADA62"
    => "Super Monkey Ball Banana Mania"

    "0048A0DCB698F64F3BC16868A1B3765C"
    => "Superliminal"

    "253A54D569A6E75C22E0CDEEA04653E2"
    => "Taiko no Tatsujin Rhythmic Adventure Pack"

    "B732C90228064529ACEBB400746D9309"
    => "Truck Driver"

    "49F3AB1FEC27E3CC1D94E9D4AD7DA138"
    => "Ultimate Chicken Horse"

    "A29C3926C7AC223A657F1415D1C1D797"
    => "Undertale"

    "A10B9E983E22D9175CE28F2275CEC8E6"
    => "Untitled Goose Game"

    "993AC1F3383F4FF879FEA9A85677F9F9"
    => "VVVVVV"

    "923C2B3481E50EED77B8655436A19AEE"
    => "Will You Snail"
);
