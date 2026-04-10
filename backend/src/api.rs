use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket};
use futures::{StreamExt, SinkExt};
use rand;

use crate::game::{GameState, Player};
use crate::storage::MemoryMappedStorage;
use crate::ai::AIService;

// 游戏命令处理
async fn handle_command(
    command: String,
    player_id: &str,
    game_state: &mut GameState,
    ai_service: &AIService
) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return "请输入命令。".to_string();
    }
    
    match parts[0] {
        "help" => {
            "可用命令: help, look, north, south, east, west, say, inventory, take, drop, stats, skills, techniques, talk, tasks, accept_task, complete_task, generate_task, attack, money, buy, sell, whisper, broadcast, players".to_string()
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
        "talk" => {
            if parts.len() > 2 {
                let npc_name = parts[1].to_string();
                let player_message = parts[2..].join(" ");
                if let Some(player) = game_state.find_player(player_id) {
                    if let Some(room) = game_state.find_room(&player.location) {
                        if let Some(npc_index) = room.npcs.iter().position(|n| n.name == npc_name) {
                            let mut npc = room.npcs[npc_index].clone();
                            // 构建NPC的提示词
                            let npc_prompts = npc.prompt_system.current_prompts.clone();
                            
                            // 构建消息历史
                            let mut messages = vec![
                                crate::ai::Message {
                                    role: "system".to_string(),
                                    content: format!("你是{}，{}。你在九州世界中与玩家互动。", npc.name, npc_prompts.join(", ")),
                                },
                            ];
                            
                            // 添加对话历史
                            for (speaker, content) in &npc.dialogue_history.messages {
                                let role = if speaker == &npc.name {
                                    "assistant"
                                } else {
                                    "user"
                                };
                                messages.push(crate::ai::Message {
                                    role: role.to_string(),
                                    content: content.clone(),
                                });
                            }
                            
                            // 添加当前玩家消息
                            messages.push(crate::ai::Message {
                                role: "user".to_string(),
                                content: player_message.clone(),
                            });
                            
                            // 生成NPC的响应
                            match ai_service.generate_response(messages).await {
                                Ok(response) => {
                                    // 更新对话历史
                                    npc.dialogue_history.messages.push((player.name.clone(), player_message));
                                    npc.dialogue_history.messages.push((npc.name.clone(), response.clone()));
                                    
                                    // 限制对话历史长度
                                    if npc.dialogue_history.messages.len() > npc.dialogue_history.max_length {
                                        npc.dialogue_history.messages = npc.dialogue_history.messages
                                            .iter()
                                            .skip(npc.dialogue_history.messages.len() - npc.dialogue_history.max_length)
                                            .cloned()
                                            .collect();
                                    }
                                    
                                    // 更新NPC
                                    let mut updated_room = room.clone();
                                    updated_room.npcs[npc_index] = npc;
                                    // 更新游戏状态中的房间
                                    if let Some(room_index) = game_state.rooms.iter().position(|r| r.id == room.id) {
                                        game_state.rooms[room_index] = updated_room;
                                    }
                                    
                                    format!("{}: {}", npc_name, response)
                                }
                                Err(e) => format!("与{}交流时发生错误: {:?}", npc_name, e),
                            }
                        } else {
                            format!("这里没有{}.", npc_name)
                        }
                    } else {
                        "你在一个未知的位置。".to_string()
                    }
                } else {
                    "找不到玩家。".to_string()
                }
            } else {
                "你想和谁说话，说什么？使用格式: talk <NPC名称> <消息>".to_string()
            }
        }
        "tasks" => {
            if let Some(player) = game_state.find_player(player_id) {
                if player.tasks.is_empty() {
                    "你当前没有任务。".to_string()
                } else {
                    let mut response = "你的任务:
".to_string();
                    for task in &player.tasks {
                        let status_str = match task.status {
                            crate::game::TaskStatus::Pending => "待完成",
                            crate::game::TaskStatus::InProgress => "进行中",
                            crate::game::TaskStatus::Completed => "已完成",
                            crate::game::TaskStatus::Failed => "失败",
                        };
                        response.push_str(&format!("- {} ({})
  {}
  目标: {}
  奖励: {}
  进度: {}/{}
", task.title, status_str, task.description, task.target, task.reward, task.progress, task.max_progress));
                    }
                    response
                }
            } else {
                "找不到玩家。".to_string()
            }
        }
        "generate_task" => {
            if let Some(player) = game_state.find_player(player_id) {
                // 生成任务
                match ai_service.generate_task(&player.name, player.level, &player.location, &player.cultivation_type).await {
                    Ok(task_data) => {
                        // 创建新任务
                        let task_id = format!("task_{}", rand::random::<u32>());
                        let new_task = crate::game::Task {
                            id: task_id,
                            title: "新任务".to_string(), // 实际应该从AI响应中解析
                            description: task_data.clone(),
                            target: "完成任务目标".to_string(),
                            reward: "经验和物品".to_string(),
                            status: crate::game::TaskStatus::Pending,
                            progress: 0,
                            max_progress: 1,
                            expiration: None,
                        };
                        
                        // 添加到玩家任务列表
                        let mut updated_player = player.clone();
                        updated_player.tasks.push(new_task.clone());
                        game_state.update_player(updated_player);
                        
                        // 添加到全局任务列表
                        game_state.tasks.push(new_task);
                        
                        format!("生成了新任务: {}", task_data)
                    }
                    Err(e) => format!("生成任务时发生错误: {:?}", e),
                }
            } else {
                "找不到玩家。".to_string()
            }
        }
        "accept_task" => {
            if parts.len() > 1 {
                let task_id = parts[1].to_string();
                if let Some(player) = game_state.find_player(player_id) {
                    // 查找任务
                    if let Some(task_index) = player.tasks.iter().position(|t| t.id == task_id) {
                        let task_title = player.tasks[task_index].title.clone();
                        let mut updated_player = player.clone();
                        updated_player.tasks[task_index].status = crate::game::TaskStatus::InProgress;
                        game_state.update_player(updated_player);
                        format!("你接受了任务: {}", task_title)
                    } else {
                        format!("找不到任务: {}", task_id)
                    }
                } else {
                    "找不到玩家。".to_string()
                }
            } else {
                "请指定要接受的任务ID。".to_string()
            }
        }
        "complete_task" => {
            if parts.len() > 1 {
                let task_id = parts[1].to_string();
                if let Some(player) = game_state.find_player(player_id) {
                    // 查找任务
                    if let Some(task_index) = player.tasks.iter().position(|t| t.id == task_id) {
                        let task_title = player.tasks[task_index].title.clone();
                        let task_reward = player.tasks[task_index].reward.clone();
                        let mut updated_player = player.clone();
                        updated_player.tasks[task_index].status = crate::game::TaskStatus::Completed;
                        updated_player.tasks[task_index].progress = updated_player.tasks[task_index].max_progress;
                        game_state.update_player(updated_player);
                        format!("你完成了任务: {}，获得奖励: {}", task_title, task_reward)
                    } else {
                        format!("找不到任务: {}", task_id)
                    }
                } else {
                    "找不到玩家。".to_string()
                }
            } else {
                "请指定要完成的任务ID。".to_string()
            }
        }
        "attack" => {
            if parts.len() > 1 {
                let npc_name = parts[1..].join(" ");
                let result = game_state.player_attack_npc(player_id, &npc_name);
                result.message
            } else {
                "你想攻击谁？使用格式: attack <NPC名称>".to_string()
            }
        }
        "money" => {
            if let Some(player) = game_state.find_player(player_id) {
                format!("你当前有 {} 金币。", player.currency)
            } else {
                "找不到玩家。".to_string()
            }
        }
        "buy" => {
            if parts.len() > 1 {
                let item_name = parts[1..].join(" ");
                if let Some(player) = game_state.find_player(player_id) {
                    let player_location = player.location.clone();
                    let room_id = player_location.clone();
                    let player_currency = player.currency;
                    
                    // 先检查物品是否存在并获取物品信息
                    let mut item_to_buy = None;
                    if let Some(room) = game_state.find_room(&player_location) {
                        if let Some(item_index) = room.items.iter().position(|item| item.name == item_name) {
                            item_to_buy = Some(room.items[item_index].clone());
                        }
                    }
                    
                    // 处理购买逻辑
                    if let Some(item) = item_to_buy {
                        if player_currency >= item.value {
                            // 扣除货币
                            let mut updated_player = player.clone();
                            updated_player.currency -= item.value;
                            updated_player.inventory.push(item.clone());
                            game_state.update_player(updated_player);
                            
                            // 从房间中移除物品
                            if let Some(room_index) = game_state.rooms.iter().position(|r| r.id == room_id) {
                                let mut updated_room = game_state.rooms[room_index].clone();
                                if let Some(item_index) = updated_room.items.iter().position(|i| i.name == item_name) {
                                    updated_room.items.remove(item_index);
                                    game_state.rooms[room_index] = updated_room;
                                }
                            }
                            
                            format!("你购买了 {}，花费了 {} 金币。", item_name, item.value)
                        } else {
                            format!("你没有足够的金币购买 {}.", item_name)
                        }
                    } else {
                        format!("这里没有 {}.", item_name)
                    }
                } else {
                    "找不到玩家。".to_string()
                }
            } else {
                "你想买什么？使用格式: buy <物品名称>".to_string()
            }
        }
        "sell" => {
            if parts.len() > 1 {
                let item_name = parts[1..].join(" ");
                if let Some(player) = game_state.find_player(player_id) {
                    // 查找玩家背包中的物品
                    if let Some(item_index) = player.inventory.iter().position(|item| item.name == item_name) {
                        let item = player.inventory[item_index].clone();
                        let sell_price = item.value / 2; // 出售价格为物品价值的一半
                        
                        // 增加货币
                        let mut updated_player = player.clone();
                        updated_player.currency += sell_price;
                        updated_player.inventory.remove(item_index);
                        game_state.update_player(updated_player);
                        
                        format!("你出售了 {}，获得了 {} 金币。", item_name, sell_price)
                    } else {
                        format!("你没有 {}.", item_name)
                    }
                } else {
                    "找不到玩家。".to_string()
                }
            } else {
                "你想卖什么？使用格式: sell <物品名称>".to_string()
            }
        }
        "whisper" => {
            if parts.len() > 2 {
                let target_player_name = parts[1].to_string();
                let message = parts[2..].join(" ");
                if let Some(player) = game_state.find_player(player_id) {
                    // 查找目标玩家
                    let mut found = false;
                    for target_player in &game_state.players {
                        if target_player.name == target_player_name {
                            found = true;
                            // 这里可以实现私聊功能，需要在WebSocket处理中添加消息转发逻辑
                            break;
                        }
                    }
                    if found {
                        format!("你悄悄对 {} 说: {}", target_player_name, message)
                    } else {
                        format!("找不到玩家: {}", target_player_name)
                    }
                } else {
                    "找不到玩家。".to_string()
                }
            } else {
                "你想对谁说什么？使用格式: whisper <玩家名称> <消息>".to_string()
            }
        }
        "broadcast" => {
            if parts.len() > 1 {
                let message = parts[1..].join(" ");
                if let Some(player) = game_state.find_player(player_id) {
                    // 这里可以实现广播功能，需要在WebSocket处理中添加消息转发逻辑
                    format!("你广播: {}", message)
                } else {
                    "找不到玩家。".to_string()
                }
            } else {
                "你想广播什么？使用格式: broadcast <消息>".to_string()
            }
        }
        "players" => {
            if let Some(player) = game_state.find_player(player_id) {
                if game_state.players.is_empty() {
                    "当前没有其他玩家在线。".to_string()
                } else {
                    let mut response = "在线玩家:
".to_string();
                    for p in &game_state.players {
                        response.push_str(&format!("- {}
", p.name));
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
    storage: Arc<Mutex<MemoryMappedStorage>>,
    ai_service: Arc<AIService>
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
            tasks: vec![],
            currency: 100, // 初始货币
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
                    let response = handle_command(text.to_string(), &player_id, &mut state, &ai_service).await;
                    
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
    storage: Arc<Mutex<MemoryMappedStorage>>,
    ai_service: Arc<AIService>
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // 添加CORS支持
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);
    
    // WebSocket路由
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let game_state = game_state.clone();
            let storage = storage.clone();
            let ai_service = ai_service.clone();
            ws.on_upgrade(move |socket| handle_websocket(socket, game_state, storage, ai_service))
        });
    
    // 健康检查路由
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));
    
    // 静态文件服务
    let static_route = warp::path::path("static")
        .and(warp::fs::dir("../frontend"));
    
    // 根路径重定向到index.html
    use warp::redirect;
    use warp::http::Uri;
    let root_route = warp::path::end()
        .and(warp::get())
        .map(|| redirect::found(Uri::try_from("/static/index.html").unwrap()));
    
    // 组合路由并添加CORS
    ws_route.or(health_route).or(static_route).or(root_route).with(cors)
}
