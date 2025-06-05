use ibc_utils::bytes::ByteArrayIntoArrayU8;
use serde_json::{Serialize, SerializerTrait, to_array_u8};

#[derive(Drop)]
struct User {
    pub name: ByteArray,
    pub age: u8,
    pub email: Option<ByteArray>,
    pub permission: Permission,
    pub is_active: bool,
    pub metadata: felt252,
}

#[generate_trait]
pub impl UserImpl of UserTrait {
    fn new(
        name: ByteArray,
        age: u8,
        email: Option<ByteArray>,
        permission: Permission,
        is_active: bool,
        metadata: felt252,
    ) -> User {
        User { name, age, email, permission, is_active, metadata }
    }
}

#[derive(Drop)]
pub enum Permission {
    Admin: u8,
    Guest,
}

pub impl PermissionSerialize of Serialize<Permission> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: Permission, ref serializer: S) {
        match self {
            Permission::Admin(level) => {
                serializer.serialize_variant("admin", level);
                serializer.end();
            },
            Permission::Guest => { serializer.serialize_string("guest"); },
        }
    }
}

#[derive(Drop)]
pub enum AccountStatus {
    Active: User,
    InActive,
}

pub impl UserSerialize of Serialize<User> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: User, ref serializer: S) {
        serializer.serialize_field("name", self.name);
        serializer.serialize_field("age", self.age);
        serializer.serialize_field("email", self.email);
        serializer.serialize_field("permission", self.permission);
        serializer.serialize_field("is_active", self.is_active);
        serializer.serialize_field("metadata", self.metadata);
        serializer.end();
    }
}

pub impl AccountStatusSerialize of Serialize<AccountStatus> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: AccountStatus, ref serializer: S) {
        match self {
            AccountStatus::Active(user) => {
                serializer.serialize_variant("active", user);
                serializer.end();
            },
            AccountStatus::InActive => { serializer.serialize_string("inactive"); },
        }
    }
}

pub fn DUMMY_USER() -> User {
    UserImpl::new(
        "john doe", 25, Option::Some("john.doe@example.com"), Permission::Admin(0), true, 100,
    )
}

#[test]
fn test_serialize_struct_ok() {
    let value = to_array_u8(DUMMY_USER());
    let expected =
        "{\"name\":\"john doe\",\"age\":25,\"email\":\"john.doe@example.com\",\"permission\":{\"admin\":0},\"is_active\":true,\"metadata\":100}";
    assert_eq!(value, ByteArrayIntoArrayU8::into(expected));
}

#[test]
fn test_serialize_struct_zero_values() {
    let value = to_array_u8(UserImpl::new("", 0, Option::None, Permission::Guest, false, 0));
    let expected =
        "{\"name\":\"\",\"age\":0,\"email\":null,\"permission\":\"guest\",\"is_active\":false,\"metadata\":0}";
    assert_eq!(value, ByteArrayIntoArrayU8::into(expected));
}

#[test]
fn test_serialize_enum_ok() {
    let enum_active = AccountStatus::Active(DUMMY_USER());

    let value = to_array_u8(enum_active);

    let expected =
        "{\"active\":{\"name\":\"john doe\",\"age\":25,\"email\":\"john.doe@example.com\",\"permission\":{\"admin\":0},\"is_active\":true,\"metadata\":100}}";

    assert_eq!(value, ByteArrayIntoArrayU8::into(expected));

    let enum_inactive = AccountStatus::InActive;

    let value = to_array_u8(enum_inactive);

    let expected = "\"inactive\"";

    assert_eq!(value, ByteArrayIntoArrayU8::into(expected));
}
