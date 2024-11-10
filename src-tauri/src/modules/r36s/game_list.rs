use std::path::{Path};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use windows::core::{h, HSTRING};
use windows::Data::Xml::Dom::{XmlDocument};

const GAME_LIST_FILE: &str = "gamelist.xml";

#[derive(Serialize, Deserialize)]
pub(crate) struct GameListEntry {
    path: String,
    name: String,
    image: String
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
        Ok(_) => {},
        Err(e) => {
            let message = "Failed to load the content of game list file: ".to_owned() + &e.message();
            return anyhow::bail!(message);
        }
    }

    let mut game_list = Vec::new();
    let games = doc.GetElementsByTagName(h!("game"))?;
    for i in 0..games.Length()? {
        let game = games.Item(i)?;
        let path = game.SelectSingleNode(h!("path"))?.InnerText()?.to_string();
        let name = game.SelectSingleNode(h!("name"))?.InnerText()?.to_string();
        let image = game.SelectSingleNode(h!("image"))?.InnerText()?.to_string();
        game_list.push(GameListEntry { path, name, image });
    }

    Ok(game_list)
}