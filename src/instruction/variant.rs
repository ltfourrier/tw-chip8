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
    ($c:expr, $o:expr) => (
        Variant{
            code: $c,
            opcode: $o,
            arguments: [argument!(None), argument!(None), argument!(None)],
        }
    );

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

    pub fn as_binary(&self, inst: &Instruction) -> Result<u16, InstructionError> {
        let mut bin_inst = self.opcode;
        for i in 0..MAX_ARGUMENTS {
            let mask = self.arguments[i].mask;
            let arg_value = inst.arguments[i].get_value()?;
            if mask != 0 && arg_value != 0 {
                bin_inst = bin_inst | ((arg_value << mask.trailing_zeros()) & mask);
            }
        }
        Ok(bin_inst)
    }
}

static INSTRUCTION_VARIANTS: [Variant; 36] =
    [
        variant!(Code::ClearScreen, 0x00E0),
        variant!(Code::Return, 0x00EE),
        variant!(Code::Jump, 0x1000, argument!(Address, 0xFFF)),
        variant!(Code::Call, 0x2000, argument!(Address, 0xFFF)),
        variant!(
            Code::SkipEquals,
            0x3000,
            argument!(Register, 0xF00),
            argument!(Byte, 0x0FF)
        ),
        variant!(
            Code::SkipNotEquals,
            0x4000,
            argument!(Register, 0xF00),
            argument!(Byte, 0x0FF)
        ),
        variant!(
            Code::SkipEquals,
            0x5000,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(
            Code::Load,
            0x6000,
            argument!(Register, 0xF00),
            argument!(Byte, 0x0FF)
        ),
        variant!(
            Code::Add,
            0x7000,
            argument!(Register, 0xF00),
            argument!(Byte, 0x0FF)
        ),
        variant!(
            Code::Load,
            0x8000,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(
            Code::Or,
            0x8001,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(
            Code::And,
            0x8002,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(
            Code::Xor,
            0x8003,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(
            Code::Add,
            0x8004,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(
            Code::Subtract,
            0x8005,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(Code::ShiftRight, 0x8006, argument!(Register, 0xF00)),
        variant!(
            Code::ShiftRight,
            0x8006,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(
            Code::SubtractInverse,
            0x8007,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(Code::ShiftLeft, 0x800E, argument!(Register, 0xF00)),
        variant!(
            Code::ShiftLeft,
            0x800E,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(
            Code::SkipNotEquals,
            0x9000,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0)
        ),
        variant!(
            Code::Load,
            0xA000,
            argument!(ImagePointer),
            argument!(Address, 0xFFF)
        ),
        variant!(
            Code::Jump,
            0xB000,
            argument!(Register),
            argument!(Address, 0xFFF)
        ),
        variant!(
            Code::Rand,
            0xC000,
            argument!(Register, 0xF00),
            argument!(Byte, 0x0FF)
        ),
        variant!(
            Code::Draw,
            0xD000,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0),
            argument!(Byte, 0x00F)
        ),
        variant!(Code::SkipKeyPressed, 0xE09E, argument!(Register, 0xF00)),
        variant!(Code::SkipKeyNotPressed, 0xE0A1, argument!(Register, 0xF00)),
        variant!(
            Code::Load,
            0xF007,
            argument!(Register, 0xF00),
            argument!(DelayTimer)
        ),
        variant!(
            Code::Load,
            0xF00A,
            argument!(Register, 0xF00),
            argument!(KeyRegister)
        ),
        variant!(
            Code::Load,
            0xF015,
            argument!(DelayTimer),
            argument!(Register, 0xF00)
        ),
        variant!(
            Code::Load,
            0xF018,
            argument!(SoundTimer),
            argument!(Register, 0xF00)
        ),
        variant!(
            Code::Add,
            0xF01E,
            argument!(ImagePointer),
            argument!(Register, 0xF00)
        ),
        variant!(
            Code::Load,
            0xF029,
            argument!(FontPointer),
            argument!(Register, 0xF00)
        ),
        variant!(
            Code::Load,
            0xF033,
            argument!(BcdPointer),
            argument!(Register, 0xF00)
        ),
        variant!(
            Code::Load,
            0xF055,
            argument!(ArrayPointer),
            argument!(Register, 0xF00)
        ),
        variant!(
            Code::Load,
            0xF065,
            argument!(Register, 0xF00),
            argument!(ArrayPointer)
        ),
    ];

pub fn get_variant(inst: &Instruction) -> Option<&Variant> {
    for variant in INSTRUCTION_VARIANTS.iter() {
        if variant.matches(inst) {
            return Some(variant);
        }
    }
    None
}

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

        // Test good instruction (with label)
        let good_label = Instruction {
            code: Code::Load,
            arguments: [
                Argument::ImagePointer,
                Argument::Label(String::from("tuturu")),
                Argument::None,
            ],
        };
        assert_eq!(variant.matches(&good_label), true);
    }

    #[test]
    fn binary_test() {
        // "DRW vx, vy, n" instruction
        let drw_variant = variant!(
            Code::Draw,
            0xD000,
            argument!(Register, 0xF00),
            argument!(Register, 0x0F0),
            argument!(Byte, 0x00F)
        );

        // "LD I, nnn" instruction
        let ld_variant = variant!(
            Code::Load,
            0xA000,
            argument!(ImagePointer),
            argument!(Address, 0xFFF)
        );

        // Test good DRW instruction
        let good_drw_inst = Instruction {
            code: Code::Draw,
            arguments: [
                Argument::Register(4),
                Argument::Register(2),
                Argument::Byte(12),
            ],
        };
        assert_eq!(drw_variant.as_binary(&good_drw_inst).unwrap(), 0xD42Cu16);

        // Test good LD instruction
        let good_ld_inst = Instruction {
            code: Code::Load,
            arguments: [
                Argument::ImagePointer,
                Argument::Address(0xDED),
                Argument::None,
            ],
        };
        assert_eq!(ld_variant.as_binary(&good_ld_inst).unwrap(), 0xADEDu16);
    }

    #[test]
    fn get_variant_test() {
        // Test instruction with one variant, without argument
        let cls_inst = Instruction {
            code: Code::ClearScreen,
            arguments: [
                Argument::None,
                Argument::None,
                Argument::None,
            ],
        };
        assert_eq!(get_variant(&cls_inst).unwrap().opcode, 0x00E0);

        // Test instruction with one variant, with good arguments
        let good_drw_inst = Instruction {
            code: Code::Draw,
            arguments: [
                Argument::Register(4),
                Argument::Register(2),
                Argument::Byte(10),
            ],
        };
        assert_eq!(get_variant(&good_drw_inst).unwrap().opcode, 0xD000);

        // Test instruction with one variant, but bad arguments
        let bad_drw_inst = Instruction {
            code: Code::Draw,
            arguments: [
                Argument::Register(4),
                Argument::Register(2),
                Argument::ImagePointer,
            ],
        };
        assert!(get_variant(&bad_drw_inst).is_none());

        // Test instruction with multiple variants
        let ldi_inst = Instruction {
            code: Code::Load,
            arguments: [
                Argument::ImagePointer,
                Argument::Address(0xDED),
                Argument::None,
            ],
        };
        assert_eq!(get_variant(&ldi_inst).unwrap().opcode, 0xA000);
    }
}
