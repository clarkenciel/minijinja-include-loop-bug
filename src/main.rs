use serde::Serialize;

fn main() {
    let mut templates = minijinja::Environment::new();
    templates.set_loader(minijinja::path_loader("templates"));

    // Fails with cycle error
    println!(
        "{:#?}",
        templates
            .get_template("loop-things.txt.j2")
            .expect("template not found")
            .render(minijinja::context! {
                things => vec![
                    Thing{name: "a"},
                    Thing{name: "b"}
                ]
            })
    );

    // Fails with cycle error
    println!(
        "{:#?}",
        templates
            .get_template("static-things.txt.j2")
            .expect("template not found")
            .render(minijinja::context! {})
    )
}

#[derive(Debug, Serialize)]
struct Thing {
    name: &'static str,
}
