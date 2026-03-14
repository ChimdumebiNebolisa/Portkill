use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "portkill",
    about = "Find and kill the process using a given TCP or UDP port",
    version
)]
pub struct Cli {
    /// TCP or UDP port number (1-65535)
    #[arg(value_parser = parse_port)]
    pub port: u16,

    /// Show what would be killed; never prompt or kill
    #[arg(long)]
    pub dry_run: bool,

    /// Skip confirmation prompt before killing
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Use forceful termination (SIGKILL / /F) and skip confirmation
    #[arg(long, short = 'f')]
    pub force: bool,
}

fn parse_port(s: &str) -> Result<u16, String> {
    let port: u16 = s.parse().map_err(|_| format!("invalid port: '{}'", s))?;
    if port == 0 {
        return Err("port must be between 1 and 65535".to_string());
    }
    Ok(port)
}

impl Cli {
    pub fn should_prompt(&self) -> bool {
        !self.dry_run && !self.yes && !self.force
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_port_valid() {
        assert_eq!(parse_port("3000").unwrap(), 3000);
        assert_eq!(parse_port("1").unwrap(), 1);
        assert_eq!(parse_port("65535").unwrap(), 65535);
    }

    #[test]
    fn test_parse_port_invalid() {
        assert!(parse_port("0").is_err());
        assert!(parse_port("99999").is_err());
        assert!(parse_port("abc").is_err());
    }
}
