use chrono::Utc;
use socal::tui::{
    model::{CurrentState, Message, Model},
    update::update,
};
use sqlx::SqlitePool;

use crate::create_default_model;

#[tokio::test]
async fn can_navigate_account_list() {
    fn assert_selected_index(expected_index: usize, model: &Model) {
        let CurrentState::ManageConnections(ref page_state) = model.current_state else {
            panic!("model not in expected state")
        };

        assert_eq!(page_state.selected_account_index, expected_index);
    }

    // Arrange
    let mut model = create_default_model().await;
    seed_account("test@test.com".into(), &model.application.db).await;
    seed_account("dave@dave.com".into(), &model.application.db).await;

    let _ = update(&mut model, Message::ManageAccounts)
        .await
        .unwrap();

    // Act/Assert
    assert_selected_index(0, &model);

    let _ = update(&mut model, Message::Down).await.unwrap();
    assert_selected_index(1, &model);

    let _ = update(&mut model, Message::Down).await.unwrap();
    assert_selected_index(0, &model);

    let _ = update(&mut model, Message::Up).await.unwrap();
    assert_selected_index(1, &model);

    let _ = update(&mut model, Message::Up).await.unwrap();
    assert_selected_index(0, &model);
}

async fn seed_account(email: String, db: &SqlitePool) -> i64 {
    let access_token = "blah";
    let refresh_token = "bloh";
    let expires_at = Utc::now().to_rfc3339();

    let row = sqlx::query!("INSERT INTO accounts (email, access_token, refresh_token, expires_at) VALUES ($1, $2, $3, $4) RETURNING id",
        email,
        access_token,
        refresh_token, 
        expires_at
    )
        .fetch_one(db)
        .await
        .expect("failed to seed account");

    row.id
}
