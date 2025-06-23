use fred::prelude::{KeysInterface, TransactionInterface};

use crate::managers::redis::{
    RedisManager,
    error::Error,
    models::{
        RedisAccessToken, RedisActiveRefreshToken, RedisItem, RedisRefreshToken,
        RedisRefreshTokenKind, RedisRefreshedRefreshToken,
    },
};

impl RedisManager {
    pub async fn store_active_auth_tokens(
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
            kind: RedisRefreshTokenKind::Active(RedisActiveRefreshToken { user_id }),
        };

        let transaction = self.client.multi();
        let _: () = transaction
            .set(
                access_token_item.to_redis_key(),
                access_token_item.to_redis_value(),
                access_token_item.get_expiration(),
                None,
                false,
            )
            .await?;
        let _: () = transaction
            .set(
                refresh_token_item.to_redis_key(),
                refresh_token_item.to_redis_value(),
                refresh_token_item.get_expiration(),
                None,
                false,
            )
            .await?;
        let _: () = transaction.exec(true).await?;

        Ok(())
    }

    pub async fn get_access_token_item(
        &self,
        access_token: &str,
    ) -> Result<Option<RedisAccessToken>, Error> {
        let key = RedisAccessToken {
            access_token: access_token.to_string(),
            user_id: 0,
        }
        .to_redis_key();
        let value: Option<String> = self.client.get(key.clone()).await?;

        let access_token_item = if let Some(value) = value {
            Some(RedisAccessToken::from_redis_item(key, value)?)
        } else {
            None
        };
        Ok(access_token_item)
    }

    pub async fn get_refresh_token_item(
        &self,
        refresh_token: &str,
    ) -> Result<Option<RedisRefreshToken>, Error> {
        let key = RedisRefreshToken {
            refresh_token: refresh_token.to_string(),
            kind: RedisRefreshTokenKind::Active(RedisActiveRefreshToken { user_id: 0 }),
        }
        .to_redis_key();
        let value: Option<String> = self.client.get(key.clone()).await?;

        let refresh_token_item = if let Some(value) = value {
            Some(RedisRefreshToken::from_redis_item(key, value)?)
        } else {
            None
        };
        Ok(refresh_token_item)
    }
    pub async fn store_refreshed_auth_tokens(
        &self,
        old_refresh_token: &str,
        fresh_access_token: &str,
        fresh_refresh_token: &str,
        user_id: u32,
    ) -> Result<(), Error> {
        let refreshed_refresh_token_item = RedisRefreshToken {
            refresh_token: old_refresh_token.to_string(),
            kind: RedisRefreshTokenKind::Refreshed(RedisRefreshedRefreshToken {
                fresh_access_token: fresh_access_token.to_string(),
                fresh_refresh_token: fresh_refresh_token.to_string(),
            }),
        };
        let access_token_item = RedisAccessToken {
            access_token: fresh_access_token.to_string(),
            user_id,
        };
        let refresh_token_item = RedisRefreshToken {
            refresh_token: fresh_refresh_token.to_string(),
            kind: RedisRefreshTokenKind::Active(RedisActiveRefreshToken { user_id }),
        };

        let transaction = self.client.multi();
        let _: () = transaction
            .set(
                refreshed_refresh_token_item.to_redis_key(),
                refreshed_refresh_token_item.to_redis_value(),
                refreshed_refresh_token_item.get_expiration(),
                None,
                false,
            )
            .await?;
        let _: () = transaction
            .set(
                access_token_item.to_redis_key(),
                access_token_item.to_redis_value(),
                access_token_item.get_expiration(),
                None,
                false,
            )
            .await?;
        let _: () = transaction
            .set(
                refresh_token_item.to_redis_key(),
                refresh_token_item.to_redis_value(),
                refresh_token_item.get_expiration(),
                None,
                false,
            )
            .await?;
        let _: () = transaction.exec(true).await?;

        Ok(())
    }
}
