use rocket::Request;
use rocket_dyn_templates::Template;

#[catch(401)]
pub (crate) fn unauthorized(_req: &Request) -> Template {
        Template::render("error_pages/unauthorized", &())
}