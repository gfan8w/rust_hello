use anyhow::{anyhow,Result};
use polars::prelude::*;
use sqlparser::ast::{Expr as SqlExpr, Statement,SetExpr,Select,Offset as SqlOffset,
                     Value as SqlValue};

pub struct Sql<'a> {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<u64>,
    pub(crate) limit: Option<usize>,
}

/*impl<'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(sql: &'a Statement) ->Result<Self>  {
        match sql {
            Statement::Query(q) =>{
                let offset =q.offset.as_ref();
                let limit =q.limit.as_ref();

                let Select{
                    from: table_with_joins,
                    selection: where_clause,
                    group_by:_,
                    ..
                }=match &q.body {
                    SetExpr::Select(statement) => statement.as_ref(),
                    _ => Err(anyhow!("we only support select")),
                };

            },
            _ =>Err(anyhow!("we only support query"))
        }


    }
}*/

// 因为 Rust trait 的孤儿规则，我们如果要想对已有的类型实现已有的 trait，
// 需要简单包装一下
pub struct Offset<'a>(pub(crate) &'a SqlOffset);



/// 把 SqlParser 的 offset expr 转换成 i64
impl<'a> From<Offset<'a>> for i64 {
    fn from(offset: Offset<'a>) -> Self {
        match offset.0 {
            SqlOffset{
                value: SqlExpr::Value(SqlValue::Number(v,_b)),
                ..
            } => v.parse::<i64>().unwrap_or(0),
            _ => 0,
        }
    }
}
