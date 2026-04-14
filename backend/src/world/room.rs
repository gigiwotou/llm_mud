use std::fs; use std::path::Path;

use crate::world::{Npc, Item};

// 出口结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Exit {
    pub direction: String,
    pub target_room: String,
    pub description: Option<String>,
}

// 房间结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exits: Vec<Exit>,
    pub npcs: Vec<Npc>,
    pub items: Vec<Item>,
    pub region_id: String,
    pub level: u32,
    pub is_safe: bool,
    pub environment: String,
}

impl Room {
    // 从目录加载房间
    pub fn load_from_directory(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 加载房间配置
        let config_path = Path::new(path).join("room.json");
        let content = fs::read_to_string(config_path)?;
        let mut room: Room = serde_json::from_str(&content)?;

        // 加载房间NPC
        let npcs_path = Path::new(path).join("npcs.json");
        if npcs_path.exists() {
            let content = fs::read_to_string(npcs_path)?;
            room.npcs = serde_json::from_str(&content)?;
        }

        // 加载房间物品
        let items_path = Path::new(path).join("items.json");
        if items_path.exists() {
            let content = fs::read_to_string(items_path)?;
            room.items = serde_json::from_str(&content)?;
        }

        Ok(room)
    }

    // 保存房间到目录
    pub fn save_to_directory(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 创建目录
        fs::create_dir_all(path)?;

        // 保存房间配置
        let config_path = Path::new(path).join("room.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;

        // 保存房间NPC
        let npcs_path = Path::new(path).join("npcs.json");
        let npcs_content = serde_json::to_string_pretty(&self.npcs)?;
        fs::write(npcs_path, npcs_content)?;

        // 保存房间物品
        let items_path = Path::new(path).join("items.json");
        let items_content = serde_json::to_string_pretty(&self.items)?;
        fs::write(items_path, items_content)?;

        Ok(())
    }

    // 查找NPC
    pub fn find_npc(&self, npc_id: &str) -> Option<&Npc> {
        self.npcs.iter().find(|npc| npc.id == npc_id)
    }

    // 查找物品
    pub fn find_item(&self, item_id: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.id == item_id)
    }

    // 查找出口
    pub fn find_exit(&self, direction: &str) -> Option<&Exit> {
        self.exits.iter().find(|exit| exit.direction == direction)
    }
}
