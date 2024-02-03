CREATE TABLE IF NOT EXISTS blocks (
    network TEXT NOT NULL,
    block_number INTEGER NOT NULL,
    block_hash TEXT NOT NULL,
    gas_fee REAL NOT NULL,
    PRIMARY KEY (network, block_number)
);