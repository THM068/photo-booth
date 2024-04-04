use sea_orm::{DatabaseConnection, DbErr};
use crate::entities::prelude::Bakery;

pub struct BakeryRepositoryImpl<'a> {
    db: &'a DatabaseConnection
}

impl BakeryRepository for BakeryRepositoryImpl {
    fn get_bakery(&self, id: i32) -> Result<Bakery, DbErr> {
        unimplemented!()
    }

    fn get_bakeries(&self) -> Result<Vec<Bakery>, DbErr> {
        unimplemented!()
    }

    fn create_bakery(&self, bakery: Bakery) -> Result<Bakery, DbErr> {
        unimplemented!()
    }

    fn update_bakery(&self, bakery: Bakery) -> Result<Bakery, DbErr> {
        unimplemented!()
    }

    fn delete_bakery(&self, id: i32) -> Result<usize, DbErr> {
        unimplemented!()
    }
}
trait BakeryRepository {
    fn get_bakery(&self, id: i32) -> Result<Bakery, DbErr>;
    fn get_bakeries(&self) -> Result<Vec<Bakery>, DbErr>;
    fn create_bakery(&self, bakery: Bakery) -> Result<Bakery, DbErr>;
    fn update_bakery(&self, bakery: Bakery) -> Result<Bakery, DbErr>;
    fn delete_bakery(&self, id: i32) -> Result<usize, DbErr>;
}