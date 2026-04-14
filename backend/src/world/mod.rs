use std::fs; use std::path::Path;

pub mod region;
pub mod room;
pub mod npc;
pub mod item;
pub mod task;

pub use self::region::Region;
pub use self::room::Room;
pub use self::npc::Npc;
pub use self::item::Item;
pub use self::task::Task;

// 游戏世界结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GameWorld {
    pub regions: Vec<Region>,
    pub global_items: Vec<item::Item>,
    pub global_npcs: Vec<npc::Npc>,
    pub global_tasks: Vec<task::Task>,
}

impl GameWorld {
    // 从目录加载游戏世界
    pub fn load_from_directory(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut world = GameWorld {
            regions: Vec::new(),
            global_items: Vec::new(),
            global_npcs: Vec::new(),
            global_tasks: Vec::new(),
        };

        // 加载区域
        let regions_path = Path::new(path).join("regions");
        if regions_path.exists() {
            for entry in fs::read_dir(regions_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if let Ok(region) = Region::load_from_directory(path.to_str().unwrap()) {
                        world.regions.push(region);
                    }
                }
            }
        }

        // 加载全局物品
        let items_path = Path::new(path).join("global_items.json");
        if items_path.exists() {
            let content = fs::read_to_string(items_path)?;
            world.global_items = serde_json::from_str(&content)?;
        }

        // 加载全局NPC
        let npcs_path = Path::new(path).join("global_npcs.json");
        if npcs_path.exists() {
            let content = fs::read_to_string(npcs_path)?;
            world.global_npcs = serde_json::from_str(&content)?;
        }

        // 加载全局任务
        let tasks_path = Path::new(path).join("global_tasks.json");
        if tasks_path.exists() {
            let content = fs::read_to_string(tasks_path)?;
            world.global_tasks = serde_json::from_str(&content)?;
        }

        Ok(world)
    }

    // 保存游戏世界到目录
    pub fn save_to_directory(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 创建目录结构
        fs::create_dir_all(Path::new(path).join("regions"))?;

        // 保存区域
        for region in &self.regions {
            let region_path = Path::new(path).join("regions").join(&region.id);
            region.save_to_directory(region_path.to_str().unwrap())?;
        }

        // 保存全局物品
        let items_path = Path::new(path).join("global_items.json");
        let items_content = serde_json::to_string_pretty(&self.global_items)?;
        fs::write(items_path, items_content)?;

        // 保存全局NPC
        let npcs_path = Path::new(path).join("global_npcs.json");
        let npcs_content = serde_json::to_string_pretty(&self.global_npcs)?;
        fs::write(npcs_path, npcs_content)?;

        // 保存全局任务
        let tasks_path = Path::new(path).join("global_tasks.json");
        let tasks_content = serde_json::to_string_pretty(&self.global_tasks)?;
        fs::write(tasks_path, tasks_content)?;

        Ok(())
    }

    // 查找房间
    pub fn find_room(&self, room_id: &str) -> Option<&room::Room> {
        for region in &self.regions {
            if let Some(room) = region.find_room(room_id) {
                return Some(room);
            }
        }
        None
    }

    // 查找NPC
    pub fn find_npc(&self, npc_id: &str) -> Option<&npc::Npc> {
        // 先在全局NPC中查找
        if let Some(npc) = self.global_npcs.iter().find(|n| n.id == npc_id) {
            return Some(npc);
        }
        // 再在各个区域中查找
        for region in &self.regions {
            if let Some(npc) = region.find_npc(npc_id) {
                return Some(npc);
            }
        }
        None
    }

    // 查找物品
    pub fn find_item(&self, item_id: &str) -> Option<&item::Item> {
        // 先在全局物品中查找
        if let Some(item) = self.global_items.iter().find(|i| i.id == item_id) {
            return Some(item);
        }
        // 再在各个区域中查找
        for region in &self.regions {
            if let Some(item) = region.find_item(item_id) {
                return Some(item);
            }
        }
        None
    }

    // 查找任务
    pub fn find_task(&self, task_id: &str) -> Option<&task::Task> {
        self.global_tasks.iter().find(|t| t.id == task_id)
    }
}
