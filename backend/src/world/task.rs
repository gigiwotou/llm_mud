// 任务状态枚举
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending, // 待接受
    InProgress, // 进行中
    Completed, // 已完成
    Failed, // 失败
    Abandoned, // 已放弃
}

// 任务目标结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TaskObjective {
    pub type_: String, // 目标类型：kill, collect, talk, reach等
    pub target_id: String, // 目标ID：NPC ID, 物品ID, 房间ID等
    pub required_count: u32, // 需求数量
    pub current_count: u32, // 当前数量
    pub description: String, // 目标描述
}

// 任务奖励结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TaskReward {
    pub experience: u32, // 经验值
    pub currency: u32, // 货币
    pub items: Vec<RewardItem>, // 物品奖励
    pub skills: Vec<String>, // 技能奖励
    pub techniques: Vec<String>, // 功法奖励
    pub reputation: Option<ReputationReward>, // 声望奖励
}

// 奖励物品结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct RewardItem {
    pub item_id: String, // 物品ID
    pub quantity: u32, // 数量
    pub is_unique: bool, // 是否唯一
}

// 声望奖励结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ReputationReward {
    pub faction_id: String, // 势力ID
    pub value: i32, // 声望值
}

// 任务结构体
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub objectives: Vec<TaskObjective>, // 任务目标
    pub reward: TaskReward, // 任务奖励
    pub status: TaskStatus, // 任务状态
    pub difficulty: u32, // 任务难度
    pub recommended_level: u32, // 推荐等级
    pub required_tasks: Vec<String>, // 前置任务
    pub repeatable: bool, // 是否可重复
    pub max_repeats: Option<u32>, // 最大重复次数
    pub current_repeats: u32, // 当前重复次数
    pub expiration: Option<u64>, // 过期时间（时间戳）
    pub giver_id: Option<String>, // 任务发布者ID
    pub location_id: Option<String>, // 任务接取地点ID
    pub lore: Option<String>, // 任务背景故事
}

impl Task {
    // 检查任务是否完成
    pub fn is_completed(&self) -> bool {
        self.objectives.iter().all(|obj| obj.current_count >= obj.required_count)
    }

    // 检查任务是否可接取
    pub fn can_accept(&self) -> bool {
        self.status == TaskStatus::Pending
    }

    // 检查任务是否可提交
    pub fn can_turn_in(&self) -> bool {
        self.status == TaskStatus::InProgress && self.is_completed()
    }
}
