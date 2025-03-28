use std::fs::File;
use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use futures_util::{StreamExt, TryStreamExt};
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::entities::images::{self, ActiveModel, Model};
use crate::models::images::{StructDeleteImageAllReq, StructUpdateImageReq, StructUploadImageReq};
use crate::utils::response::{response_t, ResponseT};

/// 上传图片
///
/// # 请求体
///
/// 需要一个 FormData 对象，包含以下字段：
/// - `imageClassId` ：整数，图库 ID（必填）
/// - `img` ：二进制，图片（必填）
///
/// # 响应
///
/// - 成功：状态码 200，新上传的图片
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    post,
    path = "/api/images/upload",
    request_body(content_type = "multipart/form-data", content = StructUploadImageReq, description = "File to upload"),
    responses(
        (status = 200, description = "OK")
    ),
    tag = "images"
)]
pub async fn upload_file(
    db: web::Data<sea_orm::DatabaseConnection>,
    mut multipart: Multipart,
) -> impl Responder {
    let mut form_data: Option<StructUploadImageReq> = None;

    while let Some(mut field) = multipart.try_next().await.unwrap() {
        let name = field.name().unwrap();

        if form_data.is_none() {
            form_data = Some(StructUploadImageReq {
                image_class_id: -1,
                img: String::new(),
            });
        }

        if name == "imageClassId" {
            let mut value = String::new();
            while let Some(chunk) = field.next().await {
                value.push_str(&String::from_utf8_lossy(&chunk.unwrap()));
            }
            let parsed_id = value.trim().parse::<i32>().unwrap();
            form_data.as_mut().unwrap().image_class_id = parsed_id;
        } else if name == "img" {
            let content = field.content_disposition().unwrap();

            let filename = content.get_filename().unwrap();

            let data_url = format!("upload/{}", filename);
            let mut file = File::create(&data_url).unwrap();
            let mut stream = field.into_stream();
            while let Some(chunk) = stream.next().await {
                let data = chunk.unwrap();
                file.write_all(&data).unwrap();
            }
            form_data.as_mut().unwrap().img = data_url;
        }
    }

    if form_data.is_none() {
        return HttpResponse::Ok().body("File uploaded successfully!");
    }

    let image_data = match form_data {
        Some(data) => data,
        None => return HttpResponse::BadRequest().body("Missing form data!"),
    };

    let now = Utc::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let new_image = ActiveModel {
        name: Set(image_data.img.clone()),
        url: Set(image_data.img.clone()),
        path: Set(image_data.img.clone()),
        image_class_id: Set(image_data.image_class_id),
        create_time: Set(format_time.clone()),
        update_time: Set(format_time.clone()),
        ..Default::default()
    };
    let result = new_image.insert(db.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None)),
        Err(err) => HttpResponse::NotImplemented().json(format!("Error inserting rule: {:?}", err)),
    }
}

/// 删除图片
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `ids` ：整数向量，图片 ID 数组（必填）
///
/// # 响应
///
/// - 成功：状态码 200，Ok
/// - 失败：状态码 500
///
#[utoipa::path(
    delete,
    path = "/api/images/delete_all",
    request_body = StructDeleteImageAllReq,
    responses(
        (status = 200, description = "图片删除成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "images"
)]
pub async fn delete_all_image(
    db: web::Data<sea_orm::DatabaseConnection>,
    image_data: web::Json<StructDeleteImageAllReq>,
) -> impl Responder {
    let result = images::Entity::delete_many()
        .filter(images::Column::Id.is_in(image_data.ids.clone()))
        .exec(db.get_ref())
        .await;
    match result {
        Ok(demo) => {
            println!("{:?}", demo);
            return HttpResponse::Ok().json(response_t(Some(200), Some(String::from("OK")), None));
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// 修改图片名称
///
/// # 路径
///
/// - `id` ：整数，图片 ID（必填）
///
/// # 请求体
///
/// 需要一个 JSON 对象，包含以下字段：
/// - `name` ：字符串，名称（必填）
///
/// # 响应
///
/// - 成功：状态码 200，修改之后的图片
/// - 失败：状态码 201，用户名已存在
/// - 失败：状态码 500
///
#[utoipa::path(
    put,
    path = "/api/images/{id}",
    request_body = StructUpdateImageReq,
    responses(
        (status = 200, description = "图片更新成功", body = ResponseT<Model>),
        (status = 500, description = "内部服务器错误")
    ),
    tag = "images"
)]
pub async fn update_image(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i16>,
    image_data: web::Json<StructUpdateImageReq>,
) -> impl Responder {
    let image_result = images::Entity::find_by_id(*id).one(db.get_ref()).await;

    match image_result {
        Ok(Some(image)) => {
            let now = Utc::now();
            let format_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let mut updated_image: ActiveModel = image.into();
            updated_image.name = image_data.name.clone().map(Set).unwrap_or(NotSet);
            updated_image.update_time = Set(format_time.clone());
            let result = updated_image.update(db.get_ref()).await;
            if let Ok(_) = result {
                return HttpResponse::Ok().json(response_t(
                    Some(200),
                    Some(String::from("OK")),
                    None,
                ));
            }
            return HttpResponse::NotImplemented().json(format!("Error inserting rule"));
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
