use actix_web::web;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::{entities::rules::ActiveModel, models::rules::Rule};

pub async fn insert_rule_with_child(
    db: &web::Data<sea_orm::DatabaseConnection>,
    rule_list: Vec<Rule>,
) {
    for rule in rule_list {
        let active_model = ActiveModel {
            id: Set(rule.id),
            rule_id: Set(rule.rule_id),
            status: Set(rule.status),
            create_time: Set(rule.create_time),
            update_time: Set(rule.update_time),
            name: Set(rule.name.clone()),
            desc: Set(rule.desc),
            front_path: Set(rule.frontpath),
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
