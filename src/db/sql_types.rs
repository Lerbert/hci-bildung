use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug, Eq, Hash, PartialEq)]
#[PgType = "role"]
#[DieselType = "Role"]
pub enum RoleDb {
    Teacher,
    Student,
}
