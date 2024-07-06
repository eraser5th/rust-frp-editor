use std::sync::Arc;

use sodium_rust::{Cell, SodiumCtx, Stream};
use sodium_rust_more_primitives::util::CellSequenceVec;

use super::{Position, Row};

pub struct Document {
    c_rows: Cell<Vec<Row>>,
    pub c_content: Cell<String>,
    pub filename: String,
    pub c_dirty: Cell<bool>,
}

impl Document {
    pub fn new(
        sodium_ctx: Arc<SodiumCtx>,
        filename: &str,
        s_return: &Stream<Position>,
        s_delete: &Stream<Position>,
        s_insert: &Stream<(Position, char)>,
    ) -> Self {
        let c_rows = c_rows(
            sodium_ctx.clone(),
            s_return,
            s_delete.clone(),
            s_insert.clone(),
        );

        let c_content = c_content(sodium_ctx.clone(), &c_rows);
        let cloop_dirty = sodium_ctx.new_cell_loop();
        let c_dirty = cloop_dirty.cell();

        Self {
            c_rows,
            c_content,
            filename: filename.to_string(),
            c_dirty,
        }
    }
}

fn c_rows(
    sodium_ctx: Arc<SodiumCtx>,
    s_return: &Stream<Position>,
    s_delete: Stream<Position>,
    s_insert: Stream<(Position, char)>,
) -> Cell<Vec<Row>> {
    let cloopc_rows = sodium_ctx.new_cell_loop();
    let cc_rows = cloopc_rows.cell();
    let c_rows: Cell<Vec<Row>> = cc_rows.switch_c();

    let _c_rows_init = sodium_ctx.new_cell(vec![Row::new_from_str(
        sodium_ctx.clone(),
        "",
        s_insert
            .filter(|(p, _)| p.y == 0)
            .map(|(p, c)| (p.x.clone(), c.clone())),
        s_delete.filter(|p| p.y == 0).map(|p| p.x.clone()),
    )]);

    let sodium_ctx_ = sodium_ctx.clone();
    let _s_new_row = s_return.snapshot3(
        &s_delete.hold(Position::default()),
        &s_insert.hold((Position::default(), 'a')),
        move |p_return, p_delete, (p_insert, c_insert)| {
            let p_return_ = p_return.clone();
            let p_return = p_return.clone();

            let s_delete = s_delete
                .clone()
                .filter(move |p_delete| p_delete.y == p_return_.y + 1)
                .map(|p| p.x.clone());
            let s_insert = s_insert
                .clone()
                .filter(move |(p_insert, _)| p_insert.y == p_return.y + 1)
                .map(|(p, c)| (p.x.clone(), c.clone()));

            Row::new_from_str(sodium_ctx_.clone(), "", s_insert, s_delete)
        },
    );

    c_rows
}

fn c_content(sodium_ctx: Arc<SodiumCtx>, c_rows: &Cell<Vec<Row>>) -> Cell<String> {
    let cc_content = {
        let c_init = sodium_ctx.new_cell(vec![]);
        c_rows.map(move |rows| {
            rows.iter()
                .map(|row: &Row| row.c_content.clone())
                .collect::<Vec<Cell<String>>>()
                .sequence(c_init.clone())
                .map(|contents| contents.iter().fold("".to_string(), |acc, c| acc + c))
        })
    };

    cc_content.switch_c()
}
