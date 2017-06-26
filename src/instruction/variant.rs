use super::*;

macro_rules! argument {
    ($t:tt) => (
        ArgumentDefinition{
            arg_type: ArgumentType::$t,
            mask: 0,
        }
    );

    ($t:tt, $m:expr) => (
        ArgumentDefinition{
            arg_type: ArgumentType::$t,
            mask: $m,
        }
    );
}

macro_rules! variant {
    ($c:expr, $o:expr, $arg1:expr) => (
        Variant{
            code: $c,
            opcode: $o,
            arguments: [$arg1, argument!(None), argument!(None)],
        }
    );

    ($c:expr, $o:expr, $arg1:expr, $arg2:expr) => (
        Variant{
            code: $c,
            opcode: $o,
            arguments: [$arg1, $arg2, argument!(None)],
        }
    );

    ($c:expr, $o:expr, $arg1:expr, $arg2:expr, $arg3:expr) => (
        Variant{
            code: $c,
            opcode: $o,
            arguments: [$arg1, $arg2, $arg3],
        }
    );
}

#[derive(PartialEq)]
pub enum ArgumentType {
    None,
    Address,
    Register,
    Byte,
    ImagePointer,
    KeyRegister,
    DelayTimer,
    SoundTimer,
    FontPointer,
    BcdPointer,
    ArrayPointer,
}

impl ArgumentType {
    fn from_argument(arg: &Argument) -> ArgumentType {
        match *arg {
            Argument::None => ArgumentType::None,
            Argument::Address(_) => ArgumentType::Address,
            Argument::Register(_) => ArgumentType::Register,
            Argument::Byte(_) => ArgumentType::Byte,
            Argument::ImagePointer => ArgumentType::ImagePointer,
            Argument::KeyRegister => ArgumentType::KeyRegister,
            Argument::DelayTimer => ArgumentType::DelayTimer,
            Argument::SoundTimer => ArgumentType::SoundTimer,
            Argument::FontPointer => ArgumentType::FontPointer,
            Argument::BcdPointer => ArgumentType::BcdPointer,
            Argument::ArrayPointer => ArgumentType::ArrayPointer,
            Argument::Label(_) => ArgumentType::Address,
        }
    }
}

pub struct ArgumentDefinition {
    arg_type: ArgumentType,
    mask: u16,
}

pub struct Variant {
    code: Code,
    opcode: u16,
    arguments: [ArgumentDefinition; MAX_ARGUMENTS],
}

impl Variant {
    pub fn matches(&self, inst: &Instruction) -> bool {
        if inst.code != self.code {
            return false;
        }
        for i in 0..MAX_ARGUMENTS {
            if self.arguments[i].arg_type != ArgumentType::from_argument(&inst.arguments[i]) {
                return false;
            }
        }
        true
    }
}

static INSTRUCTION_VARIANTS: [Variant; 1] = [
    variant!(
        Code::Load,
        0xA000,
        argument!(ImagePointer),
        argument!(Address, 0xFFF)
    ),
];

#[cfg(test)]
mod tests {
    use instruction::variant::*;

    #[test]
    fn matches_test() {
        // "LD I, 0xnnn" instruction
        let variant = variant!(
            Code::Load,
            0xA000,
            argument!(ImagePointer),
            argument!(Address, 0xFFF)
        );

        // Test good instruction
        let good = Instruction {
            code: Code::Load,
            arguments: [
                Argument::ImagePointer,
                Argument::Address(0x123),
                Argument::None,
            ],
        };
        assert_eq!(variant.matches(&good), true);

        // Test bad code instruction
        let bad_code = Instruction {
            code: Code::Add,
            arguments: [
                Argument::ImagePointer,
                Argument::Address(0x123),
                Argument::None,
            ],
        };
        assert_eq!(variant.matches(&bad_code), false);

        // Test bad argument instruction
        let bad_arg = Instruction {
            code: Code::Load,
            arguments: [
                Argument::ImagePointer,
                Argument::Address(0x123),
                Argument::Byte(42),
            ],
        };
        assert_eq!(variant.matches(&bad_arg), false);
    }
}
