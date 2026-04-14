use std::fs; use std::path::Path;

use crate::world::{Room, Npc, Item};

// 区域结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rooms: Vec<Room>,
    pub npcs: Vec<Npc>,
    pub items: Vec<Item>,
}

impl Region {
    // 从目录加载区域
    pub fn load_from_directory(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 加载区域配置
        let config_path = Path::new(path).join("region.json");
        let content = fs::read_to_string(config_path)?;
        let mut region: Region = serde_json::from_str(&content)?;

        // 加载房间
        let rooms_path = Path::new(path).join("rooms");
        if rooms_path.exists() {
            region.rooms.clear();
            for entry in fs::read_dir(rooms_path)? {
                let entry = entry?;
                let room_path = entry.path();
                if room_path.is_dir() {
                    if let Ok(room) = Room::load_from_directory(room_path.to_str().unwrap()) {
                        region.rooms.push(room);
                    }
                }
            }
        }

        // 加载区域NPC
        let npcs_path = Path::new(path).join("npcs.json");
        if npcs_path.exists() {
            let content = fs::read_to_string(npcs_path)?;
            region.npcs = serde_json::from_str(&content)?;
        }

        // 加载区域物品
        let items_path = Path::new(path).join("items.json");
        if items_path.exists() {
            let content = fs::read_to_string(items_path)?;
            region.items = serde_json::from_str(&content)?;
        }

        Ok(region)
    }

    // 保存区域到目录
    pub fn save_to_directory(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 创建目录结构
        fs::create_dir_all(Path::new(path).join("rooms"))?;

        // 保存区域配置
        let config_path = Path::new(path).join("region.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;

        // 保存房间
        for room in &self.rooms {
            let room_path = Path::new(path).join("rooms").join(&room.id);
            room.save_to_directory(room_path.to_str().unwrap())?;
        }

        // 保存区域NPC
        let npcs_path = Path::new(path).join("npcs.json");
        let npcs_content = serde_json::to_string_pretty(&self.npcs)?;
        fs::write(npcs_path, npcs_content)?;

        // 保存区域物品
        let items_path = Path::new(path).join("items.json");
        let items_content = serde_json::to_string_pretty(&self.items)?;
        fs::write(items_path, items_content)?;

        Ok(())
    }

    // 查找房间
    pub fn find_room(&self, room_id: &str) -> Option<&Room> {
        self.rooms.iter().find(|room| room.id == room_id)
    }

    // 查找NPC
    pub fn find_npc(&self, npc_id: &str) -> Option<&Npc> {
        // 先在区域NPC中查找
        if let Some(npc) = self.npcs.iter().find(|n| n.id == npc_id) {
            return Some(npc);
        }
        // 再在各个房间中查找
        for room in &self.rooms {
            if let Some(npc) = room.find_npc(npc_id) {
                return Some(npc);
            }
        }
        None
    }

    // 查找物品
    pub fn find_item(&self, item_id: &str) -> Option<&Item> {
        // 先在区域物品中查找
        if let Some(item) = self.items.iter().find(|i| i.id == item_id) {
            return Some(item);
        }
        // 再在各个房间中查找
        for room in &self.rooms {
            if let Some(item) = room.find_item(item_id) {
                return Some(item);
            }
        }
        None
    }
}
