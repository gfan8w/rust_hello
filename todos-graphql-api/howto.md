- 创建`.env` 文件，里面内容是：
  `DATABASE_URL=postgres://postgres:postgres@localhost/graphql_todos_example`
- 运行`diesel setup`，这会创建db
- 运行`diesel migration generate create_todos`，会有`up.sql` 和 `down.sql` 2个文件
- 在文件中填入内容
- 运行`diesel migration run`，然后查看db里是否有数据。
- 运行 `diesel migration redo`，看db里的内容是否有变化