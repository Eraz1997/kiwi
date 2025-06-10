use fred::prelude::{KeysInterface, TransactionInterface};

use crate::managers::redis::{
    RedisManager,
    error::Error,
    models::{RedisAccessToken, RedisItem, RedisRefreshToken},
};

impl RedisManager {
    pub async fn store_auth_tokens(
        &self,
        access_token: &str,
        refresh_token: &str,
        user_id: u32,
    ) -> Result<(), Error> {
        let access_token_item = RedisAccessToken {
            access_token: access_token.to_string(),
            user_id,
        };
        let refresh_token_item = RedisRefreshToken {
            refresh_token: refresh_token.to_string(),
            user_id,
        };

        let transaction = self.client.multi();
        let _: () = transaction
            .set(
                access_token_item.get_key(),
                access_token_item.get_value(),
                access_token_item.get_expiration(),
                None,
                false,
            )
            .await?;
        let _: () = transaction
            .set(
                refresh_token_item.get_key(),
                refresh_token_item.get_value(),
                refresh_token_item.get_expiration(),
                None,
                false,
            )
            .await?;
        let _: () = transaction.exec(true).await?;

        Ok(())
    }
}
