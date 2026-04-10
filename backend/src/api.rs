use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket};
use futures::{StreamExt, SinkExt};
use rand;

use crate::game::{GameState, Player};
use crate::storage::MemoryMappedStorage;

// 游戏命令处理
async fn handle_command(
    command: String,
    player_id: &str,
    game_state: &mut GameState
) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return "请输入命令。".to_string();
    }
    
    match parts[0] {
        "help" => {
            "可用命令: help, look, north, south, east, west, say, inventory, take, drop, stats, skills, techniques".to_string()
        }
        "look" => {
            if let Some(player) = game_state.find_player(player_id) {
                if let Some(room) = game_state.find_room(&player.location) {
                    let mut response = format!("{}\n{}\n区域: {}\n\n出口: ", room.name, room.description, room.region);
                    for exit in &room.exits {
                        response.push_str(&format!("{}, ", exit.direction));
                    }
                    if !room.npcs.is_empty() {
                        response.push_str("\n\nNPC: ");
                        for npc in &room.npcs {
                            response.push_str(&format!("{} ({}), ", npc.name, npc.race));
                        }
                    }
                    if !room.items.is_empty() {
                        response.push_str("\n\n物品: ");
                        for item in &room.items {
                            response.push_str(&format!("{} ({}), ", item.name, item.item_type));
                        }
                    }
                    response
                } else {
                    "你在一个未知的位置。".to_string()
                }
            } else {
                "找不到玩家。".to_string()
            }
        }
        "north" | "south" | "east" | "west" => {
            if let Some(player) = game_state.find_player(player_id) {
                if let Some(room) = game_state.find_room(&player.location) {
                    let direction = parts[0];
                    if let Some(exit) = room.exits.iter().find(|e| e.direction == direction) {
                        let target_room = exit.target_room.clone();
                        let mut updated_player = player.clone();
                        updated_player.location = target_room.clone();
                        game_state.update_player(updated_player);
                        if let Some(new_room) = game_state.find_room(&target_room) {
                            format!("你向{}移动。\n\n{}\n{}", direction, new_room.name, new_room.description)
                        } else {
                            "你移动了，但最终到达了一个未知的位置。".to_string()
                        }
                    } else {
                        format!("你不能向{}走。", direction)
                    }
                } else {
                    "你在一个未知的位置。".to_string()
                }
            } else {
                "找不到玩家。".to_string()
            }
        }
        "say" => {
            if parts.len() > 1 {
                let message = parts[1..].join(" ");
                format!("你说: {}", message)
            } else {
                "你想说什么？".to_string()
            }
        }
        "inventory" => {
            if let Some(player) = game_state.find_player(player_id) {
                if player.inventory.is_empty() {
                    "你的背包是空的。".to_string()
                } else {
                    let mut response = "你的背包:\n".to_string();
                    for item in &player.inventory {
                        response.push_str(&format!("- {}: {} (等级: {}, 价值: {})\n", item.name, item.description, item.level, item.value));
                    }
                    response
                }
            } else {
                "找不到玩家。".to_string()
            }
        }
        "take" => {
            if parts.len() > 1 {
                let item_name = parts[1..].join(" ");
                if let Some(player) = game_state.find_player(player_id) {
                    if let Some(room) = game_state.find_room(&player.location) {
                        if let Some(item_index) = room.items.iter().position(|item| item.name == item_name) {
                            let item = room.items[item_index].clone();
                            let mut updated_player = player.clone();
                            updated_player.inventory.push(item);
                            game_state.update_player(updated_player);
                            format!("你拿起了{}.", item_name)
                        } else {
                            format!("这里没有{}.", item_name)
                        }
                    } else {
                        "你在一个未知的位置。".to_string()
                    }
                } else {
                    "找不到玩家。".to_string()
                }
            } else {
                "你想拿什么？".to_string()
            }
        }
        "drop" => {
            if parts.len() > 1 {
                let item_name = parts[1..].join(" ");
                if let Some(player) = game_state.find_player(player_id) {
                    if let Some(item_index) = player.inventory.iter().position(|item| item.name == item_name) {
                        let mut updated_player = player.clone();
                        updated_player.inventory.remove(item_index);
                        game_state.update_player(updated_player);
                        format!("你放下了{}.", item_name)
                    } else {
                        format!("你没有{}.", item_name)
                    }
                } else {
                    "找不到玩家。".to_string()
                }
            } else {
                "你想放下什么？".to_string()
            }
        }
        "stats" => {
            if let Some(player) = game_state.find_player(player_id) {
                let mut response = format!("{} ({} - {})\n", player.name, player.race, player.cultivation_type);
                response.push_str(&format!("等级: {}\n", player.level));
                response.push_str("属性:\n");
                response.push_str(&format!("  力量: {}\n", player.attributes.strength));
                response.push_str(&format!("  敏捷: {}\n", player.attributes.agility));
                response.push_str(&format!("  智力: {}\n", player.attributes.intelligence));
                response.push_str(&format!("  体力: {}\n", player.attributes.vitality));
                response.push_str(&format!("  灵力: {}\n", player.attributes.qi));
                response.push_str(&format!("  灵识: {}\n", player.attributes.spiritual_awareness));
                response.push_str(&format!("  肉身: {}\n", player.attributes.physical_strength));
                response.push_str(&format!("  剑心: {}\n", player.attributes.sword_heart));
                response.push_str(&format!("  生命: {}\n", player.attributes.health));
                response.push_str(&format!("  法力: {}\n", player.attributes.mana));
                response.push_str(&format!("  防御: {}\n", player.attributes.defense));
                response.push_str(&format!("  攻击: {}\n", player.attributes.attack));
                response.push_str(&format!("  速度: {}\n", player.attributes.speed));
                response
            } else {
                "找不到玩家。".to_string()
            }
        }
        "skills" => {
            if let Some(player) = game_state.find_player(player_id) {
                if player.skills.is_empty() {
                    "你没有任何技能。".to_string()
                } else {
                    let mut response = "你的技能:\n".to_string();
                    for skill in &player.skills {
                        response.push_str(&format!("- {} (等级: {}): {} (伤害: {}, 法力消耗: {})\n", skill.name, skill.level, skill.description, skill.damage, skill.mana_cost));
                    }
                    response
                }
            } else {
                "找不到玩家。".to_string()
            }
        }
        "techniques" => {
            if let Some(player) = game_state.find_player(player_id) {
                if player.techniques.is_empty() {
                    "你没有任何功法。".to_string()
                } else {
                    let mut response = "你的功法:\n".to_string();
                    for technique in &player.techniques {
                        response.push_str(&format!("- {} (等级: {}): {} (适用: {})\n", technique.name, technique.level, technique.description, technique.cultivation_type));
                        response.push_str("  效果: ");
                        for effect in &technique.effects {
                            response.push_str(&format!("{}, ", effect));
                        }
                        response.push_str("\n");
                    }
                    response
                }
            } else {
                "找不到玩家。".to_string()
            }
        }
        _ => {
            format!("未知命令: {}. 输入 'help' 查看可用命令。", parts[0])
        }
    }
}

// WebSocket处理
async fn handle_websocket(
    ws: WebSocket,
    game_state: Arc<Mutex<GameState>>,
    storage: Arc<Mutex<MemoryMappedStorage>>
) {
    let (mut tx, mut rx) = ws.split();
    
    // 生成唯一玩家ID
    let player_id = format!("player_{}", rand::random::<u32>());
    
    // 创建新玩家
    {
        let mut state = game_state.lock().await;
        let new_player = Player {
            id: player_id.clone(),
            name: format!("修士{}", player_id.split('_').nth(1).unwrap()),
            level: 1,
            attributes: crate::game::default_attributes(1),
            cultivation_type: "QiCultivator".to_string(), // 默认气修
            race: "人类".to_string(), // 默认人类
            inventory: vec![],
            location: "room1".to_string(), // 起始房间
            skills: vec![
                crate::game::Skill {
                    name: "基础拳法".to_string(),
                    level: 1,
                    description: "基础的拳法技能，适合初学者".to_string(),
                    damage: 10,
                    mana_cost: 5,
                },
            ],
            techniques: vec![
                crate::game::Technique {
                    name: "基础吐纳法".to_string(),
                    level: 1,
                    description: "基础的修炼功法，能够缓慢提升灵力".to_string(),
                    cultivation_type: "QiCultivator".to_string(),
                    effects: vec!["提升灵力回复速度", "增加基础灵力"].iter().map(|s| s.to_string()).collect(),
                },
            ],
            equipment: crate::game::Equipment {
                weapon: None,
                armor: None,
                accessories: vec![],
            },
            prompt_system: crate::game::default_prompt_system(vec!["修士", "初学者", "好奇", "勇敢"].iter().map(|s| s.to_string()).collect()),
        };
        state.add_player(new_player);
        
        // 保存游戏状态
        let mut storage = storage.lock().await;
        storage.write(&*state).unwrap();
    }
    
    // 发送欢迎消息
    let welcome_msg = format!("Welcome to the MUD! Your player ID is {}. Type 'help' for available commands.", player_id);
    tx.send(Message::text(welcome_msg)).await.unwrap();
    
    // 处理命令
    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    let mut state = game_state.lock().await;
                    let response = handle_command(text.to_string(), &player_id, &mut state).await;
                    
                    // 保存游戏状态
                    let mut storage = storage.lock().await;
                    storage.write(&*state).unwrap();
                    
                    tx.send(Message::text(response)).await.unwrap();
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {:?}", e);
                break;
            }
        }
    }
}

// API路由
pub fn routes(
    game_state: Arc<Mutex<GameState>>,
    storage: Arc<Mutex<MemoryMappedStorage>>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // WebSocket路由
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let game_state = game_state.clone();
            let storage = storage.clone();
            ws.on_upgrade(move |socket| handle_websocket(socket, game_state, storage))
        });
    
    // 健康检查路由
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));
    
    // 组合路由
    ws_route.or(health_route)
}
