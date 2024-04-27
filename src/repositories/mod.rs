use rocket::FromForm;
use sea_orm::{DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};

mod ContactRepository;
pub mod bakeryRepository;

trait Repository<T> {
    async fn findAll(&self, db: &DatabaseConnection) -> Result<Vec<T>, DbErr>;
    async fn save(&self, db: &DatabaseConnection) -> Result<i32, DbErr>;

    async fn count(&self, db: &DatabaseConnection) -> Result<i64, DbErr>;
    // fn findById(&self, id: i32) -> Result<Option<T>, DbErr>;
    //
    // fn update(&self, entity: T) -> Result<T, DbErr>;
    // fn delete(&self, id: i32, db: &DatabaseConnection) -> Result<usize, DbErr>;
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Contact_Entity {
    pub id: i32,
    pub given_name: String,
    pub family_name: String,
    pub phone: String,
    pub email: String,
}
#[derive(FromForm, Debug, PartialEq)]
pub struct Contact_DTO {
    #[field(validate=len(1..))]
    pub given_name: String,
    #[field(validate=len(1..))]
    pub family_name: String,
    #[field(validate=len(1..))]
    pub phone: String,
    #[field(validate=len(1..))]
    pub email: String,
}

impl From<Contact_DTO> for Contact_Entity {
    fn from(dto: Contact_DTO) -> Self {
        Contact_Entity {
            id: 0,
            given_name: dto.given_name,
            family_name: dto.family_name,
            phone: dto.phone,
            email: dto.email,
        }
    }
}
