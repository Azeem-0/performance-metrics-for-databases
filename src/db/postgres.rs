use std::env;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};

use crate::{
    models::{depth_history_model::DepthHistory, rune_pool_history_model::RunePoolHistory},
    utils::types::{Error, Result},
};

pub struct PostgreSQL {
    pub pool: Pool<Postgres>,
}

impl PostgreSQL {
    pub async fn init() -> Result<Self> {
        dotenv().ok();

        let uri = match env::var("POSGRES_DATABASE_URL") {
            Ok(v) => v.to_string(),
            Err(_) => {
                return Err(Error::DataBaseConnectionFailed(
                    "Unable to read mongodb uri.".to_string(),
                ))
            }
        };

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&uri)
            .await
            .map_err(|e| e)
            .unwrap();

        let postgres = PostgreSQL { pool };

        postgres.ensure_table_exists().await?;

        Ok(postgres)
    }

    pub async fn ensure_table_exists(&self) -> Result<()> {
        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS depth_history (
            start_time DOUBLE PRECISION NOT NULL,
            end_time DOUBLE PRECISION NOT NULL,
            asset_depth DOUBLE PRECISION NOT NULL,
            rune_depth DOUBLE PRECISION NOT NULL,
            asset_price DOUBLE PRECISION NOT NULL,
            asset_price_usd DOUBLE PRECISION NOT NULL,
            liquidity_units DOUBLE PRECISION NOT NULL,
            members_count DOUBLE PRECISION NOT NULL,
            synth_units DOUBLE PRECISION NOT NULL,
            synth_supply DOUBLE PRECISION NOT NULL,
            units DOUBLE PRECISION NOT NULL,
            luvi DOUBLE PRECISION NOT NULL
        )
        "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::DataBaseCreationFailed(format!("Table creation failed: {}", e)))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS rune_pool_history (
                start_time DOUBLE PRECISION NOT NULL,
                end_time DOUBLE PRECISION NOT NULL,
                count DOUBLE PRECISION NOT NULL,
                units DOUBLE PRECISION NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            Error::DataBaseCreationFailed(format!("Rune pool history table creation failed: {}", e))
        })?;

        Ok(())
    }

    pub async fn insert_depth_history(&self, depth_history: &DepthHistory) -> Result<()> {
        sqlx::query(
            r#"
        INSERT INTO depth_history (
            start_time, end_time, asset_depth, rune_depth, asset_price, asset_price_usd,
            liquidity_units, members_count, synth_units, synth_supply, units, luvi
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        )
        .bind(depth_history.start_time)
        .bind(depth_history.end_time)
        .bind(depth_history.asset_depth)
        .bind(depth_history.rune_depth)
        .bind(depth_history.asset_price)
        .bind(depth_history.asset_price_usd)
        .bind(depth_history.liquidity_units)
        .bind(depth_history.members_count)
        .bind(depth_history.synth_units)
        .bind(depth_history.synth_supply)
        .bind(depth_history.units)
        .bind(depth_history.luvi)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            Error::DataBaseInsertionFailed(format!("Depth history insert failed: {}", e))
        })?;

        Ok(())
    }

    pub async fn read_depth_history(&self) -> Result<Vec<DepthHistory>> {
        let rows = sqlx::query(
            r#"
        SELECT 
            start_time, 
            end_time, 
            asset_depth, 
            rune_depth, 
            asset_price, 
            asset_price_usd,
            liquidity_units, 
            members_count, 
            synth_units, 
            synth_supply, 
            units, 
            luvi
        FROM depth_history
        "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::DataBaseReadFailed(format!("Failed to read depth history: {}", e)))?;
        let records: Vec<DepthHistory> = rows
            .into_iter()
            .map(|row| DepthHistory {
                start_time: row.get("start_time"),
                end_time: row.get("end_time"),
                asset_depth: row.get("asset_depth"),
                rune_depth: row.get("rune_depth"),
                asset_price: row.get("asset_price"),
                asset_price_usd: row.get("asset_price_usd"),
                liquidity_units: row.get("liquidity_units"),
                members_count: row.get("members_count"),
                synth_units: row.get("synth_units"),
                synth_supply: row.get("synth_supply"),
                units: row.get("units"),
                luvi: row.get("luvi"),
            })
            .collect();

        Ok(records)
    }

    pub async fn read_rune_pool_history(&self) -> Result<Vec<RunePoolHistory>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                start_time,
                end_time,
                count,
                units
            FROM rune_pool_history
        "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            Error::DataBaseReadFailed(format!("Failed to read rune pool history: {}", e))
        })?;

        let records: Vec<RunePoolHistory> = rows
            .into_iter()
            .map(|row| RunePoolHistory {
                count: row.get("count"),
                start_time: row.get("start_time"),
                end_time: row.get("end_time"),
                units: row.get("units"),
            })
            .collect();

        Ok(records)
    }

    pub async fn insert_rune_pool_history(&self, pool_history: &RunePoolHistory) -> Result<()> {
        sqlx::query(
            r#"
                INSERT INTO rune_pool_history (
                start_time,end_time,count,units
                )
                VALUES ($1,$2,$3,$4)
            "#,
        )
        .bind(pool_history.start_time)
        .bind(pool_history.end_time)
        .bind(pool_history.count)
        .bind(pool_history.units)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            Error::DataBaseInsertionFailed(format!("Rune pool history insert failed: {}", e))
        })?;

        Ok(())
    }
}
