use super::npc::PromptSystem;

// 物品结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub value: u32,
    pub item_type: String, // Normal, Herb, Weapon, Armor, CultivationItem等
    pub level: u32,
    pub attributes: Option<Attributes>,
    pub prompt_system: PromptSystem,
    pub is_growable: bool, // 是否可成长
    pub growth_stage: u32, // 成长阶段
    pub max_growth_stage: u32, // 最大成长阶段
    pub effects: Vec<Effect>, // 物品效果
    pub durability: Option<u32>, // 耐久度
    pub max_durability: Option<u32>, // 最大耐久度
    pub is_equippable: bool, // 是否可装备
    pub equipment_slot: Option<String>, // 装备槽位
    pub crafting_recipe: Option<CraftingRecipe>, // 制作配方
    pub required_level: u32, // 需求等级
    pub required_cultivation: Option<String>, // 需求修炼类型
    pub rarity: String, // 稀有度：Common, Uncommon, Rare, Epic, Legendary
    pub lore: Option<String>, // 物品 lore
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

// 效果结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Effect {
    pub type_: String, // 效果类型：heal, damage, buff, debuff等
    pub value: i32, // 效果值
    pub duration: Option<u32>, // 持续时间（秒）
    pub target: String, // 目标：self, enemy, ally等
    pub chance: f32, // 触发几率（0.0-1.0）
}

// 制作配方结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct CraftingRecipe {
    pub ingredients: Vec<Ingredient>, // 材料
    pub required_skill: Option<String>, // 需求技能
    pub required_skill_level: u32, // 需求技能等级
    pub crafting_time: u32, // 制作时间（秒）
    pub success_chance: f32, // 成功几率（0.0-1.0）
}

// 材料结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Ingredient {
    pub item_id: String, // 物品ID
    pub quantity: u32, // 数量
}

impl Item {
    // 创建默认提示词系统
    pub fn default_prompt_system(base_prompts: Vec<String>) -> PromptSystem {
        PromptSystem {
            base_prompts: base_prompts.clone(),
            personality_prompts: Vec::new(),
            growth_prompts: vec!["成长中", "成熟", "古老", "神秘"].iter().map(|s| s.to_string()).collect(),
            current_prompts: base_prompts,
        }
    }

    // 创建默认属性
    pub fn default_attributes() -> Attributes {
        Attributes {
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
            attack: 0,
            speed: 0,
        }
    }
}
