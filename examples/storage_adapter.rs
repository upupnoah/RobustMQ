// 需求:
// 在持久化存储数据的时候，数据存储需要支持不同的存储引擎，比如 Redis、本地文件、MySQL 等等
// 此时如何用 Rust 来实现这个存储层，适配不同类型的存储，该怎么写？
// 再加一个条件，这个存储层需要能在多线程环境下运行

// // 存储适配器特征定义
// pub trait AuthStorageAdapter {
//     // 方法1: 读取所有用户信息
//     async fn read_all_user(&self) -> Result<DashMap<String, MQTTUser>, MQError>;
//     // 方法2: 获取单个用户信息
//     async fn get_user(&self, username: String) -> Result<Option<MQTTUser>, MQError>;
// }

// pub struct PlacementAuthStorageAdapter {}

// impl PlacementAuthStorageAdapter {
//     pub fn new() -> Self {
//         return PlacementAuthStorageAdapter {};
//     }
// }

// #[async_trait]
// impl AuthStorageAdapter for PlacementAuthStorageAdapter {
//     async fn read_all_user(&self) -> Result<DashMap<String, MQTTUser>, RobustMQError> {
//         return Ok(DashMap::with_capacity(2));
//     }

//     async fn get_user(&self, username: String) -> Result<Option<MQTTUser>, RobustMQError> {
//         return Ok(None);
//     }
// }

// pub struct MySQLAuthStorageAdapter {}

// impl MySQLAuthStorageAdapter {
//     pub fn new() -> Self {
//         return PlacementAuthStorageAdapter {};
//     }
// }

// #[async_trait]
// impl AuthStorageAdapter for MySQLAuthStorageAdapter {
//     async fn read_all_user(&self) -> Result<DashMap<String, MQTTUser>, RobustMQError> {
//         return Ok(DashMap::with_capacity(2));
//     }

//     async fn get_user(&self, username: String) -> Result<Option<MQTTUser>, RobustMQError> {
//         return Ok(None);
//     }
// }

/// 构建存储适配器实例
///
/// 返回类型 Result<Arc<dyn AuthStorageAdapter + Send + 'static + Sync>, RobustMQError> 的设计考虑:
///
/// 1. Arc<...> - 原子引用计数智能指针
///    - 允许在多个线程间安全地共享存储适配器实例
///    - 通过引用计数管理实例生命周期
///
/// 2. dyn AuthStorageAdapter - 特征对象
///    - 支持运行时动态分发，可以根据配置切换不同的存储实现
///    - 调用方无需知道具体的存储类型，只需要使用特征定义的接口
///
/// 3. Send + Sync - 并发安全标记特征
///    - Send: 允许在线程间转移所有权
///    - Sync: 允许多个线程同时访问
///    - 确保存储适配器可以安全地在并发环境中使用
///
/// 4. 'static - 静态生命周期约束
///    - 确保存储适配器实例在整个程序运行期间都有效
///    - 适合作为长期运行的系统组件
///
/// 5. Result<T, RobustMQError> - 错误处理
///    - 显式处理存储初始化可能发生的错误（如连接失败）
///    - 让调用方能够进行适当的错误处理
// pub fn build_driver() -> Result<Arc<dyn AuthStorageAdapter + Send + 'static + Sync>, RobustMQError> {
//     if storage_is_placement(&auth.storage_type) {
//         let driver = PlacementAuthStorageAdapter::new();
//         return Ok(Arc::new(driver));
//     }
//     if storage_is_mysql(&auth.storage_type) {
//         let driver = MySQLAuthStorageAdapter::new();
//         return Ok(Arc::new(driver));
//     }
//     return Err(RobustMQError::UnavailableStorageType);
// }
fn main() {
    println!("Hello, world!");
}

// 伪代码展示使用场景
// async fn handle_client_connect(username: String, password: String) {
//     // auth_storage 可能是文件存储或MySQL存储的实现
//     let auth_storage = build_driver();

//     // 获取用户信息进行认证
//     match auth_storage.get_user(username).await {
//         Ok(Some(user)) => {
//             if user.password == password {
//                 // 认证成功
//                 allow_connection();
//             }
//         }
//         _ => deny_connection(),
//     }
// }

// 系统启动时加载所有用户信息到内存缓存
// async fn init_auth_cache() {
//     let auth_storage = build_driver();
//     let all_users = auth_storage.read_all_user().await.unwrap();
//     GLOBAL_USER_CACHE.extend(all_users);
// }
