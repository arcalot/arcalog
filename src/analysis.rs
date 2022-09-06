pub enum Labels {
    Transient = 1,
    Environment = 2,
    Code = 3,
    Dependency = 4,
    Unknown = 5,
}

impl Labels {
    pub fn to_str(&mut self) -> String {
        match self {
            Self::Transient => "💨 Transient".to_string(),
            Self::Environment => "🤖 Environment".to_string(),
            Self::Code => "🐤 Code".to_string(),
            Self::Dependency => "🕳 Dependency".to_string(),
            Self::Unknown => "❓ Unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {}
