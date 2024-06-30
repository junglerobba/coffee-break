use clap::Args;
use std::fmt::{Debug, Display};
use sysinfo::Process;

#[derive(Args, Debug, Copy, Clone)]
pub(crate) struct CaffeinateFlags {
    /// prevent the display from sleeping
    #[arg(long, value_name = "true | false", default_value_t = true)]
    display: std::primitive::bool,
    /// prevent the system from idle sleeping
    #[arg(long, value_name = "true | false", default_value_t = true)]
    idle: std::primitive::bool,
    /// prevent the disk from idle sleeping
    #[arg(long, value_name = "true | false", default_value_t = true)]
    disk: std::primitive::bool,
    /// prevent the system from sleeping. valid only when system is running on AC power
    #[arg(long, value_name = "true | false", default_value_t = true)]
    sleep: std::primitive::bool,
}

impl CaffeinateFlags {
    /// returns true if any flag is set
    pub fn any(&self) -> bool {
        self.display || self.idle || self.disk || self.sleep
    }
}

impl IntoIterator for CaffeinateFlags {
    type Item = bool;
    type IntoIter = std::array::IntoIter<bool, 4>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.display, self.idle, self.disk, self.sleep])
    }
}

impl Display for CaffeinateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let features = ["display", "idle", "disk", "sleep"];
            f.write_str(
                &self
                    .into_iter()
                    .enumerate()
                    .filter(|(_, flag)| *flag)
                    .map(|(index, _)| features[index])
                    .collect::<Vec<&str>>()
                    .join(", "),
            )
        } else {
            let chars = ['d', 'i', 'm', 's'];
            f.write_str(
                &self
                    .into_iter()
                    .enumerate()
                    .filter(|(_, flag)| *flag)
                    .map(|(index, _)| chars[index])
                    .collect::<String>(),
            )
        }
    }
}

impl From<&[String]> for CaffeinateFlags {
    fn from(value: &[String]) -> Self {
        let chars = ['d', 'i', 'm', 's'];
        let mut flags = [false, false, false, false];

        for value in value.iter() {
            let Some(value) = value.strip_prefix('-') else {
                continue;
            };
            if value.starts_with('-') {
                continue;
            }
            for (index, flag) in chars.iter().enumerate() {
                if value.contains(*flag) {
                    flags[index] = true;
                }
            }
        }

        Self {
            display: flags[0],
            idle: flags[1],
            disk: flags[2],
            sleep: flags[3],
        }
    }
}

impl From<&Process> for CaffeinateFlags {
    fn from(value: &Process) -> Self {
        value.cmd().into()
    }
}
