# 游戏世界框架设计文档

## 1. 整体架构

### 1.1 层次结构

游戏世界采用分层结构，从高到低依次为：

1. **世界 (World)** - 最高级别的容器，包含多个区域
2. **区域 (Region)** - 包含多个房间，如城市、森林、山脉等地理区域
3. **房间 (Room)** - 基本的游戏空间，包含NPC、物品、出口等
4. **实体 (Entity)** - 包括NPC、玩家、敌人等可交互对象
5. **物品 (Item)** - 游戏中的各种物品，如武器、药品、材料等
6. **任务 (Task)** - 游戏中的任务系统

### 1.2 核心设计原则

- **模块化**：每个层次都是独立的模块，可单独加载和修改
- **数据与逻辑分离**：游戏数据存储在配置文件中，不硬编码在代码里
- **目录化存储**：使用目录结构组织游戏世界数据，便于管理和扩展
- **可扩展性**：支持动态添加新的区域、房间、NPC和物品
- **版本控制友好**：便于使用Git等版本控制工具管理世界数据

## 2. 数据模型

### 2.1 世界 (World)

```rust
struct GameWorld {
    id: String,              // 世界唯一标识符
    name: String,            // 世界名称
    description: String,     // 世界描述
    regions: Vec<Region>,     // 包含的区域
    global_npcs: Vec<Npc>,    // 全局NPC（如系统管理员）
    global_items: Vec<Item>,  // 全局物品
    tasks: Vec<Task>,         // 全局任务
    economy: Economy,         // 经济系统
    social: SocialSystem,     // 社交系统
}
```

### 2.2 区域 (Region)

```rust
struct Region {
    id: String,              // 区域唯一标识符
    name: String,            // 区域名称
    description: String,     // 区域描述
    rooms: Vec<Room>,         // 包含的房间
    npcs: Vec<Npc>,           // 区域内的NPC
    items: Vec<Item>,         // 区域内的物品
    weather: Weather,         // 区域天气系统
    danger_level: u32,        // 危险等级
}
```

### 2.3 房间 (Room)

```rust
struct Room {
    id: String,              // 房间唯一标识符
    name: String,            // 房间名称
    description: String,     // 房间描述
    exits: Vec<Exit>,         // 出口
    npcs: Vec<Npc>,           // 房间内的NPC
    items: Vec<Item>,         // 房间内的物品
    environment: Environment, // 环境属性
    is_safe: bool,            // 是否安全区域
}

struct Exit {
    direction: String,        // 方向（如北、南、东、西）
    target_room_id: String,   // 目标房间ID
    description: String,      // 出口描述
    is_locked: bool,          // 是否锁定
    key_item_id: Option<String>, // 解锁需要的物品ID
}
```

### 2.4 实体 (Entity)

```rust
struct Entity {
    id: String,              // 实体唯一标识符
    name: String,            // 实体名称
    entity_type: EntityType,  // 实体类型（NPC、玩家、敌人）
    race: Race,              // 种族（人类、妖兽、魔族）
    attributes: Attributes,   // 属性
    skills: Vec<Skill>,       // 技能
    inventory: Inventory,     // 背包
    cultivation: Cultivation, // 修炼境界
    behavior: Behavior,       // 行为模式
    prompt: Prompt,           // AI提示词系统
}

enum EntityType {
    NPC,
    Player,
    Enemy,
}

enum Race {
    Human,
    Monster,
    Demon,
}

struct Attributes {
    strength: u32,            // 力量
    agility: u32,             // 敏捷
    intelligence: u32,        // 智力
    vitality: u32,            // 体力
    mana: u32,                // 法力
    spirit: u32,              // 精神
}

struct Cultivation {
    level: u32,               // 修炼等级
    cultivation_type: CultivationType, // 修炼类型（气修、体修、剑修）
    realm: String,            // 境界名称
    mana_capacity: u32,       // 法力上限
    spiritual_power: u32,     // 灵力
}

enum CultivationType {
    Qi,                       // 气修
    Body,                     // 体修
    Sword,                    // 剑修
}

struct Prompt {
    base_prompt: String,      // 基础提示词
    personality_prompt: String, // 性格提示词
    situation_prompt: String,  // 情境提示词
    growth_prompt: String,     // 成长提示词
}
```

### 2.5 物品 (Item)

```rust
struct Item {
    id: String,              // 物品唯一标识符
    name: String,            // 物品名称
    description: String,     // 物品描述
    item_type: ItemType,      // 物品类型
    rarity: Rarity,          // 稀有度
    effects: Vec<Effect>,     // 物品效果
    attributes: Attributes,   // 物品属性
    craft_recipe: Option<CraftRecipe>, // 制作配方
    prompt: Prompt,           // AI提示词系统
    is_usable: bool,          // 是否可使用
    is_equippable: bool,      // 是否可装备
    weight: f32,              // 重量
    value: u32,               // 价值
}

enum ItemType {
    Weapon,                   // 武器
    Armor,                    // 防具
    Consumable,               // 消耗品
    Material,                 // 材料
    Key,                      // 钥匙
    Treasure,                 // 宝物
    Cultivation,              // 修炼相关
    Other,                    // 其他
}

enum Rarity {
    Common,                   // 普通
    Uncommon,                 //  uncommon
    Rare,                     // 稀有
    Epic,                     // 史诗
    Legendary,                // 传说
    Mythic,                   // 神话
}

struct Effect {
    effect_type: EffectType,  // 效果类型
    value: f32,               // 效果值
    duration: Option<u32>,    // 持续时间（秒）
}

enum EffectType {
    Heal,                     // 治疗
    Damage,                   // 伤害
    Buff,                     // 增益
    Debuff,                   // 减益
    Teleport,                 // 传送
    Summon,                   // 召唤
    Other,                    // 其他
}

struct CraftRecipe {
    ingredients: Vec<Ingredient>, // 材料
    tools: Vec<String>,        // 工具
    skill_required: Option<String>, // 所需技能
    success_rate: f32,         // 成功率
}

struct Ingredient {
    item_id: String,          // 物品ID
    quantity: u32,            // 数量
}
```

### 2.6 任务 (Task)

```rust
struct Task {
    id: String,              // 任务唯一标识符
    name: String,            // 任务名称
    description: String,     // 任务描述
    objectives: Vec<Objective>, // 任务目标
    rewards: Vec<Reward>,     // 任务奖励
    difficulty: u32,          // 难度
    required_level: u32,      // 所需等级
    is_repeatable: bool,      // 是否可重复
    expiration_time: Option<u64>, // 过期时间
}

struct Objective {
    objective_type: ObjectiveType, // 目标类型
    target_id: String,         // 目标ID
    quantity: u32,             // 数量
    is_completed: bool,        // 是否已完成
}

enum ObjectiveType {
    Kill,                     // 杀死
    Collect,                  // 收集
    Talk,                     // 对话
    Deliver,                  // 交付
    Explore,                  // 探索
    Other,                    // 其他
}

struct Reward {
    reward_type: RewardType,   // 奖励类型
    item_id: Option<String>,   // 物品ID
    quantity: u32,             // 数量
    experience: u32,           // 经验值
    gold: u32,                 // 金币
}

enum RewardType {
    Item,                     // 物品
    Experience,               // 经验
    Gold,                     // 金币
    Skill,                    // 技能
    Title,                    // 称号
    Other,                    // 其他
}
```

## 3. 存储系统

### 3.1 目录结构

游戏世界数据采用目录结构存储，每个层次对应一个目录，每个实体对应一个文件：

```
world/
├── world.json              # 世界配置文件
├── regions/
│   ├── region1/
│   │   ├── region.json     # 区域配置文件
│   │   ├── rooms/
│   │   │   ├── room1.json
│   │   │   ├── room2.json
│   │   │   └── ...
│   │   ├── npcs/
│   │   │   ├── npc1.json
│   │   │   ├── npc2.json
│   │   │   └── ...
│   │   └── items/
│   │       ├── item1.json
│   │       ├── item2.json
│   │       └── ...
│   ├── region2/
│   └── ...
├── global_npcs/
│   ├── npc1.json
│   ├── npc2.json
│   └── ...
├── global_items/
│   ├── item1.json
│   ├── item2.json
│   └── ...
└── tasks/
    ├── task1.json
    ├── task2.json
    └── ...
```

### 3.2 文件格式

所有配置文件均使用JSON格式，便于人类阅读和编辑，同时也便于程序解析。

### 3.3 加载机制

1. **世界加载**：
   - 首先加载 `world.json` 文件，获取世界基本信息
   - 然后加载各个区域、全局NPC、全局物品和任务

2. **区域加载**：
   - 加载 `region.json` 文件，获取区域基本信息
   - 然后加载区域内的房间、NPC和物品

3. **房间加载**：
   - 加载 `room.json` 文件，获取房间基本信息和出口
   - 然后加载房间内的NPC和物品

### 3.4 保存机制

1. **世界保存**：
   - 保存 `world.json` 文件
   - 保存各个区域、全局NPC、全局物品和任务

2. **区域保存**：
   - 保存 `region.json` 文件
   - 保存区域内的房间、NPC和物品

3. **房间保存**：
   - 保存 `room.json` 文件
   - 保存房间内的NPC和物品

## 4. 世界构建工具

### 4.1 功能需求

1. **可视化编辑**：提供直观的界面编辑游戏世界
2. **层级管理**：支持在不同层级之间导航和编辑
3. **实体创建**：支持创建新的区域、房间、NPC、物品和任务
4. **属性编辑**：支持编辑各种实体的属性
5. **关系管理**：支持管理房间之间的连接、任务目标和奖励等关系
6. **导入导出**：支持导入导出游戏世界数据
7. **预览功能**：支持预览游戏世界的效果

### 4.2 界面设计

1. **左侧导航栏**：显示世界层级结构，可展开和折叠
2. **右侧编辑区**：显示当前选中实体的编辑表单
3. **顶部工具栏**：提供保存、导入、导出等操作
4. **底部状态栏**：显示当前操作状态和提示信息

### 4.3 技术实现

- **前端**：使用HTML、CSS和JavaScript实现，可集成到游戏前端或作为独立工具
- **后端**：使用Rust实现，提供API接口用于加载和保存游戏世界数据
- **数据传输**：使用JSON格式传输数据

## 5. 可扩展性设计

### 5.1 模块化设计

- **插件系统**：支持通过插件扩展游戏世界功能
- **脚本系统**：支持使用脚本语言（如Lua）编写游戏逻辑
- **事件系统**：支持通过事件触发游戏逻辑

### 5.2 动态加载

- **运行时加载**：支持在游戏运行时动态加载新的区域、房间、NPC和物品
- **热更新**：支持在游戏运行时更新游戏世界数据，无需重启游戏

### 5.3 多语言支持

- **国际化**：支持多语言版本的游戏世界描述
- **本地化**：支持根据玩家语言显示不同的游戏世界内容

## 6. 性能优化

### 6.1 加载优化

- **懒加载**：只加载当前需要的游戏世界数据
- **缓存机制**：缓存已加载的游戏世界数据，减少重复加载

### 6.2 存储优化

- **压缩存储**：对于大型游戏世界数据，使用压缩格式存储
- **索引机制**：建立索引，加速游戏世界数据的查找

### 6.3 内存优化

- **内存管理**：合理管理游戏世界数据的内存使用
- **对象池**：使用对象池减少内存分配和回收的开销

## 7. 安全性考虑

### 7.1 数据验证

- **输入验证**：验证用户输入的游戏世界数据，防止无效数据
- **数据一致性**：确保游戏世界数据的一致性，防止数据损坏

### 7.2 权限控制

- **编辑权限**：控制谁可以编辑游戏世界数据
- **版本控制**：使用版本控制工具管理游戏世界数据的变更

## 8. 实施计划

### 8.1 阶段一：基础框架

1. 实现游戏世界的基本数据模型
2. 实现基于目录结构的存储系统
3. 实现加载和保存机制

### 8.2 阶段二：工具开发

1. 开发世界构建工具的前端界面
2. 开发世界构建工具的后端API
3. 实现导入导出功能

### 8.3 阶段三：功能扩展

1. 实现插件系统
2. 实现脚本系统
3. 实现事件系统

### 8.4 阶段四：优化和测试

1. 优化加载和保存性能
2. 测试游戏世界框架的稳定性
3. 修复发现的问题

## 9. 总结

本设计文档提供了一个灵活、可扩展的游戏世界框架，通过目录化存储和模块化设计，实现了游戏世界数据与逻辑的分离，使游戏世界可以独立于代码进行设计和修改。同时，提供了直观的世界构建工具，便于开发者和内容创作者创建丰富的游戏世界。

该框架支持大型游戏世界的管理，通过懒加载和缓存机制优化性能，同时通过插件系统和脚本系统提供了高度的可扩展性。

通过本框架，开发者可以专注于游戏内容的创作，而不必担心技术实现的细节，从而更高效地创建丰富、动态的游戏世界。