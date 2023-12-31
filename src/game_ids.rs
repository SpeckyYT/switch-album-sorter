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
                            if let [Value::String(id), Value::String(game_name)] = &array[..] {
                                output.push([id.clone(), game_name.clone()]);
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
            Some(game_name)
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
    "57B4628D2267231D57E0FC1078C0596D"
    "CCFA659F4857F96DDA29AFEDB2E166E6"
    "50E2A11CE4BDDC72EF99DF78315D4938"
    "0C7E8798D2996597BAFD59B427BF6132"
    "1E95E5926F1CB99A87326D927F27B47E"
    "CF99E5A89997DEB91E571D16DB468578"
    "1AB131B6E6571375B79964211BB3F5AE"
    => "Nintendo Switch"

    "D8E7C0AAD8A3BD026A180757B047D272"
    => "3 out of 10"

    "83E7DB95268C5F43C3DEE7171F4FA3A8"
    => "A Short Hike"

    "02CB906EA538A35643C1E1484C4B947D"
    => "Animal Crossing New Horizons"

    "EAD9CDD267F449963C89657382E411AE"
    => "Baba Is You"

    "96794F2558A5B75BC0DBCC69AA0119EE"
    => "Super Bomberman R"

    "FAC96D01C8204092E78AF6ED7F526845"
    => "Bluey The Videogame"

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

    "8D40251E125031858C3BF60FE94F331F"
    => "CoComelon Play with JJ"

    "8D659B81D121384A314B9F29331F4846"
    => "Crawlco Block Knockers"

    "CA5CB375B74CA5E8EF03099883908123"
    => "Cry Babies Magic Tears The Big Game"

    "D30E1C2C7666B65122781E72D3DC0467"
    => "Cuphead"

    "B2AA81D02C39303EBD04D8D7F246F34A"
    => "Deltarune"

    "220C7642926DEA0DE14849A462C4A8EB"
    => "Doki Doki Literature Club Plus"

    "856C45B9690EEBA345D65FB4CE2A29EB"
    => "Fez"

    "3AF4E7A11E56CB938D8EBAB910B97022"
    => "Furry Hentai Tangram"

    "3AB3FB37F4D60F3208932271E26CFE62"
    => "Hole io"

    "604C76706F52F5A7C9EB068576404348"
    => "Hotshot Racing"

    "D83899EEBA9305445B9D20AF92C8ECD0"
    => "It Takes Two"

    "C33B823CE199779FD3A7722EDC1C67C8"
    => "Just Shapes & Beats"

    "336DB1DA8BDC3BF38ED8609901964A6B"
    => "Kirby and the Forgotten Land"

    "C74C768DBE8AD2777FE8EE0BB5CCCEAF"
    => "LEGO 2K Drive"

    "143CA80E86A5FF28CB6B35E95F8B427E"
    => "LEGO Brawls"

    "07D9814262B38BD7F7D0D29A25B162B8"
    => "LEGO City Undercover"

    "4D0B8BABC0F98827395A382631074284"
    => "Lil Gator Game"

    "FD3E588C42DA89808EDE57DCCF03D2A6"
    => "L.O.L. Surprise! Roller Dreams Racing"

    "0C015090E6C5E3F06D97FEDE95458758"
    => "Luigi's Mansion 3"
    
    "9600BAE614E6833B1A261F5FB229CDBA"
    => "Mario + Rabbids Kingdom Battle"
    
    "4E3C1F870D1997912EA61E1657E1AE11"
    => "Mario & Sonic at the Olympic Games Tokyo 2020"

    "16851BE00BC6068871FE49D98876D6C5"
    => "Mario Kart 8 Deluxe"

    "E9FC2E92476F589F890674FE873B0D05"
    => "Mario Party Superstars"
    
    "E99362CFFB666A5E0A01AB378C44C507"
    => "Moving Out"

    "8D65880476C8A1746DABBEAB4860F790"
    => "Moving Out 2"

    "A7CD5182B5FB5D3EDC78DCD8ABC77625"
    => "Muse Dash"

    "4A3C965CA3E534C39F98E517C8AE2522"
    => "Need for Speed Hot Pursuit"

    "837C83632B4DA9326E244736E3868F42"
    => "NEEDY STREAMER OVERLOAD"

    "33B8CC310F76D76B17C6DB0011A75C8A"
    => "New Super Mario Bros U Deluxe"

    "01E6912592E0E7CAAE75026165628A73"
    => "Nickelodeon All-Star Brawl"

    "030CC84A0859ABD745D45D5F8AAD9975"
    => "Nintendo Switch Sports"

    "C46BE5741BFDE75BA1110C84E61676BA"
    => "Nintendo Switch Online - Nintendo 64"

    "CAA427D4F8C8DC9AE87E9002D724F42C"
    => "No Man's Sky"

    "5F006BF2CE7EC39D3B861FA6F150B381"
    => "Octodad - Dadliest Catch"

    "7D6F548625BC0EB94C235FFE679A3299"
    => "Overcooked! 2"

    "EC9541A5F19A0D56BEB69F9C89FBD5CF"
    => "Peppa Pig Avventure Intorno al Mondo"

    "31B7926A16300FCAC90E7674F3916DE7"
    => "Persona 5 Royal"

    "79431C08B38AB91E0A7D597E503B4A08"
    => "PICO PARK"

    "39432935800814FC356D7F44D2FB9357"
    => "PICROSS S MEGA DRIVE & Master System edition"

    "F5596A8FA308CD8548E2185C7EAFC373"
    => "Polyturbo Drift Racing"

    "E0526A8155CFFE505A1FFDF7F2C82AB0"
    => "Puyo Puyo Tetris 2"

    "509C2E182CFA1FEC85638A867964A772"
    => "Rabbids Party of Legends"

    "638E7E1EEC4CD8A239243633C0345A07"
    => "Ring Fit Adventure"

    "6F4D679ED7D2A016B654B265B956C5F0"
    => "Rocket League"

    "D869332CBCFA233B05144B42B5CCC54B"
    => "Rubberduck Wave Racer"

    "94ADEF489DD19F1CD57AE194CF403C6C"
    => "Scribblenauts Mega Pack"

    "66C396DFF5BBB36B52AEA44F72B55406"
    => "Snipperclips"

    "85F8C21DFB6C68234E4132C1BEC3368D"
    => "Sonic Frontiers"

    "723B572B8930A7D95DCAB6E150B83EC1"
    => "SpongeBob Krusty Cook-Off"

    "9465A26C8319F79D230D2D555FA4E03E"
    => "SpongeBob SquarePants Battle for Bikini Bottom - Rehydrated"

    "F07AFCBE7985190919700F5A85BBBE99"
    => "SpongeBob SquarePants The Cosmic Shake"

    "70D37874B56AA7B458E05479CF0E8FDB"
    => "Spyro Reignited Trilogy"

    "20295BD25291E8A82F5A998935ADA6D8"
    => "Suika Game"

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

    "DD36EAD444102ED69B0AF6882696E2E7"
    => "Super Mario RPG"

    "E5B04B40CFD924420DF6142200E473B4"
    => "Super Mario Wonder"

    "E73E4976CD1992F1CF99AC0053695FD6"
    => "Super Monkey Ball Banana Blitz"

    "465FC248781ABFD962B56E27C73ADA62"
    => "Super Monkey Ball Banana Mania"

    "0048A0DCB698F64F3BC16868A1B3765C"
    => "Superliminal"

    "253A54D569A6E75C22E0CDEEA04653E2"
    => "Taiko no Tatsujin Rhythmic Adventure Pack"

    "0BF556E192EE2CE113FCAA64F3178DFE"
    => "THE LONGING"

    "4B42E62DC6B986005C9F76C8C24F28A8"
    => "Trolls Remix Rescue"

    "BDBA961F9C5E6E39D558A949E04CECEE"
    => "Trombone Champ"

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

    "C62B7194CC61B274390C97CFDB54A760"
    => "Youtubers Life 2"

    "923C2B3481E50EED77B8655436A19AEE"
    => "Will You Snail"

    "47A65CA163C826522A27A853DA152761"
    => "Who Wants To Be A Millionaire"
);
