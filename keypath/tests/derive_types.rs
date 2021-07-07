//! test deriving on various struct types

use keypath::Keyable;

#[derive(Keyable)]
struct PlainStruct;

#[derive(Keyable)]
struct EmptyTupleStruct();

#[derive(Keyable)]
struct SingleTupleStruct(bool);

#[derive(Keyable)]
struct MultiTupleStruct(bool, i64, String);

#[derive(Keyable)]
struct EmptyFieldStruct {}

#[derive(Keyable)]
struct SingleFieldStruct {
    a: bool,
}

#[derive(Keyable)]
struct MultiFieldStruct {
    a: bool,
    b: i64,
    c: String,
}
