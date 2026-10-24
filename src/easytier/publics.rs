use crate::controller::Room;

pub type PublicServers = Vec<String>;

/// 内置默认节点（qiuyue 自建节点）。
///
/// 想换节点不需要重新编译，按下面的优先级覆盖即可。
const DEFAULT_NODES: &[&str] = &["tcp://47.112.97.138:11010"];

/// 节点列表来源，优先级从高到低：
///
/// 1. 调用方显式传入的 `public_nodes`（HTTP API 的 `?public_nodes=`）
/// 2. 环境变量 `TERRACOTTA_NODES`（逗号 / 分号 / 换行分隔）
/// 3. 可执行文件同目录的 `terracotta-nodes.txt`（每行一个节点，`#` 开头为注释）
/// 4. 内置默认节点 [`DEFAULT_NODES`]
///
/// 这样 `--hmcl`（无界面）模式下也能改节点：只要在 terracotta.exe 旁边放一个
/// `terracotta-nodes.txt`，或者由启动器设置环境变量即可。
pub fn fetch_public_nodes(_: &Room, external_nodes: PublicServers) -> PublicServers {
    let mut nodes: Vec<String> = Vec::new();

    for node in external_nodes {
        push_node(&mut nodes, &node);
    }

    if nodes.is_empty() {
        if let Ok(env_nodes) = std::env::var("TERRACOTTA_NODES") {
            for node in env_nodes.split([',', ';', '\n']) {
                push_node(&mut nodes, node);
            }
        }
    }

    if nodes.is_empty() {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                if let Ok(content) = std::fs::read_to_string(dir.join("terracotta-nodes.txt")) {
                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }
                        push_node(&mut nodes, line);
                    }
                }
            }
        }
    }

    if nodes.is_empty() {
        for node in DEFAULT_NODES {
            push_node(&mut nodes, *node);
        }
    }

    logging!("PublicServers", "Using public servers: {:?}", nodes);
    nodes
}

fn push_node(nodes: &mut Vec<String>, node: &str) {
    let node = node.trim();
    if node.is_empty() {
        return;
    }
    if nodes.iter().any(|existing| existing == node) {
        return;
    }
    nodes.push(node.to_string());
}
