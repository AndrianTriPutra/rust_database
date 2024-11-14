# rust_database_sea-orm

## getting started
### postgres
> - [1] create db
> - [2] open directory .sql
> - [3] run create.sql at your db
> - [4] run timeSync.sql at your db, refresh and check trigger at your table.
> - [5] run againt point 4 if file nox exist
> - [6] run insert.sql for check function point 4
> - [7] run update.sql for check function point 4
> - [8] run delete.sql for check function point 4

### run with
-> create       : cargo run create
-> read-all     : cargo run readAll
-> read by uuid : cargo run readUUID UUID
-> update       : cargo run update UUID
-> soft delete  : cargo run delete_soft UUID
-> hard delete  : cargo run delete_hard UUID


## documentation
> - [medium]()