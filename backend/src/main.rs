use std::sync::Arc;
use tokio::sync::Mutex;

mod game;
mod storage;
mod api;
mod ai;

#[tokio::main]
async fn main() {
    // 初始化游戏状态
    let game_state = Arc::new(Mutex::new(game::GameState::new()));
    
    // 初始化存储
    let storage = Arc::new(Mutex::new(storage::MemoryMappedStorage::new("game_data.dat", 1024 * 1024).unwrap()));
    
    // 尝试加载游戏状态
    if let Ok(loaded_state) = storage.lock().await.read::<game::GameState>() {
        *game_state.lock().await = loaded_state;
        println!("Game state loaded from storage");
    } else {
        println!("No existing game state found, using default");
    }
    
    // 初始化AI服务（使用OpenAI API作为示例）
    let ai_service = Arc::new(ai::AIService::new(
        "https://api.openai.com/v1/chat/completions",
        "YOUR_OPENAI_API_KEY", // 这里需要替换为实际的API密钥
        "gpt-3.5-turbo"
    ));
    
    // 创建API路由
    let routes = api::routes(game_state.clone(), storage.clone(), ai_service.clone());
    
    // 启动服务器
    println!("Starting MUD server on 0.0.0.0:3030");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
