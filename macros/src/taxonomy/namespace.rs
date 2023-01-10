
use lazy_static::lazy_static;
use regex::Regex;
use syn::{parse::{Parse, ParseStream}, Result, Path, spanned::Spanned};

lazy_static! {
    static ref NAMESPACE_REGEX : Regex = Regex::new(r"[A-Z_]{3,}").unwrap();
}

pub struct Namespace {
    id : String,
    declaration: (String, usize, usize)
}

impl Namespace {
    pub fn id(&self) -> &String {
        self.id()
    }

    pub fn source_loc(&self) -> String {
        let (ref s, line, col) = self.declaration;
        format!("{s}:{line}:{col}")
    }
}

impl Parse for Namespace {
    fn parse(input: ParseStream) -> Result<Self> {
        let path : Path = input.parse()?;
        let id = match path.get_ident() {
            None =>
                return Err(
                    syn::Error::new(path.span(), "Namespace should be a valid Rust identifier.")
                ),
            Some(ident) => {
                let s = ident.to_string();

                if !NAMESPACE_REGEX.is_match(&s) {
                    ident.span().unwrap()
                        .warning("Xander: Namespaces names should use CONSTANT_CASE.")
                        .emit();
                }
                s
            }
        };

        let span = path.span().unwrap();
        Ok(Self {
            id: id.to_string(),
            declaration: (
                span.source_file().path().to_str().unwrap().to_string(),
                span.start().line,
                span.start().column,
            )
        })
    }
}