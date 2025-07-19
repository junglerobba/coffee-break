use bitflags::bitflags;
use clap::Args;
use std::fmt::{Debug, Display};
use sysinfo::Process;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct CaffeinateFlags: u8 {
        const DISPLAY = 0b0001;
        const IDLE    = 0b0010;
        const DISK    = 0b0100;
        const SLEEP   = 0b1000;
        const ALL     = Self::DISPLAY.bits() | Self::IDLE.bits() | Self::DISK.bits() | Self::SLEEP.bits();
    }
}

#[derive(Args, Debug, Copy, Clone)]
pub(crate) struct CaffeinateFlagsArgs {
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

const FLAG_CHARS: [char; 4] = ['d', 'i', 'm', 's'];
const FLAG_NAMES: [&str; 4] = ["display", "idle", "disk", "sleep"];

impl From<CaffeinateFlagsArgs> for CaffeinateFlags {
    fn from(args: CaffeinateFlagsArgs) -> Self {
        let mut flags = CaffeinateFlags::empty();
        if args.display {
            flags |= CaffeinateFlags::DISPLAY;
        }
        if args.idle {
            flags |= CaffeinateFlags::IDLE;
        }
        if args.disk {
            flags |= CaffeinateFlags::DISK;
        }
        if args.sleep {
            flags |= CaffeinateFlags::SLEEP;
        }
        flags
    }
}

impl CaffeinateFlags {
    /// returns true if any flag is set
    pub fn any(&self) -> bool {
        !self.is_empty()
    }
}

impl From<u8> for CaffeinateFlags {
    fn from(index: u8) -> Self {
        let mut flags = Self::empty();
        match index {
            0 => flags |= Self::DISPLAY,
            1 => flags |= Self::IDLE,
            2 => flags |= Self::DISK,
            3 => flags |= Self::SLEEP,
            _ => {}
        }
        flags
    }
}

impl Display for CaffeinateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let active_flags: Vec<&str> = self
                .iter()
                .filter_map(|flag| {
                    let index = flag.bits().trailing_zeros() as usize;
                    FLAG_NAMES.get(index).copied()
                })
                .collect();
            f.write_str(&active_flags.join(", "))
        } else {
            let active_chars: String = self
                .iter()
                .filter_map(|flag| {
                    let index = flag.bits().trailing_zeros() as usize;
                    FLAG_CHARS.get(index)
                })
                .collect();
            f.write_str(&active_chars)
        }
    }
}

impl From<&[String]> for CaffeinateFlags {
    fn from(value: &[String]) -> Self {
        let mut flags = CaffeinateFlags::empty();

        for arg in value.iter() {
            let Some(arg) = arg.strip_prefix('-') else {
                continue;
            };
            if arg.starts_with('-') {
                continue;
            }

            for (index, &flag_char) in FLAG_CHARS.iter().enumerate() {
                if arg.contains(flag_char) {
                    flags |= CaffeinateFlags::from(index as u8);
                }
            }
        }

        flags
    }
}

impl From<&Process> for CaffeinateFlags {
    fn from(value: &Process) -> Self {
        value.cmd().into()
    }
}
