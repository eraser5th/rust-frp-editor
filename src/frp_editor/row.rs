use std::sync::Arc;

use sodium_rust::{Cell, SodiumCtx, Stream};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
pub struct Row {
    pub c_content: Cell<String>,
    pub c_len: Cell<usize>,
}

impl Row {
    pub fn new_from_str(
        sodium_ctx: Arc<SodiumCtx>,
        s: &str,
        s_insert: Stream<(usize, char)>,
        s_delete: Stream<usize>,
    ) -> Self {
        let cloopc_content = sodium_ctx.new_cell_loop();
        let c_content = cloopc_content.cell();
        let c_len = c_content.map(|content: &String| content[..].graphemes(true).count());

        let s_inserted = insert(&s_insert, &c_content, &c_len);
        let s_deleted = delete(&s_delete, &c_content, &c_len);

        let c_content = s_inserted.or_else(&s_deleted).hold(s.to_string());

        Self { c_content, c_len }
    }
}

impl Row {
    pub fn append(
        sodium_ctx: Arc<SodiumCtx>,
        a: &Self,
        b: &Self,
        s_insert: Stream<(usize, char)>,
        s_delete: Stream<usize>,
    ) -> Self {
        let content = a
            .c_content
            .lift2(&b.c_content, |a, b| a.clone() + b)
            .sample();

        let cloopc_content = sodium_ctx.new_cell_loop();
        let c_content = cloopc_content.cell();
        let c_len = c_content.map(|content: &String| content[..].graphemes(true).count());

        let s_inserted = insert(&s_insert, &c_content, &c_len);
        let s_deleted = delete(&s_delete, &c_content, &c_len);

        let c_content = s_inserted.or_else(&s_deleted).hold(content);

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
