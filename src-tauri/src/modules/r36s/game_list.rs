use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use windows::core::{h, HSTRING};
use windows::Data::Xml::Dom::{XmlDocument};

const GAME_LIST_FILE: &str = "gamelist.xml";

#[derive(Serialize, Deserialize)]
pub(crate) struct GameListEntry {
    path: Vec<u16>,
    name: Vec<u16>,
    image: Vec<u16>,
}

pub fn get_game_list(dir: &Path) -> Result<Vec<GameListEntry>, Error> {
    let file_path = dir.join(GAME_LIST_FILE);
    if !file_path.is_file() {
        return Ok(Vec::new());
    }

    //Read the file.
    let xml_content = std::fs::read_to_string(file_path);
    if xml_content.is_err() {
        return anyhow::bail!("Failed to read the content of game list file");
    }

    let doc = XmlDocument::new();
    let doc = doc?;
    match doc.LoadXml(&HSTRING::from(xml_content?)) {
        Ok(_) => {}
        Err(e) => {
            let message = "Failed to load the content of game list file: ".to_owned() + &e.message();
            return anyhow::bail!(message);
        }
    }

    let mut game_list = Vec::new();
    let games = doc.GetElementsByTagName(h!("game"))?;
    for i in 0..games.Length()? {
        let game = games.Item(i)?;
        let path = game.SelectSingleNode(h!("path")).map_or(OsString::new(), |n| n.InnerText().unwrap().to_os_string());
        let name = game.SelectSingleNode(h!("name")).map_or(OsString::new(), |n| n.InnerText().unwrap().to_os_string());
        let image = game.SelectSingleNode(h!("image")).map_or(OsString::new(), |n| n.InnerText().unwrap().to_os_string());
        game_list.push(GameListEntry {
            path: path.encode_wide().collect::<Vec<u16>>(),
            name: name.encode_wide().collect::<Vec<u16>>(),
            image: image.encode_wide().collect::<Vec<u16>>()
        });
    }

    Ok(game_list)
}