use tower_sessions::cookie::{CookieBuilder, SameSite, time::Duration};

pub enum CookieKind {
    JWT,
    Refresh,
}

pub struct Cookie {
    kind: CookieKind,
    token: String,
}

impl Cookie {
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
    pub fn build(&self) -> CookieBuilder<'_> {
        tower_sessions::cookie::Cookie::build((self.name(), &self.token))
            .path(self.path())
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(self.age())
    }
}
