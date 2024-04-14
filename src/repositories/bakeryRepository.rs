use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};

use crate::entities::bakery;
use crate::entities::prelude::Bakery;

pub struct BakeryRepositoryImpl;

impl BakeryRepository for BakeryRepositoryImpl {
    fn get_bakery(&self, id: i32) -> Result<Bakery, DbErr> {
        unimplemented!()
    }

    fn get_bakeries(&self) -> Result<Vec<Bakery>, DbErr> {
        unimplemented!()
    }

     async fn create_bakery(&self, db: &DatabaseConnection) -> Result<i32, DbErr> {
        let happy_bakery = bakery::ActiveModel {
            name: ActiveValue::Set("Happy Bakery".to_owned()),
            profit_margin: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let result = Bakery::insert(happy_bakery).exec(db).await;
        match result {
            Ok(result) => {
                Ok(result.last_insert_id as i32)
            },
            Err(err) => Err(DbErr::from(err))
        }
    }

    fn update_bakery(&self, bakery: Bakery) -> Result<Bakery, DbErr> {
        unimplemented!()
    }

    fn delete_bakery(&self, id: i32) -> Result<usize, DbErr> {
        unimplemented!()
    }
}

pub trait BakeryRepository {
    fn get_bakery(&self, id: i32) -> Result<Bakery, DbErr>;
    fn get_bakeries(&self) -> Result<Vec<Bakery>, DbErr>;
    async fn create_bakery(&self, db: &DatabaseConnection) -> Result<i32, DbErr>;
    fn update_bakery(&self, bakery: Bakery) -> Result<Bakery, DbErr>;
    fn delete_bakery(&self, id: i32) -> Result<usize, DbErr>;
}