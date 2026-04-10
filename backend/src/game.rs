use serde::{Serialize, Deserialize};

// 修炼类型
enum CultivationType {
    QiCultivator, // 气修
    BodyCultivator, // 体修
    SwordCultivator, // 剑修
}

// 角色类型
enum CharacterType {
    NPC,
    Player,
    Enemy,
}

// 种族
enum Race {
    Human,
    Monster,
    Demon,
}

// 物品类型
enum ItemType {
    Normal,
    Herb,
    MagicWeapon,
    MagicTool,
    Weapon,
    Ore,
    CultivationItem,
}

// 属性系统
#[derive(Serialize, Deserialize, Clone)]
pub struct Attributes {
    // 基础属性
    pub strength: u32, // 力量
    pub agility: u32, // 敏捷
    pub intelligence: u32, // 智力
    pub vitality: u32, // 体力
    
    // 修仙属性
    pub qi: u32, // 灵力
    pub spiritual_awareness: u32, // 灵识
    pub physical_strength: u32, // 肉身强度
    pub sword_heart: u32, // 剑心
    
    // 战斗属性
    pub health: u32, // 生命值
    pub mana: u32, // 法力值
    pub defense: u32, // 防御
    pub attack: u32, // 攻击
    pub speed: u32, // 速度
}

// 提示词系统
#[derive(Serialize, Deserialize, Clone)]
pub struct PromptSystem {
    pub base_prompts: Vec<String>, // 基础提示词
    pub personality_prompts: Vec<String>, // 性格提示词
    pub growth_prompts: Vec<String>, // 成长提示词
    pub current_prompts: Vec<String>, // 当前提示词组合
}

// 玩家数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub level: u32,
    pub attributes: Attributes,
    pub cultivation_type: String, // 修炼类型
    pub race: String, // 种族
    pub inventory: Vec<Item>,
    pub location: String,
    pub skills: Vec<Skill>,
    pub techniques: Vec<Technique>, // 功法
    pub equipment: Equipment, // 装备
    pub prompt_system: PromptSystem, // 提示词系统
    pub tasks: Vec<Task>, // 玩家任务列表
    pub currency: u32, // 货币
}

// 物品数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: u32,
    pub item_type: String, // 物品类型
    pub level: u32, // 物品等级
    pub attributes: Option<Attributes>, // 物品属性
    pub prompt_system: PromptSystem, // 提示词系统
    pub is_growable: bool, // 是否可成长
    pub growth_stage: u32, // 成长阶段
}

// 技能数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Skill {
    pub name: String,
    pub level: u32,
    pub description: String,
    pub damage: u32,
    pub mana_cost: u32,
}

// 功法数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Technique {
    pub name: String,
    pub level: u32,
    pub description: String,
    pub cultivation_type: String, // 适用修炼类型
    pub effects: Vec<String>, // 效果
}

// 装备数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Equipment {
    pub weapon: Option<Item>, // 武器
    pub armor: Option<Item>, //  armor
    pub accessories: Vec<Item>, // 饰品
}

// 房间数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exits: Vec<Exit>,
    pub npcs: Vec<Npc>,
    pub items: Vec<Item>,
    pub region: String, // 区域
}

// 出口数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Exit {
    pub direction: String,
    pub target_room: String,
}

// 对话历史
#[derive(Serialize, Deserialize, Clone)]
pub struct DialogueHistory {
    pub messages: Vec<(String, String)>, // (角色, 内容)
    pub max_length: usize,
}

// NPC数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Npc {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: u32,
    pub attributes: Attributes,
    pub character_type: String, // 角色类型
    pub race: String, // 种族
    pub ai_type: String, // AI类型
    pub prompt_system: PromptSystem, // 提示词系统
    pub behaviors: Vec<String>, // 行为模式
    pub dialogue_history: DialogueHistory, // 对话历史
}

// 创建默认对话历史
pub fn default_dialogue_history() -> DialogueHistory {
    DialogueHistory {
        messages: Vec::new(),
        max_length: 10, // 最多保存10条消息
    }
}

// 任务状态
#[derive(Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    Pending, // 待完成
    InProgress, // 进行中
    Completed, // 已完成
    Failed, // 失败
}

// 任务数据
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target: String, // 任务目标
    pub reward: String, // 奖励
    pub status: TaskStatus,
    pub progress: u32, // 任务进度
    pub max_progress: u32, // 最大进度
    pub expiration: Option<u64>, // 过期时间（时间戳）
}

// 战斗结果
#[derive(Serialize, Deserialize, Clone)]
pub struct CombatResult {
    pub success: bool,
    pub message: String,
    pub damage: u32,
    pub attacker_health: u32,
    pub defender_health: u32,
}

// 游戏状态数据
#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub rooms: Vec<Room>,
    pub npcs: Vec<Npc>,
    pub items: Vec<Item>,
    pub tasks: Vec<Task>, // 全局任务列表
}

// 创建默认属性
pub fn default_attributes(level: u32) -> Attributes {
    Attributes {
        strength: 10 + level * 2,
        agility: 10 + level * 2,
        intelligence: 10 + level * 2,
        vitality: 10 + level * 2,
        qi: 5 + level * 3,
        spiritual_awareness: 5 + level * 2,
        physical_strength: 5 + level * 2,
        sword_heart: 5 + level * 2,
        health: 100 + level * 10,
        mana: 50 + level * 5,
        defense: 5 + level * 2,
        attack: 10 + level * 3,
        speed: 5 + level * 2,
    }
}

// 创建默认提示词系统
pub fn default_prompt_system(base_prompts: Vec<String>) -> PromptSystem {
    PromptSystem {
        base_prompts: base_prompts.clone(),
        personality_prompts: vec!["温和", "急躁", "谨慎", "勇敢", "聪明", "忠诚"].iter().map(|s| s.to_string()).collect(),
        growth_prompts: vec!["成长中", "经验丰富", "初学者", "大师"].iter().map(|s| s.to_string()).collect(),
        current_prompts: base_prompts,
    }
}

impl GameState {
    pub fn new() -> Self {
        // 创建初始房间
        let start_room = Room {
            id: "room1".to_string(),
            name: "九州村广场".to_string(),
            description: "一个繁华的村庄广场，中央有一口水井。周围有几座建筑物，包括客栈、药店和武器店。".to_string(),
            exits: vec![
                Exit {
                    direction: "north".to_string(),
                    target_room: "room2".to_string(),
                },
                Exit {
                    direction: "south".to_string(),
                    target_room: "room3".to_string(),
                },
                Exit {
                    direction: "east".to_string(),
                    target_room: "room4".to_string(),
                },
                Exit {
                    direction: "west".to_string(),
                    target_room: "room5".to_string(),
                },
            ],
            npcs: vec![
                Npc {
                    id: "npc1".to_string(),
                    name: "客栈老板".to_string(),
                    description: "一个友好的客栈老板，站在柜台后面。他经营着这个村庄唯一的客栈，提供住宿和餐饮服务。".to_string(),
                    level: 1,
                    attributes: default_attributes(1),
                    character_type: "NPC".to_string(),
                    race: "人类".to_string(),
                    ai_type: "merchant".to_string(),
                    prompt_system: default_prompt_system(vec!["客栈老板", "友好", "经商", "提供住宿"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["欢迎顾客", "提供信息", "出售物品"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
                },
                Npc {
                    id: "npc2".to_string(),
                    name: "药店老板".to_string(),
                    description: "一个年长的药店老板，精通草药知识。他可以为你提供各种疗伤和修炼用的药物。".to_string(),
                    level: 2,
                    attributes: default_attributes(2),
                    character_type: "NPC".to_string(),
                    race: "人类".to_string(),
                    ai_type: "herbalist".to_string(),
                    prompt_system: default_prompt_system(vec!["药店老板", "草药专家", "知识渊博", "治疗"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["识别草药", "配制药物", "提供医疗建议"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
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
                    attributes: Some(Attributes {
                        strength: 0,
                        agility: 0,
                        intelligence: 0,
                        vitality: 0,
                        qi: 0,
                        spiritual_awareness: 0,
                        physical_strength: 0,
                        sword_heart: 0,
                        health: 50,
                        mana: 0,
                        defense: 0,
                        attack: 0,
                        speed: 0,
                    }),
                    prompt_system: default_prompt_system(vec!["疗伤丹", "红色", "恢复生命", "丹药"].iter().map(|s| s.to_string()).collect()),
                    is_growable: false,
                    growth_stage: 0,
                },
                Item {
                    id: "item2".to_string(),
                    name: "灵力草".to_string(),
                    description: "一种蕴含灵力的草药，可以用来修炼或制作丹药。".to_string(),
                    value: 5,
                    item_type: "Herb".to_string(),
                    level: 1,
                    attributes: None,
                    prompt_system: default_prompt_system(vec!["灵力草", "绿色", "蕴含灵力", "草药"].iter().map(|s| s.to_string()).collect()),
                    is_growable: true,
                    growth_stage: 1,
                },
            ],
            region: "九州".to_string(),
        };
        
        // 创建其他房间
        let north_room = Room {
            id: "room2".to_string(),
            name: "北部山林".to_string(),
            description: "一条狭窄的小路，向北延伸进入茂密的山林。这里经常有妖兽出没。".to_string(),
            exits: vec![
                Exit {
                    direction: "south".to_string(),
                    target_room: "room1".to_string(),
                },
                Exit {
                    direction: "north".to_string(),
                    target_room: "room6".to_string(),
                },
            ],
            npcs: vec![
                Npc {
                    id: "npc3".to_string(),
                    name: "山林妖兽".to_string(),
                    description: "一只盘踞在山林中的妖兽，看起来很凶猛。".to_string(),
                    level: 3,
                    attributes: default_attributes(3),
                    character_type: "Enemy".to_string(),
                    race: "妖兽".to_string(),
                    ai_type: "monster".to_string(),
                    prompt_system: default_prompt_system(vec!["山林妖兽", "凶猛", "攻击性强", "妖兽"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["攻击入侵者", "守卫领地", "寻找食物"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
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
                    attributes: Some(Attributes {
                        strength: 0,
                        agility: 0,
                        intelligence: 0,
                        vitality: 0,
                        qi: 20,
                        spiritual_awareness: 0,
                        physical_strength: 0,
                        sword_heart: 0,
                        health: 0,
                        mana: 0,
                        defense: 0,
                        attack: 0,
                        speed: 0,
                    }),
                    prompt_system: default_prompt_system(vec!["妖兽内丹", "红色", "蕴含灵力", "稀有"].iter().map(|s| s.to_string()).collect()),
                    is_growable: false,
                    growth_stage: 0,
                },
            ],
            region: "九州".to_string(),
        };
        
        let south_room = Room {
            id: "room3".to_string(),
            name: "南部农田".to_string(),
            description: "村庄的南部农田，农民们正在辛勤劳作。远处可以看到一片果园。".to_string(),
            exits: vec![
                Exit {
                    direction: "north".to_string(),
                    target_room: "room1".to_string(),
                },
                Exit {
                    direction: "south".to_string(),
                    target_room: "room7".to_string(),
                },
            ],
            npcs: vec![
                Npc {
                    id: "npc4".to_string(),
                    name: "农民".to_string(),
                    description: "一个正在田间劳作的农民，看起来很朴实。".to_string(),
                    level: 1,
                    attributes: default_attributes(1),
                    character_type: "NPC".to_string(),
                    race: "人类".to_string(),
                    ai_type: "villager".to_string(),
                    prompt_system: default_prompt_system(vec!["农民", "朴实", "勤劳", "种田"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["种田", "收获", "出售农产品"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
                },
            ],
            items: vec![
                Item {
                    id: "item4".to_string(),
                    name: "新鲜水果".to_string(),
                    description: "从果园采摘的新鲜水果，可以补充体力。".to_string(),
                    value: 2,
                    item_type: "Normal".to_string(),
                    level: 1,
                    attributes: Some(Attributes {
                        strength: 0,
                        agility: 0,
                        intelligence: 0,
                        vitality: 0,
                        qi: 0,
                        spiritual_awareness: 0,
                        physical_strength: 0,
                        sword_heart: 0,
                        health: 20,
                        mana: 0,
                        defense: 0,
                        attack: 0,
                        speed: 0,
                    }),
                    prompt_system: default_prompt_system(vec!["新鲜水果", "香甜", "补充体力", "食物"].iter().map(|s| s.to_string()).collect()),
                    is_growable: false,
                    growth_stage: 0,
                },
            ],
            region: "九州".to_string(),
        };
        
        let east_room = Room {
            id: "room4".to_string(),
            name: "东部市场".to_string(),
            description: "一个繁忙的市场，商贩们在售卖各种商品，包括武器、防具和修炼资源。".to_string(),
            exits: vec![
                Exit {
                    direction: "west".to_string(),
                    target_room: "room1".to_string(),
                },
                Exit {
                    direction: "east".to_string(),
                    target_room: "room8".to_string(),
                },
            ],
            npcs: vec![
                Npc {
                    id: "npc5".to_string(),
                    name: "武器商".to_string(),
                    description: "一个经验丰富的武器商，出售各种武器和防具。".to_string(),
                    level: 2,
                    attributes: default_attributes(2),
                    character_type: "NPC".to_string(),
                    race: "人类".to_string(),
                    ai_type: "merchant".to_string(),
                    prompt_system: default_prompt_system(vec!["武器商", "经验丰富", "出售武器", " craftsmanship"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["出售武器", "修理装备", "提供武器建议"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
                },
            ],
            items: vec![
                Item {
                    id: "item5".to_string(),
                    name: "铁剑".to_string(),
                    description: "一把普通的铁剑，适合初学者使用。".to_string(),
                    value: 15,
                    item_type: "Weapon".to_string(),
                    level: 1,
                    attributes: Some(Attributes {
                        strength: 0,
                        agility: 0,
                        intelligence: 0,
                        vitality: 0,
                        qi: 0,
                        spiritual_awareness: 0,
                        physical_strength: 0,
                        sword_heart: 0,
                        health: 0,
                        mana: 0,
                        defense: 0,
                        attack: 10,
                        speed: 0,
                    }),
                    prompt_system: default_prompt_system(vec!["铁剑", "普通", "初学者", "武器"].iter().map(|s| s.to_string()).collect()),
                    is_growable: false,
                    growth_stage: 0,
                },
                Item {
                    id: "item6".to_string(),
                    name: "灵铁矿".to_string(),
                    description: "一种蕴含灵力的矿石，可以用来打造法器和法宝。".to_string(),
                    value: 8,
                    item_type: "Ore".to_string(),
                    level: 2,
                    attributes: None,
                    prompt_system: default_prompt_system(vec!["灵铁矿", "银色", "蕴含灵力", "矿石"].iter().map(|s| s.to_string()).collect()),
                    is_growable: false,
                    growth_stage: 0,
                },
            ],
            region: "九州".to_string(),
        };
        
        let west_room = Room {
            id: "room5".to_string(),
            name: "西部客栈".to_string(),
            description: "一个舒适的客栈，有温暖的壁炉和舒适的床铺。这里是过往修士的歇脚之处。".to_string(),
            exits: vec![
                Exit {
                    direction: "east".to_string(),
                    target_room: "room1".to_string(),
                },
                Exit {
                    direction: "west".to_string(),
                    target_room: "room9".to_string(),
                },
            ],
            npcs: vec![
                Npc {
                    id: "npc6".to_string(),
                    name: "修士".to_string(),
                    description: "一个游历的修士，正在客栈中休息。他看起来修为不浅。".to_string(),
                    level: 5,
                    attributes: default_attributes(5),
                    character_type: "NPC".to_string(),
                    race: "人类".to_string(),
                    ai_type: "cultivator".to_string(),
                    prompt_system: default_prompt_system(vec!["修士", "游历", "修为深厚", "知识渊博"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["修炼", "传授知识", "交流修炼心得"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
                },
            ],
            items: vec![
                Item {
                    id: "item7".to_string(),
                    name: "修行心得".to_string(),
                    description: "一本记录修炼心得的书，可以帮助修士提升修为。".to_string(),
                    value: 30,
                    item_type: "CultivationItem".to_string(),
                    level: 5,
                    attributes: Some(Attributes {
                        strength: 0,
                        agility: 0,
                        intelligence: 5,
                        vitality: 0,
                        qi: 10,
                        spiritual_awareness: 5,
                        physical_strength: 0,
                        sword_heart: 0,
                        health: 0,
                        mana: 0,
                        defense: 0,
                        attack: 0,
                        speed: 0,
                    }),
                    prompt_system: default_prompt_system(vec!["修行心得", "古老", "知识渊博", "修炼"].iter().map(|s| s.to_string()).collect()),
                    is_growable: false,
                    growth_stage: 0,
                },
            ],
            region: "九州".to_string(),
        };
        
        // 新增房间
        let mountain_peak = Room {
            id: "room6".to_string(),
            name: "山顶".to_string(),
            description: "山林的山顶，可以俯瞰整个九州村。这里有一座古老的道观。".to_string(),
            exits: vec![
                Exit {
                    direction: "south".to_string(),
                    target_room: "room2".to_string(),
                },
            ],
            npcs: vec![
                Npc {
                    id: "npc7".to_string(),
                    name: "道长".to_string(),
                    description: "山顶道观的道长，修为深厚，精通道法。".to_string(),
                    level: 8,
                    attributes: default_attributes(8),
                    character_type: "NPC".to_string(),
                    race: "人类".to_string(),
                    ai_type: "cultivator".to_string(),
                    prompt_system: default_prompt_system(vec!["道长", "修为深厚", "精通道法", "慈祥"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["修炼", "传授道法", "讲经布道"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
                },
            ],
            items: vec![
                Item {
                    id: "item8".to_string(),
                    name: "道德经".to_string(),
                    description: "一本古老的道德经，蕴含着深奥的道法。".to_string(),
                    value: 50,
                    item_type: "CultivationItem".to_string(),
                    level: 8,
                    attributes: Some(Attributes {
                        strength: 0,
                        agility: 0,
                        intelligence: 10,
                        vitality: 0,
                        qi: 15,
                        spiritual_awareness: 10,
                        physical_strength: 0,
                        sword_heart: 0,
                        health: 0,
                        mana: 0,
                        defense: 0,
                        attack: 0,
                        speed: 0,
                    }),
                    prompt_system: default_prompt_system(vec!["道德经", "古老", "深奥", "道法"].iter().map(|s| s.to_string()).collect()),
                    is_growable: false,
                    growth_stage: 0,
                },
            ],
            region: "九州".to_string(),
        };
        
        let orchard = Room {
            id: "room7".to_string(),
            name: "果园".to_string(),
            description: "一片茂密的果园，种满了各种水果树。空气中弥漫着果香。".to_string(),
            exits: vec![
                Exit {
                    direction: "north".to_string(),
                    target_room: "room3".to_string(),
                },
            ],
            npcs: vec![
                Npc {
                    id: "npc8".to_string(),
                    name: "果农".to_string(),
                    description: "果园的主人，负责管理和采摘水果。".to_string(),
                    level: 2,
                    attributes: default_attributes(2),
                    character_type: "NPC".to_string(),
                    race: "人类".to_string(),
                    ai_type: "villager".to_string(),
                    prompt_system: default_prompt_system(vec!["果农", "勤劳", "善良", "熟悉水果"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["采摘水果", "管理果园", "出售水果"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
                },
            ],
            items: vec![
                Item {
                    id: "item9".to_string(),
                    name: "灵果".to_string(),
                    description: "一种蕴含灵力的水果，食用后可以提升修为。".to_string(),
                    value: 15,
                    item_type: "CultivationItem".to_string(),
                    level: 3,
                    attributes: Some(Attributes {
                        strength: 0,
                        agility: 0,
                        intelligence: 0,
                        vitality: 0,
                        qi: 10,
                        spiritual_awareness: 0,
                        physical_strength: 0,
                        sword_heart: 0,
                        health: 30,
                        mana: 20,
                        defense: 0,
                        attack: 0,
                        speed: 0,
                    }),
                    prompt_system: default_prompt_system(vec!["灵果", "香甜", "蕴含灵力", "稀有"].iter().map(|s| s.to_string()).collect()),
                    is_growable: true,
                    growth_stage: 2,
                },
            ],
            region: "九州".to_string(),
        };
        
        let workshop = Room {
            id: "room8".to_string(),
            name: "工坊".to_string(),
            description: "一个铁匠工坊，里面有各种锻造工具和熔炉。铁匠正在打造武器。".to_string(),
            exits: vec![
                Exit {
                    direction: "west".to_string(),
                    target_room: "room4".to_string(),
                },
            ],
            npcs: vec![
                Npc {
                    id: "npc9".to_string(),
                    name: "铁匠".to_string(),
                    description: "一个经验丰富的铁匠，精通武器锻造。".to_string(),
                    level: 4,
                    attributes: default_attributes(4),
                    character_type: "NPC".to_string(),
                    race: "人类".to_string(),
                    ai_type: "craftsman".to_string(),
                    prompt_system: default_prompt_system(vec!["铁匠", "经验丰富", "精通锻造", "豪爽"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["锻造武器", "修理装备", "传授锻造技巧"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
                },
            ],
            items: vec![
                Item {
                    id: "item10".to_string(),
                    name: "精铁剑".to_string(),
                    description: "一把由精铁打造的剑，比普通铁剑更锋利。".to_string(),
                    value: 30,
                    item_type: "Weapon".to_string(),
                    level: 3,
                    attributes: Some(Attributes {
                        strength: 0,
                        agility: 0,
                        intelligence: 0,
                        vitality: 0,
                        qi: 0,
                        spiritual_awareness: 0,
                        physical_strength: 0,
                        sword_heart: 0,
                        health: 0,
                        mana: 0,
                        defense: 0,
                        attack: 15,
                        speed: 0,
                    }),
                    prompt_system: default_prompt_system(vec!["精铁剑", "锋利", "耐用", "武器"].iter().map(|s| s.to_string()).collect()),
                    is_growable: false,
                    growth_stage: 0,
                },
            ],
            region: "九州".to_string(),
        };
        
        let forest = Room {
            id: "room9".to_string(),
            name: "西部森林".to_string(),
            description: "一片茂密的森林，树木高大，阳光透过树叶洒下斑驳的光影。".to_string(),
            exits: vec![
                Exit {
                    direction: "east".to_string(),
                    target_room: "room5".to_string(),
                },
            ],
            npcs: vec![
                Npc {
                    id: "npc10".to_string(),
                    name: "森林精灵".to_string(),
                    description: "一个居住在森林中的精灵，守护着这片森林。".to_string(),
                    level: 6,
                    attributes: default_attributes(6),
                    character_type: "NPC".to_string(),
                    race: "妖兽".to_string(),
                    ai_type: "guardian".to_string(),
                    prompt_system: default_prompt_system(vec!["森林精灵", "守护森林", "善良", "神秘"].iter().map(|s| s.to_string()).collect()),
                    behaviors: vec!["守护森林", "帮助迷路的人", "与自然交流"].iter().map(|s| s.to_string()).collect(),
                    dialogue_history: default_dialogue_history(),
                },
            ],
            items: vec![
                Item {
                    id: "item11".to_string(),
                    name: "森林之泪".to_string(),
                    description: "森林精灵的眼泪，蕴含着生命的力量。".to_string(),
                    value: 40,
                    item_type: "CultivationItem".to_string(),
                    level: 6,
                    attributes: Some(Attributes {
                        strength: 0,
                        agility: 0,
                        intelligence: 0,
                        vitality: 10,
                        qi: 15,
                        spiritual_awareness: 5,
                        physical_strength: 0,
                        sword_heart: 0,
                        health: 50,
                        mana: 30,
                        defense: 0,
                        attack: 0,
                        speed: 0,
                    }),
                    prompt_system: default_prompt_system(vec!["森林之泪", "神秘", "蕴含生命力量", "稀有"].iter().map(|s| s.to_string()).collect()),
                    is_growable: false,
                    growth_stage: 0,
                },
            ],
            region: "九州".to_string(),
        };
        
        Self {
            players: vec![],
            rooms: vec![start_room, north_room, south_room, east_room, west_room, mountain_peak, orchard, workshop, forest],
            npcs: vec![],
            items: vec![],
            tasks: vec![],
        }
    }
    
    // 查找房间
    pub fn find_room(&self, room_id: &str) -> Option<&Room> {
        self.rooms.iter().find(|room| room.id == room_id)
    }
    
    // 查找玩家
    pub fn find_player(&self, player_id: &str) -> Option<&Player> {
        self.players.iter().find(|player| player.id == player_id)
    }
    
    // 添加玩家
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
    
    // 更新玩家
    pub fn update_player(&mut self, updated_player: Player) {
        if let Some(index) = self.players.iter().position(|p| p.id == updated_player.id) {
            self.players[index] = updated_player;
        }
    }
    
    // 计算伤害
    pub fn calculate_damage(&self, attacker: &Attributes, defender: &Attributes) -> u32 {
        let base_damage = attacker.attack;
        let defense = defender.defense;
        let damage = if base_damage > defense {
            base_damage - defense
        } else {
            1 // 最小伤害为1
        };
        damage
    }
    
    // 玩家攻击NPC
    pub fn player_attack_npc(&mut self, player_id: &str, npc_name: &str) -> CombatResult {
        if let Some(player) = self.find_player(player_id) {
            if let Some(room) = self.find_room(&player.location) {
                if let Some(npc_index) = room.npcs.iter().position(|n| n.name == npc_name) {
                    let mut npc = room.npcs[npc_index].clone();
                    let mut player = player.clone();
                    
                    // 计算伤害
                    let damage = self.calculate_damage(&player.attributes, &npc.attributes);
                    
                    // 减少NPC生命值
                    if npc.attributes.health > damage {
                        npc.attributes.health -= damage;
                    } else {
                        npc.attributes.health = 0;
                    }
                    
                    // 计算NPC反击伤害
                    let counter_damage = self.calculate_damage(&npc.attributes, &player.attributes);
                    if player.attributes.health > counter_damage {
                        player.attributes.health -= counter_damage;
                    } else {
                        player.attributes.health = 0;
                    }
                    
                    // 保存生命值状态
                    let npc_health = npc.attributes.health;
                    let player_health = player.attributes.health;
                    
                    // 更新NPC
                    let mut updated_room = room.clone();
                    updated_room.npcs[npc_index] = npc;
                    if let Some(room_index) = self.rooms.iter().position(|r| r.id == room.id) {
                        self.rooms[room_index] = updated_room;
                    }
                    
                    // 更新玩家
                    self.update_player(player);
                    
                    // 构建战斗结果
                    let message = if npc_health == 0 {
                        format!("你击败了{}！", npc_name)
                    } else if player_health == 0 {
                        "你被击败了！".to_string()
                    } else {
                        format!("你对{}造成了{}点伤害，{}对你造成了{}点伤害。", npc_name, damage, npc_name, counter_damage)
                    };
                    
                    CombatResult {
                        success: true,
                        message,
                        damage,
                        attacker_health: player_health,
                        defender_health: npc_health,
                    }
                } else {
                    CombatResult {
                        success: false,
                        message: format!("这里没有{}.", npc_name),
                        damage: 0,
                        attacker_health: 0,
                        defender_health: 0,
                    }
                }
            } else {
                CombatResult {
                    success: false,
                    message: "你在一个未知的位置。".to_string(),
                    damage: 0,
                    attacker_health: 0,
                    defender_health: 0,
                }
            }
        } else {
            CombatResult {
                success: false,
                message: "找不到玩家。".to_string(),
                damage: 0,
                attacker_health: 0,
                defender_health: 0,
            }
        }
    }
}
