use proc_macro::{TokenStream, TokenTree};
use std::fmt::Write;
use std::sync::atomic::AtomicU32;

static VAR_ID: AtomicU32 = AtomicU32::new(0);

#[proc_macro]
pub fn var(tokens: TokenStream) -> TokenStream {
    let id = VAR_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut t = tokens.into_iter();
    let name = t.next().expect("name has to be given");
    let mut val = vec![t.nth(1).unwrap()];
    loop {
        match t.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => break,
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => break,
            Some(t) => val.push(t),
            None => break,
        }
    }
    let val = if val.len() > 1 || !matches!(val[0], TokenTree::Literal(_)) {
        val.into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        format!("Val::Val({} as f32)", val[0])
    };

    let comment = t.next();
    let mut output = String::new();
    if let Some(comment) = comment {
        writeln!(
            output,
            "static _{name}: Var = Var::commented(\"{name}\", {comment}, {val}, {id});"
        )
        .unwrap();
        writeln!(output, "/// {comment}").unwrap();
    } else {
        writeln!(
            output,
            "static _{name}: Var = Var::new(\"{name}\", {val}, {id});"
        )
        .unwrap();
    }
    writeln!(output, "static {name}: &Var = &_{name};").unwrap();
    output.parse().unwrap()
}
