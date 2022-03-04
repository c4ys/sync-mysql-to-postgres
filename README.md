# sync-mysql-to-postgres
Sync mysql database records to postgresql

```toml
from = "mysql://root:root@127.0.0.1/test?chaset=utf8mb4"
to = "postgres://postgres:postgres@127.0.0.1/test"
tables = [
    "table1",
    "table2",
]
limit = 1000
sleep = 1000
```
