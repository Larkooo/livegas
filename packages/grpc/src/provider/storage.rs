use std::sync::Arc;

use sqlx::{sqlite::SqliteRow, SqlitePool};

use crate::{
    error::Error,
    pb::{BlockUpdate, Network},
};

pub struct ProviderStorage {
    database: Arc<SqlitePool>,
}

impl ProviderStorage {
    pub fn new(database: Arc<SqlitePool>) -> Self {
        Self { database }
    }

    /// Inserts a block into the database.
    /// - `network`: The network identifier (e.g., "ETH_MAINNET").
    /// - `block_number`: The block number.
    /// - `block_hash`: The block hash.
    /// - `block_timestamp`: The block timestamp.
    /// - `gas_fee`: The gas fee.
    /// Returns the result of the query.
    pub async fn insert_block(
        &self,
        network: &str,
        block_number: i64,
        block_hash: &str,
        block_timestamp: i64,
        gas_fee: f64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, Error> {
        let mut conn = self.database.acquire().await.unwrap();

        sqlx::query!(
            r#"
            INSERT INTO blocks (network, block_number, block_hash, block_timestamp, gas_fee)
            VALUES (?, ?, ?, ?, ?)
            "#,
            network,
            block_number,
            block_hash,
            block_timestamp,
            gas_fee
        )
        .execute(conn.as_mut())
        .await
        .map_err(Error::Database)
    }

    /// Reads blocks from the database.
    /// - `network`: The network identifier (e.g., "ETH_MAINNET").
    /// - `start`: The starting block number or timestamp.
    /// - `end`: The ending block number or timestamp.
    /// - `by_timestamp`: Whether the range is based on timestamps (true) or block numbers (false).
    /// Returns the rows.
    pub async fn read_blocks(
        &self,
        network: &str,
        start: i64,
        end: i64,
        by_timestamp: bool,
    ) -> Result<Vec<BlockUpdate>, Error> {
        let mut conn = self.database.acquire().await?;

        if by_timestamp {
            let rows = sqlx::query!(
                r#"
                SELECT network, block_number, block_hash, block_timestamp, gas_fee
                FROM blocks
                WHERE network = ? AND block_timestamp >= ? AND block_timestamp <= ?
                ORDER BY block_timestamp ASC
                "#,
                network,
                start,
                end
            )
            .fetch_all(conn.as_mut())
            .await?;

            let blocks = rows
                .into_iter()
                .map(|row| BlockUpdate {
                    network: Network::from_str_name(&row.network).unwrap().into(),
                    block_number: row.block_number as u64,
                    block_hash: row.block_hash,
                    timestamp: row.block_timestamp as u64,
                    gas_fee: row.gas_fee,
                })
                .collect();

            Ok(blocks)
        } else {
            let rows = sqlx::query!(
                r#"
                SELECT network, block_number, block_hash, block_timestamp, gas_fee
                FROM blocks
                WHERE network = ? AND block_number >= ? AND block_number <= ?
                ORDER BY block_number ASC
                "#,
                network,
                start,
                end
            )
            .fetch_all(conn.as_mut())
            .await?;

            let blocks = rows
                .into_iter()
                .map(|row| BlockUpdate {
                    network: Network::from_str_name(&row.network).unwrap().into(),
                    block_number: row.block_number as u64,
                    block_hash: row.block_hash,
                    timestamp: row.block_timestamp as u64,
                    gas_fee: row.gas_fee,
                })
                .collect();

            Ok(blocks)
        }
    }

    /// Reads the latest block from the database.
    /// - `network`: The network identifier (e.g., "ETH_MAINNET").
    /// Returns the row.
    pub async fn read_latest_block(&self, network: &str) -> Result<Option<BlockUpdate>, Error> {
        let mut conn = self.database.acquire().await?;

        let row = sqlx::query!(
            r#"
            SELECT network, block_number, block_hash, block_timestamp, gas_fee
            FROM blocks
            WHERE network = ?
            ORDER BY block_number DESC
            LIMIT 1
            "#,
            network
        )
        .fetch_optional(conn.as_mut())
        .await?;

        match row {
            Some(row) => Ok(Some(BlockUpdate {
                network: Network::from_str_name(&row.network).unwrap().into(),
                block_number: row.block_number as u64,
                block_hash: row.block_hash,
                timestamp: row.block_timestamp as u64,
                gas_fee: row.gas_fee,
            })),
            None => Ok(None),
        }
    }

    /// Read a block from the database.
    /// - `network`: The network identifier (e.g., "ETH_MAINNET").
    /// - `block_number`: The block number.
    /// Returns the row.
    pub async fn read_block(
        &self,
        network: &str,
        block_number: i64,
    ) -> Result<Option<BlockUpdate>, Error> {
        let mut conn = self.database.acquire().await?;

        let row = sqlx::query!(
            r#"
            SELECT network, block_number, block_hash, block_timestamp, gas_fee
            FROM blocks
            WHERE network = ? AND block_number = ?
            "#,
            network,
            block_number
        )
        .fetch_optional(conn.as_mut())
        .await?;

        match row {
            Some(row) => Ok(Some(BlockUpdate {
                network: Network::from_str_name(&row.network).unwrap().into(),
                block_number: row.block_number as u64,
                block_hash: row.block_hash,
                timestamp: row.block_timestamp as u64,
                gas_fee: row.gas_fee,
            })),
            None => Ok(None),
        }
    }
}
