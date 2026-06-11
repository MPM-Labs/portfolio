use tower_sessions::cookie::{SameSite, time::Duration};

pub enum CookieKind {
    JWT,
    Refresh,
}

pub struct Cookie<'a> {
    kind: CookieKind,
    token: &'a str,
}

impl<'a> Cookie<'a> {
    pub fn new(kind: CookieKind, token: &'a str) -> Self {
        Cookie { kind, token }
    }
    pub fn name(&self) -> &'static str {
        match self.kind {
            CookieKind::JWT => "jwt",
            CookieKind::Refresh => "refresh",
        }
    }
    pub fn path(&self) -> &'static str {
        match self.kind {
            CookieKind::JWT => "/admin",
            CookieKind::Refresh => "/auth",
        }
    }
    pub fn age(&self) -> Duration {
        match self.kind {
            CookieKind::JWT => Duration::minutes(15),
            CookieKind::Refresh => Duration::days(30),
        }
    }
    pub fn build(&self) -> tower_sessions::cookie::CookieBuilder<'static> {
        tower_sessions::cookie::Cookie::build((self.name(), self.token.to_owned()))
            .path(self.path())
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(self.age())
    }
}
