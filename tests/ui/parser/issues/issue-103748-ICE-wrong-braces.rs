#![crate_type = "lib"]

struct Apple((Apple, Option(Banana ? Citron)));
// Note: `Banana ?` is now parsed as Option<Banana> in type position
//~^^ ERROR unexpected token: `Citron`
