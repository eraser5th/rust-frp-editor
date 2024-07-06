use std::{
    fs::{File, OpenOptions},
    io::Read,
    sync::Arc,
};

use sodium_rust::{Cell, SodiumCtx, Stream};

pub struct Document {
    pub c_content: Cell<String>,
    pub c_filename: Cell<Option<String>>,
}

impl Document {
    pub fn new(sodium_ctx: Arc<SodiumCtx>, s_filename: &Stream<String>) -> Self {
        let cloop_content = sodium_ctx.new_cell_loop();
        let c_content = cloop_content.cell();
        let s_file = s_filename.map(|filename| -> Arc<File> {
            let mut options = OpenOptions::new();
            let file = options
                .read(true)
                .write(true)
                .create(true)
                .open(filename)
                .unwrap();
            Arc::new(file)
        });
        cloop_content.loop_(
            &s_file
                .map(|file: &Arc<File>| {
                    let mut file_content = String::new();
                    file.clone()
                        .read_to_string(&mut file_content)
                        .expect("Cannnot read file");
                    file_content
                })
                .hold("hoge".to_string()),
        );
        let c_filename = s_filename.map(|x| Some(x.clone())).hold(None);

        Self {
            c_content,
            c_filename,
        }
    }
}
