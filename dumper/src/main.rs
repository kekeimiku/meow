#[cfg(not(all(unix, target_pointer_width = "64", target_endian = "little")))]
panic!("Not support");

use dumper::cmd::{CommandEnum, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match argh::from_env::<Commands>().cmds {
        CommandEnum::WithDisk(args) => args.init(),
        CommandEnum::WithNet(args) => args.init(),
        CommandEnum::TestPtrs(args) => args.init(),
    }
}
