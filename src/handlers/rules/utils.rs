use actix_web::web;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{
    entities::rules::{ActiveModel, Model},
    models::rules::StructRule,
};

pub async fn insert_rule_with_child(
    db: &web::Data<sea_orm::DatabaseConnection>,
    rule_list: Vec<StructRule>,
) {
    for rule in rule_list {
        let active_model = ActiveModel {
            id: Set(rule.id),
            rule_id: Set(rule.rule_id),
            status: Set(rule.status),
            create_time: Set(rule.create_time),
            update_time: Set(rule.update_time),
            name: Set(rule.name.clone()),
            front_path: Set(rule.front_path),
            condition: Set(rule.condition),
            menu: Set(rule.menu),
            order: Set(rule.order),
            icon: Set(rule.icon),
            method: Set(rule.method),
        };
        let insert_result = active_model.insert(db.get_ref()).await;
        match insert_result {
            Ok(_) => println!("成功插入规则"),
            Err(err) => println!("插入失败: {:?}", err),
        }
        if rule.child.len() != 0 {
            let _ = Box::pin(insert_rule_with_child(db, rule.child)).await;
        }
    }
}

pub fn build_rule_tree(nodes: Vec<Model>, rule_id: i32) -> Vec<StructRule> {
    let mut tree: Vec<StructRule> = Vec::new();

    // 找到所有 parent_rule_id 为当前父节点的节点
    for node in nodes.iter().filter(|&node| node.rule_id == rule_id) {
        let mut node_with_children = StructRule {
            id: node.id,
            name: node.name.clone(),
            rule_id: node.rule_id,
            status: node.status,
            create_time: node.create_time.clone(),
            update_time: node.update_time.clone(),
            front_path: node.front_path.clone(),
            condition: node.condition.clone(),
            menu: node.menu,
            order: node.order,
            icon: node.icon.clone(),
            method: node.method.clone(),
            child: vec![],
        };
        // 递归构建子树
        node_with_children.child = build_rule_tree(nodes.clone(), node.id);
        tree.push(node_with_children);
    }

    tree
}
