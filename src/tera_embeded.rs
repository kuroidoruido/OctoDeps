use std::sync::RwLock;

use rocket::fairing::Fairing;
use rocket::response::content::Html;
use rocket_contrib::templates::Template;
use serde::Serialize;
use tera::{Context, Tera};

// static TERA: AtomicPtr<Tera> = AtomicPtr::new(&mut Tera::default());

lazy_static! {
    static ref TERA: RwLock<Tera> = RwLock::new(Tera::default());
}

pub struct TeraEmbeded {}
impl TeraEmbeded {
    pub fn fairing(templates: &[TeraEmbededTemplate]) -> impl Fairing {
        let mut tera = TERA.write().unwrap();
        let _ = tera.add_raw_templates(
            templates
                .iter()
                .map(|t| (&(*t.name), t.template.clone())),
        );
        Template::fairing()
    }

    pub fn render<C>(template_name: &str, ctx: C) -> Html<Result<String, tera::Error>>
    where
        C: Serialize,
    {
        let tera = TERA.read().unwrap();
        let c = Context::from_serialize(ctx);
        match c {
            Ok(context) => Html(tera.render(template_name, &context)),
            Err(err) => Html(Err(err)),
        }
    }
}

pub struct TeraEmbededTemplate {
    pub name: &'static str,
    pub template: String,
}

// https://github.com/rust-lang/rust/issues/75075#issuecomment-671370162
#[cfg(host_family = "windows")]
#[macro_export]
macro_rules! tera_tpl {
    ($template_name: literal) => {
        TeraEmbededTemplate {
            name: $template_name,
            template: include_str!(concat!("..\\templates\\", $template_name, ".html.tera"))
                .to_string(),
        }
    };
}
#[cfg(not(host_family = "windows"))]
#[macro_export]
macro_rules! tera_tpl {
    ($template_name: literal) => {
        TeraEmbededTemplate {
            name: $template_name,
            template: include_str!(concat!("../templates/", $template_name, ".html.tera"))
                .to_string(),
        }
    };
}
