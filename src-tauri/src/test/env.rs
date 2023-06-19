use serde_json::{json, Value};





/// ! #chaos #watching
/// 不再使用 env 环境变量, 直接在这里配置
fn my_config()->Value {
    json!({
        "a":0
    })
}