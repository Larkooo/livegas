CREATE TABLE IF NOT EXISTS blocks (
    network INTEGER NOT NULL,
    blockNumber INTEGER NOT NULL,
    blockHash TEXT NOT NULL,
    gasFee REAL NOT NULL,
    PRIMARY KEY (network, blockNumber)
);