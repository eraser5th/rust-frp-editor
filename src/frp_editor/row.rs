use sodium_rust::{Cell, SodiumCtx, Stream};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
pub struct Row {
    pub c_content: Cell<String>,
    pub c_len: Cell<usize>,
}

impl Row {
    pub fn new_from_str(
        sodium_ctx: &SodiumCtx,
        s: &str,
        s_insert_init: Stream<(usize, char)>,
        s_delete_init: Stream<usize>,
        s_append: Stream<(Row, Stream<(usize, char)>, Stream<usize>)>,
    ) -> Self {
        let cloop_content = sodium_ctx.new_cell_loop();
        let c_content = cloop_content.cell();
        let c_len = c_content.map(|content: &String| content[..].graphemes(true).count());

        let cloops_insert = sodium_ctx.new_cell_loop::<Stream<(usize, char)>>();
        let cs_insert = cloops_insert.cell();
        let s_insert = cs_insert.switch_s();
        cloops_insert.loop_(&s_append.map(|(_, s_i, _)| s_i.clone()).hold(s_insert_init));

        let cloops_delete = sodium_ctx.new_cell_loop::<Stream<usize>>();
        let cs_delete = cloops_delete.cell();
        let s_delete = cs_delete.switch_s();
        cloops_delete.loop_(&s_append.map(|(_, _, s_d)| s_d.clone()).hold(s_delete_init));

        let s_inserted = insert(&s_insert, &c_content, &c_len);
        let s_deleted = delete(&s_delete, &c_content, &c_len);

        cloop_content.loop_(&s_inserted.or_else(&s_deleted).hold(s.to_string()));

        let cloopc_content = sodium_ctx.new_cell_loop::<Cell<String>>();
        let cc_content = cloopc_content.cell();

        cloopc_content.loop_(
            &s_append
                .map(|(r, _, _)| r.clone())
                .snapshot(&cc_content, |r, c_content| {
                    c_content.lift2(&r.c_content, |a, b| a.clone() + b)
                })
                .hold(c_content),
        );
        let c_content = Cell::switch_c(&cc_content);

        Self { c_content, c_len }
    }
}

impl Row {
    pub fn slice(&self, c_start: Cell<usize>, c_end: Cell<usize>) -> Cell<String> {
        let end = c_end.lift2(&self.c_len, |e, len| e.min(len).clone());
        let start = c_start.lift2(&end, |s, e| s.min(e).clone());

        self.c_content.lift3(&start, &end, |content, start, end| {
            content[..]
                .graphemes(true)
                .skip(start.clone())
                .take(end - start)
                .map(|g| if g == "\t" { "  " } else { g })
                .collect()
        })
    }

    pub fn is_empty(&self) -> Cell<bool> {
        self.c_len.map(|len| len == &0)
    }
}

fn insert(
    s_insert: &Stream<(usize, char)>,
    c_content: &Cell<String>,
    c_len: &Cell<usize>,
) -> Stream<String> {
    s_insert.snapshot3(c_content, c_len, |(at, c), content, len| {
        let mut content = content.clone();
        let len = len.clone();
        let at = at.clone();
        let c = c.clone();

        if at >= len {
            content.push(c);
            content
        } else {
            let mut result: String = content[..].graphemes(true).take(at).collect();
            let remainder: String = content[..].graphemes(true).skip(at).collect();
            result.push(c);
            result + &remainder
        }
    })
}

fn delete(
    s_delete: &Stream<usize>,
    c_content: &Cell<String>,
    c_len: &Cell<usize>,
) -> Stream<String> {
    s_delete.snapshot3(c_content, c_len, |at, content, len| {
        let content = content.clone();
        let len = len.clone();
        let at = at.clone();

        if at >= len {
            content
        } else {
            let result: String = content[..].graphemes(true).take(at).collect();
            let remainder: String = content[..].graphemes(true).skip(at + 1).collect();
            result + &remainder
        }
    })
}
