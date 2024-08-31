use std::{env, str::FromStr};

use super::errors::{RgiombiniError, RgiombiniResult};

#[derive(Debug, Clone)]
pub struct Config {
    pub db_uri: String,
    pub db_uri_without_dbname: String,
    pub db_name: String,
    pub port: u16,
    pub workers: u16,
    pub superuser_email: String,
    pub superuser_pass: String,

    pub access_token_expire_in_secs: u32,
    pub refresh_token_expire_in_days: u16,
    pub secret_key: String,
}

impl Config {
    pub fn new() -> RgiombiniResult<Self> {
        let db_user: String = Self::get_env_var("DATABASE_USER")?;
        let db_password: String = Self::get_env_var("DATABASE_PASSWORD")?;
        let db_name: String = Self::get_env_var("DATABASE_NAME")?;
        let db_host: String = Self::get_env_var("DATABASE_HOST")?;
        let db_port: String = Self::get_env_var("DATABASE_PORT")?;
        let db_uri =
            { format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}") };
        let db_uri_without_dbname =
            { format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}") };
        let port = Self::get_env_var("PORT")?;
        let workers = Self::get_env_var("WORKERS")?;
        let superuser_email = Self::get_env_var("SUPERUSER_EMAIL")?;
        let superuser_pass = Self::get_env_var("SUPERUSER_PASS")?;
        let access_token_expire_in_secs = Self::get_env_var("ACCESS_TOKEN_EXPIRE_IN_SECS")?;
        let refresh_token_expire_in_days = Self::get_env_var("REFRESH_TOKEN_EXPIRE_IN_DAYS")?;
        let secret_key = Self::get_env_var("SECRET_KEY")?;

        Ok(Self {
            db_uri,
            db_uri_without_dbname,
            db_name,
            port,
            workers,
            superuser_email,
            superuser_pass,
            access_token_expire_in_secs,
            refresh_token_expire_in_days,
            secret_key,
        })
    }

    #[inline]
    fn get_env_var<T: FromStr>(env_var: &str) -> RgiombiniResult<T> {
        env::var(env_var)
            .map_err(|_| RgiombiniError::EnvConfigLoadingError(env_var.to_owned()))?
            .parse::<T>()
            .map_err(|_| RgiombiniError::EnvVarParsingError(env_var.to_owned()))
    }

    #[inline]
    fn get_env_var_with_default<T: FromStr>(env_var: &str, default: T) -> RgiombiniResult<T> {
        let result = Self::get_env_var(env_var);

        if matches!(result, Err(RgiombiniError::EnvConfigLoadingError(_))) {
            return Ok(default);
        }

        result
    }
}
