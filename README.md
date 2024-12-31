# Performance Metrics of Different Databases

## Overview

This project evaluates and compares the performance of different databases, focusing on the time taken for inserting large data and reading the same data.

### Supported Databases:
- **MongoDB**
- **PostgreSQL**
- **SurrealDB**
- **LevelDB**
- **RocksDB**

The performance of each database is measured in terms of:
- Time taken for data insertion
- Time taken for data retrieval


## API Routes Overview

This project exposes several API routes for interacting with the databases:

### 1. **GET `/fetch-and-insert-data`**
   - **Description**: This route is used to fetch thorchain data from midgard API and insert it into the different databases and measure the performance of each database.

### 2. **GET `/read-data`**
   - **Description**: This route is used to read the stored thorchain data from different databases and measure the performance of each database. 

## Performance Metrics Logging

The performance metrics for database operations (insertion and retrieval times) are logged in the `performance-metrics.txt` file.
