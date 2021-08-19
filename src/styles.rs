use console::Style;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ERROR: Style = Style::new().bold().red();
    pub static ref WARNING: Style = Style::new().bold().yellow();
    pub static ref INFO: Style = Style::new().bold().blue();
    pub static ref GOOD: Style = Style::new().bold().green();
}
