use user_management::{
    common::types::{SortDirection, SortProperty},
    user::{user_model::UserData, user_service::UserService},
};

use crate::{
    common_test::test_util::USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT,
    user::user_test_util::save_default_user,
};

use super::user_test_util::save_user;

#[actix_rt::test]
async fn should_find_all_users() {
    // given
    let user_service = UserService::new(USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT.pool.clone());
    let user = save_user("first", "last", "zzemail@test.com").await;
    let sort_properties = vec![SortProperty::new("email", SortDirection::Desc)];

    // when
    let result = user_service
        .find_all(0, 10, Some(&sort_properties))
        .await
        .unwrap();

    // then
    assert!(result.total_elements > 0);
    assert_eq!(result.content.first().unwrap().email, user.email);

    // and when
    let sort_properties = vec![SortProperty::new("email", SortDirection::Asc)];

    let result = user_service
        .find_all(0, 10, Some(&sort_properties))
        .await
        .unwrap();

    // then
    assert_eq!(result.content.last().unwrap().email, user.email);
}

#[actix_rt::test]
async fn should_not_fail_sorting_by_any_property() {
    let user_service = UserService::new(USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT.pool.clone());
    let sort_properties = vec![
        SortProperty::new("id", SortDirection::Desc),
        SortProperty::new("first_name", SortDirection::Asc),
        SortProperty::new("last_name", SortDirection::Desc),
        SortProperty::new("email", SortDirection::Asc),
    ];

    // when
    let result = user_service
        .find_all(0, 10, Some(&sort_properties))
        .await
        .unwrap();

    // then
    assert!(result.total_elements > 0);
}

#[actix_rt::test]
async fn should_find_user() {
    // given
    let user_service = UserService::new(USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT.pool.clone());
    let user = save_default_user().await;

    // when
    let result = user_service.find_by_id(user.id).await.unwrap();

    // then
    assert_eq!(result.id, user.id);
}

#[actix_rt::test]
async fn should_save_user() {
    // given
    let user = UserData::new("new user name", "new user last", "new_user_email@test.com");
    let user_service = UserService::new(USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT.pool.clone());

    // when
    let result = user_service.save(&user).await.unwrap();

    // then
    assert_eq!(result.first_name, user.first_name);
    assert_eq!(result.last_name, user.last_name);
    assert_eq!(result.email, user.email);
}

#[actix_rt::test]
async fn should_update_user() {
    // given
    let user_service = UserService::new(USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT.pool.clone());
    let user = save_default_user().await;
    let user_data = UserData::new("updated user name", "updated user last", "updated@test.com");

    // when
    let result = user_service.update(user.id, &user_data).await.unwrap();

    // then
    assert_eq!(result.first_name, user_data.first_name);
    assert_eq!(result.last_name, user_data.last_name);
    assert_eq!(result.email, user_data.email);
}

#[actix_rt::test]
async fn should_delete_user() {
    // given
    let user_service = UserService::new(USER_MANAGEMENT_TEST_ENVIRONMENT_CONTEXT.pool.clone());
    let user = save_default_user().await;

    // when
    let result = user_service.delete(user.id).await.unwrap();

    // then
    assert_eq!(result, 1);
}
