// 需求:
// 在持久化存储数据的时候，数据存储需要支持不同的存储引擎，比如 Redis、本地文件、MySQL 等等
// 此时如何用 Rust 来实现这个存储层，适配不同类型的存储，该怎么写？
// 再加一个条件，这个存储层需要能在多线程环境下运行

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

// pub fn build_driver() -> Result<Arc<dyn AuthStorageAdapter + Send + 'static + Sync>, RobustMQError>
// {
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
