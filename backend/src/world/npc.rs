// 对话历史结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DialogueHistory {
    pub messages: Vec<(String, String)>, // (角色, 内容)
    pub max_length: usize,
}

// 提示词系统结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct PromptSystem {
    pub base_prompts: Vec<String>, // 基础提示词
    pub personality_prompts: Vec<String>, // 性格提示词
    pub growth_prompts: Vec<String>, // 成长提示词
    pub current_prompts: Vec<String>, // 当前提示词组合
}

// NPC结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Npc {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: u32,
    pub attributes: Attributes,
    pub character_type: String, // NPC, Enemy, Merchant等
    pub race: String, // 人类, 妖兽, 魔族等
    pub ai_type: String, // merchant, herbalist, monster等
    pub prompt_system: PromptSystem,
    pub behaviors: Vec<String>, // 行为模式
    pub dialogue_history: DialogueHistory,
    pub location: String, // 所在房间ID
    pub schedule: Option<Vec<Schedule>>, // 日程安排
    pub relationships: Vec<Relationship>, // 与其他NPC的关系
    pub inventory: Vec<String>, // 物品ID列表
    pub shop_items: Option<Vec<ShopItem>>, // 商店物品
}

// 属性结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
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

// 日程结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Schedule {
    pub time: String, // 时间
    pub activity: String, // 活动
    pub location: String, // 地点
}

// 关系结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Relationship {
    pub npc_id: String, // 目标NPC ID
    pub type_: String, // 关系类型：friend, enemy, family等
    pub value: i32, // 关系值
}

// 商店物品结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ShopItem {
    pub item_id: String, // 物品ID
    pub price: u32, // 价格
    pub stock: u32, // 库存
    pub is_unlimited: bool, // 是否无限供应
}

impl Npc {
    // 创建默认对话历史
    pub fn default_dialogue_history() -> DialogueHistory {
        DialogueHistory {
            messages: Vec::new(),
            max_length: 10,
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
}
