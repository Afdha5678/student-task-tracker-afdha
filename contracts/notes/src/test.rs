#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_create_task() {

    let env = Env::default();

    let title = String::from_str(&env, "Belajar Soroban");

    let result = TaskContract::create_task(
        env.clone(),
        title,
    );

    assert_eq!(
        result,
        String::from_str(&env, "Task berhasil ditambahkan")
    );
}

#[test]
fn test_get_tasks() {

    let env = Env::default();

    let title = String::from_str(&env, "Belajar Rust");

    TaskContract::create_task(
        env.clone(),
        title,
    );

    let tasks = TaskContract::get_tasks(env.clone());

    assert_eq!(tasks.len(), 1);
}

#[test]
fn test_delete_task() {

    let env = Env::default();

    let title = String::from_str(&env, "Kerjakan tugas");

    TaskContract::create_task(
        env.clone(),
        title,
    );

    let tasks = TaskContract::get_tasks(env.clone());

    let id = tasks.get(0).unwrap().id;

    let result = TaskContract::delete_task(
        env.clone(),
        id,
    );

    assert_eq!(
        result,
        String::from_str(&env, "Task berhasil dihapus")
    );
}