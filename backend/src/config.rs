use serde::{Serialize, Deserialize};
use crate::world::GameWorld;

// 加载游戏世界
pub fn load_world() -> GameWorld {
    use std::fs;
    
    // 尝试从world目录加载
    if fs::metadata("world").is_ok() {
        match GameWorld::load_from_directory("world") {
            Ok(world) => {
                println!("Loaded game world from directory");
                world
            },
            Err(e) => {
                println!("Failed to load world from directory: {}, using default world", e);
                default_world()
            }
        }
    } else {
        // 如果world目录不存在，使用默认世界
        println!("World directory not found, using default world");
        default_world()
    }
}

// 默认游戏世界
pub fn default_world() -> GameWorld {
    use crate::world::{Region, Room, Npc, Item, Task};
    use crate::world::room::Exit;
    
    // 创建九州区域
    let jiuzhou_region = Region {
        id: "jiuzhou".to_string(),
        name: "九州".to_string(),
        description: "一个充满修仙元素的区域，包含村庄、山林、农田等多种地形。".to_string(),
        rooms: vec![
            Room {
                id: "room1".to_string(),
                name: "九州村广场".to_string(),
                description: "一个繁华的村庄广场，中央有一口水井。周围有几座建筑物，包括客栈、药店和武器店。".to_string(),
                exits: vec![
                    Exit {
                        direction: "north".to_string(),
                        target_room: "room2".to_string(),
                        description: Some("向北通往山林".to_string()),
                    },
                    Exit {
                        direction: "south".to_string(),
                        target_room: "room3".to_string(),
                        description: Some("向南通往农田".to_string()),
                    },
                    Exit {
                        direction: "east".to_string(),
                        target_room: "room4".to_string(),
                        description: Some("向东通往市场".to_string()),
                    },
                    Exit {
                        direction: "west".to_string(),
                        target_room: "room5".to_string(),
                        description: Some("向西通往客栈".to_string()),
                    },
                ],
                npcs: vec![
                    Npc {
                        id: "npc1".to_string(),
                        name: "客栈老板".to_string(),
                        description: "一个友好的客栈老板，站在柜台后面。他经营着这个村庄唯一的客栈，提供住宿和餐饮服务。".to_string(),
                        level: 1,
                        attributes: Npc::default_attributes(1),
                        character_type: "NPC".to_string(),
                        race: "人类".to_string(),
                        ai_type: "merchant".to_string(),
                        prompt_system: Npc::default_prompt_system(vec!["客栈老板", "友好", "经商", "提供住宿"].iter().map(|s| s.to_string()).collect()),
                        behaviors: vec!["欢迎顾客", "提供信息", "出售物品"].iter().map(|s| s.to_string()).collect(),
                        dialogue_history: Npc::default_dialogue_history(),
                        location: "room1".to_string(),
                        schedule: None,
                        relationships: vec![],
                        inventory: vec![],
                        shop_items: None,
                    },
                    Npc {
                        id: "npc2".to_string(),
                        name: "药店老板".to_string(),
                        description: "一个年长的药店老板，精通草药知识。他可以为你提供各种疗伤和修炼用的药物。".to_string(),
                        level: 2,
                        attributes: Npc::default_attributes(2),
                        character_type: "NPC".to_string(),
                        race: "人类".to_string(),
                        ai_type: "herbalist".to_string(),
                        prompt_system: Npc::default_prompt_system(vec!["药店老板", "草药专家", "知识渊博", "治疗"].iter().map(|s| s.to_string()).collect()),
                        behaviors: vec!["识别草药", "配制药物", "提供医疗建议"].iter().map(|s| s.to_string()).collect(),
                        dialogue_history: Npc::default_dialogue_history(),
                        location: "room1".to_string(),
                        schedule: None,
                        relationships: vec![],
                        inventory: vec![],
                        shop_items: None,
                    },
                ],
                items: vec![
                    Item {
                        id: "item1".to_string(),
                        name: "疗伤丹".to_string(),
                        description: "一颗红色的丹药，服用后可以恢复生命值。".to_string(),
                        value: 10,
                        item_type: "CultivationItem".to_string(),
                        level: 1,
                        attributes: Some(Item::default_attributes()),
                        prompt_system: Item::default_prompt_system(vec!["疗伤丹", "红色", "恢复生命", "丹药"].iter().map(|s| s.to_string()).collect()),
                        is_growable: false,
                        growth_stage: 0,
                        max_growth_stage: 1,
                        effects: vec![],
                        durability: None,
                        max_durability: None,
                        is_equippable: false,
                        equipment_slot: None,
                        crafting_recipe: None,
                        required_level: 0,
                        required_cultivation: None,
                        rarity: "Common".to_string(),
                        lore: None,
                    },
                    Item {
                        id: "item2".to_string(),
                        name: "灵力草".to_string(),
                        description: "一种蕴含灵力的草药，可以用来修炼或制作丹药。".to_string(),
                        value: 5,
                        item_type: "Herb".to_string(),
                        level: 1,
                        attributes: None,
                        prompt_system: Item::default_prompt_system(vec!["灵力草", "绿色", "蕴含灵力", "草药"].iter().map(|s| s.to_string()).collect()),
                        is_growable: true,
                        growth_stage: 1,
                        max_growth_stage: 3,
                        effects: vec![],
                        durability: None,
                        max_durability: None,
                        is_equippable: false,
                        equipment_slot: None,
                        crafting_recipe: None,
                        required_level: 0,
                        required_cultivation: None,
                        rarity: "Common".to_string(),
                        lore: None,
                    },
                ],
                region_id: "jiuzhou".to_string(),
                level: 1,
                is_safe: true,
                environment: "村庄".to_string(),
            },
            Room {
                id: "room2".to_string(),
                name: "北部山林".to_string(),
                description: "一条狭窄的小路，向北延伸进入茂密的山林。这里经常有妖兽出没。".to_string(),
                exits: vec![
                    Exit {
                        direction: "south".to_string(),
                        target_room: "room1".to_string(),
                        description: Some("向南通往村庄广场".to_string()),
                    },
                    Exit {
                        direction: "north".to_string(),
                        target_room: "room6".to_string(),
                        description: Some("向北通往山顶".to_string()),
                    },
                ],
                npcs: vec![
                    Npc {
                        id: "npc3".to_string(),
                        name: "山林妖兽".to_string(),
                        description: "一只盘踞在山林中的妖兽，看起来很凶猛。".to_string(),
                        level: 3,
                        attributes: Npc::default_attributes(3),
                        character_type: "Enemy".to_string(),
                        race: "妖兽".to_string(),
                        ai_type: "monster".to_string(),
                        prompt_system: Npc::default_prompt_system(vec!["山林妖兽", "凶猛", "攻击性强", "妖兽"].iter().map(|s| s.to_string()).collect()),
                        behaviors: vec!["攻击入侵者", "守卫领地", "寻找食物"].iter().map(|s| s.to_string()).collect(),
                        dialogue_history: Npc::default_dialogue_history(),
                        location: "room2".to_string(),
                        schedule: None,
                        relationships: vec![],
                        inventory: vec![],
                        shop_items: None,
                    },
                ],
                items: vec![
                    Item {
                        id: "item3".to_string(),
                        name: "妖兽内丹".to_string(),
                        description: "从妖兽体内取出的内丹，蕴含强大的灵力。".to_string(),
                        value: 20,
                        item_type: "CultivationItem".to_string(),
                        level: 3,
                        attributes: Some(Item::default_attributes()),
                        prompt_system: Item::default_prompt_system(vec!["妖兽内丹", "红色", "蕴含灵力", "稀有"].iter().map(|s| s.to_string()).collect()),
                        is_growable: false,
                        growth_stage: 0,
                        max_growth_stage: 1,
                        effects: vec![],
                        durability: None,
                        max_durability: None,
                        is_equippable: false,
                        equipment_slot: None,
                        crafting_recipe: None,
                        required_level: 3,
                        required_cultivation: None,
                        rarity: "Rare".to_string(),
                        lore: None,
                    },
                ],
                region_id: "jiuzhou".to_string(),
                level: 3,
                is_safe: false,
                environment: "山林".to_string(),
            },
        ],
        npcs: vec![],
        items: vec![],
    };
    
    // 创建默认游戏世界
    GameWorld {
        regions: vec![jiuzhou_region],
        global_items: vec![],
        global_npcs: vec![],
        global_tasks: vec![],
    }
}

// 保存游戏世界
pub fn save_world(world: &GameWorld) {
    match world.save_to_directory("world") {
        Ok(_) => println!("Game world saved successfully"),
        Err(e) => println!("Failed to save game world: {}", e),
    }
}
