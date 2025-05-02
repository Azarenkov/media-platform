use actix_web::rt::task;
use anyhow::{Context, anyhow};
use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{self, SaltString, rand_core::OsRng},
};
use argon2::{PasswordHash, PasswordHasher};

use crate::domain::hasher_abstract::HasherAbstract;

pub struct ArgonHasher;

impl HasherAbstract for ArgonHasher {
    async fn hash(&self, password: String) -> anyhow::Result<String> {
        task::spawn_blocking(move || {
            let salt = SaltString::generate(OsRng);
            Ok(Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| anyhow!(e).context("failed to hash password"))?
                .to_string())
        })
        .await
        .context("panic in hash()")?
        // .context("panic in hash()")?
    }

    async fn verify(&self, password: String, hash: String) -> anyhow::Result<bool> {
        task::spawn_blocking(move || {
            let hash = PasswordHash::new(&hash)
                .map_err(|e| anyhow!(e).context("BUG: password hash invalid"))?;

            let res = Argon2::default().verify_password(password.as_bytes(), &hash);

            match res {
                Ok(()) => Ok(true),
                Err(password_hash::Error::Password) => Ok(false),
                Err(e) => Err(anyhow!(e).context("failed to verify password")),
            }
        })
        .await
        .context("panic in verify()")?
    }
}
