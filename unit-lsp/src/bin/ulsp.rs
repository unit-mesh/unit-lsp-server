use unit_lsp::intelligence::{Language, TreeSitterFile};

fn main() {
    println!("Hello, world!");

    let src = r#"
            #include <stdio.h>

            #define PI 355/113
            #define AREA(r) PI * r * r

            int main() {
                int radius = 5;
                printf("%d", AREA(radius));
            }
            "#.as_bytes();

    let lang_id = "Rust";
    let language = match Language::from_id(lang_id) {
        Language::Supported(config) => config,
        _ => panic!("testing unsupported language"),
    };

    println!("{:?}", language);

    let tsf = TreeSitterFile::try_build(src, lang_id).unwrap();
    let scope_graph = tsf.scope_graph().unwrap();
    println!("{:?}", scope_graph)
}