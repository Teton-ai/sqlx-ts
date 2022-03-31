<h1 align="center">SQLx-ts</h1>
<div align="center">
 <strong>
   🧰 The Typescript/Javascript SQL Toolkit
 </strong>
</div>

<br />

<div align="center">
Built to free Node developers from ORMs
</div>

SQLx-ts is a CLI application featuring compile-time checked queries without a DSL and prevents broken SQL queries being run during runtime.

- **Compile time checked queries** - never ship a broken SQL query to production

### ROADMAP

##### Phase 1

- [x] swc to parse JavaScript/TypeScript
- [x] pick all sql`` tagged template literals
- [x] compilation success if no errors detected in SQLs
- [x] compilation failure if any error is detected in SQLs

##### Phase 2

- [ ] CLI integration - supporting parameters for folder source, database credentials and etc
- [ ] Add more complex Typescript and Javascript examples in `samples` folder

##### Phase 3

- [ ] Support for MySQL, SQLite and possibly mssql
- [ ] Run sqlx-tx-core multi-threaded and check multiple files at once

##### Phase 4

- [ ] Publish an NPM module with `sql` tagged template literal. It shouldn't do anything special but return the raw SQL back again
