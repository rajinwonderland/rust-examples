#![crate_name = "bytecode"]

use crate::debug::PrettyPrint;

mod interpreter;
mod bytecode;
mod objects;
mod types;
mod serializable;
mod program;
mod debug;
mod io;
mod compiler;

#[cfg(test)]
mod bytecode_deserialization_tests {
    use std::io::Cursor;
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::types::{ConstantPoolIndex, LocalFrameIndex, Arity};

    fn test(expected: OpCode, input: Vec<u8>) {
        assert_eq!(OpCode::from_bytes(&mut Cursor::new(input)), expected);
    }

    #[test] fn label () {
        let expected = OpCode::Label { name: ConstantPoolIndex::new(1) };
        let bytes = vec!(0x00, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn literal () {
        let expected = OpCode::Literal { index: ConstantPoolIndex::new(1) };
        let bytes = vec!(0x01, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn get_local () {
        let expected = OpCode::GetLocal { index: LocalFrameIndex::new(1) };
        let bytes = vec!(0x0A, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn set_local () {
        let expected = OpCode::SetLocal { index: LocalFrameIndex::new(1) };
        let bytes = vec!(0x09, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn get_global () {
        let expected = OpCode::GetGlobal { name: ConstantPoolIndex::new(1) };
        let bytes = vec!(0x0C, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn set_global () {
        let expected = OpCode::SetGlobal { name: ConstantPoolIndex::new(1) };
        let bytes = vec!(0x0B, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn object () {
        let expected = OpCode::Object { class: ConstantPoolIndex::new(1) };
        let bytes = vec!(0x04, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn array () {
        let expected = OpCode::Array;
        let bytes = vec!(0x03);
        test(expected, bytes);
    }

    #[test] fn get_slot () {
        let expected = OpCode::GetSlot { name: ConstantPoolIndex::new(1) };
        let bytes = vec!(0x05, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn set_slot () {
        let expected = OpCode::SetSlot { name: ConstantPoolIndex::new(1) };
        let bytes = vec!(0x06, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn call_method () {
        let expected = OpCode::CallMethod { name: ConstantPoolIndex::new(1), arguments: Arity::new(1) };
        let bytes = vec!(0x07, 0x01, 0x00, 0x01);
        test(expected, bytes);
    }

    #[test] fn call_function () {
        let expected = OpCode::CallFunction { name: ConstantPoolIndex::new(1), arguments: Arity::new(2) };
        let bytes = vec!(0x08, 0x01, 0x00, 0x02);
        test(expected, bytes);
    }

    #[test] fn print () {
        let expected = OpCode::Print { format: ConstantPoolIndex::new(1), arguments: Arity::new(2) };
        let bytes = vec!(0x02, 0x01, 0x00, 0x02);
        test(expected, bytes);
    }

    #[test] fn jump () {
        let expected = OpCode::Jump { label: ConstantPoolIndex::new(1) };
        let bytes = vec!(0x0E, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn branch () {
        let expected = OpCode::Branch { label: ConstantPoolIndex::new(1) };
        let bytes = vec!(0x0D, 0x01, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn return_op () {
        let expected = OpCode::Return;
        let bytes = vec!(0x0F);
        test(expected, bytes);
    }

    #[test] fn drop () {
        let expected = OpCode::Drop;
        let bytes = vec!(0x10);
        test(expected, bytes);
    }
}

#[cfg(test)]
mod bytecode_serialization_tests {
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::types::{ConstantPoolIndex, LocalFrameIndex, Arity};

    fn test (expected: Vec<u8>, object: OpCode) {
        let mut actual: Vec<u8> = Vec::new();
        object.serialize(&mut actual);
        assert_eq!(actual, expected);
    }

    #[test] fn label () {
        let expected = vec!(0x00, 0x01, 0x00);
        let object = OpCode::Label { name: ConstantPoolIndex::new(1) };
        test(expected, object);
    }

    #[test] fn literal () {
        let expected = vec!(0x01, 0x01, 0x00, );
        let object = OpCode::Literal { index: ConstantPoolIndex::new(1) };
        test(expected, object);
    }

    #[test] fn get_local () {
        let expected = vec!(0x0A, 0x01, 0x00, );
        let object = OpCode::GetLocal { index: LocalFrameIndex::new(1) };
        test(expected, object);
    }

    #[test] fn set_local () {
        let expected = vec!(0x09, 0x01, 0x00,);
        let object = OpCode::SetLocal { index: LocalFrameIndex::new(1) };
        test(expected, object);
    }

    #[test] fn get_global () {
        let expected = vec!(0x0C, 0x01, 0x00, );
        let object = OpCode::GetGlobal { name: ConstantPoolIndex::new(1) };
        test(expected, object);
    }

    #[test] fn set_global () {
        let expected = vec!(0x0B, 0x01, 0x00, );
        let object = OpCode::SetGlobal { name: ConstantPoolIndex::new(1) };
        test(expected, object);
    }

    #[test] fn object () {
        let expected = vec!(0x04, 0x01, 0x00, );
        let object = OpCode::Object { class: ConstantPoolIndex::new(1) };
        test(expected, object);
    }

    #[test] fn array () {
        let expected = vec!(0x03);
        let object = OpCode::Array;
        test(expected, object);
    }

    #[test] fn get_slot () {
        let expected = vec!(0x05, 0x01, 0x00, );
        let object = OpCode::GetSlot { name: ConstantPoolIndex::new(1) };
        test(expected, object);
    }

    #[test] fn set_slot () {
        let expected = vec!(0x06, 0x01, 0x00, );
        let object = OpCode::SetSlot { name: ConstantPoolIndex::new(1) };
        test(expected, object);
    }

    #[test] fn call_method () {
        let expected = vec!(0x07, 0x01, 0x00, 0x01);
        let object = OpCode::CallMethod { name: ConstantPoolIndex::new(1), arguments: Arity::new(1) };
        test(expected, object);
    }

    #[test] fn call_function () {
        let expected = vec!(0x08, 0x01, 0x00, 0x02);
        let object = OpCode::CallFunction { name: ConstantPoolIndex::new(1), arguments: Arity::new(2) };
        test(expected, object);
    }

    #[test] fn print () {
        let expected = vec!(0x02, 0x01, 0x00, 0x02);
        let object = OpCode::Print { format: ConstantPoolIndex::new(1), arguments: Arity::new(2) };
        test(expected, object);
    }

    #[test] fn jump () {
        let expected = vec!(0x0E, 0x01, 0x00, );
        let object = OpCode::Jump { label: ConstantPoolIndex::new(1) };
        test(expected, object);
    }

    #[test] fn branch () {
        let expected = vec!(0x0D, 0x01, 0x00, );
        let object = OpCode::Branch { label: ConstantPoolIndex::new(1) };
        test(expected, object);
    }

    #[test] fn return_op () {
        let expected = vec!(0x0F);
        let object = OpCode::Return;
        test(expected, object);
    }

    #[test] fn drop () {
        let expected = vec!(0x10);
        let object = OpCode::Drop;
        test(expected, object);
    }
}

#[cfg(test)]
mod program_object_serialization_tests {
    use crate::bytecode::OpCode;
    use crate::serializable::SerializableWithContext;
    use crate::types::{ConstantPoolIndex, Size, Arity, AddressRange};
    use crate::objects::ProgramObject;
    use crate::program::Code;

    fn test(expected: Vec<u8>, object: ProgramObject) {
        let mut output: Vec<u8> = Vec::new();
        let code = Code::new();
        object.serialize(&mut output, &code);
        assert_eq!(output, expected);
    }

    fn test_with_context(expected: Vec<u8>, object: ProgramObject, code: Code) {
        let mut output: Vec<u8> = Vec::new();
        object.serialize(&mut output, &code);
        assert_eq!(output, expected);
    }

    #[test] fn null () {
        let expected = vec!(0x01);
        let object = ProgramObject::Null;
        test(expected, object);
    }

    #[test] fn integer () {
        let expected = vec!(0x00, 0x2A, 0x00, 0x00, 0x00);
        let object = ProgramObject::Integer(42);
        test(expected, object);
    }

    #[test] fn boolean () {
        let expected = vec!(0x06, 0x01);
        let object = ProgramObject::Boolean(true);
        test(expected, object);
    }

    #[test] fn string () {
        let expected = vec!(0x02,
                            0x0C, 0x00, 0x00, 0x00,
                            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x0A);
        let object = ProgramObject::String("Hello World\n".to_string());
        test(expected, object);
    }

    #[test] fn slot () {
        let expected = vec!(0x04, 0x2A, 0x00);
        let object = ProgramObject::Slot { name: ConstantPoolIndex::new(42) };
        test(expected, object);
    }

    #[test] fn class () {
        let expected = vec!(0x05,
                            0x02, 0x00,
                            0x2A, 0x00,
                            0x9A, 0x02, );
        let object = ProgramObject::Class(vec!(ConstantPoolIndex::new(42),
                                               ConstantPoolIndex::new(666)));
        test(expected, object);
    }

    #[test] fn method () {
        let expected = vec!(0x03,
                            0xFF, 0x00,
                            0x03,
                            0x0F, 0x00,
                            0x02, 0x00, 0x00, 0x00,
                            0x01,
                            0x2A, 0x00,
                            0x0F);

        let object = ProgramObject::Method {
            name: ConstantPoolIndex::new(255),
            arguments: Arity::new(3),
            locals: Size::new(15),
            code: AddressRange::from(0, 2),
        };

        let code = Code::from(vec!(/* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(42) },
                                   /* 1 */ OpCode::Return));

        test_with_context(expected, object, code);
    }
}


#[cfg(test)]
mod program_object_deserialization_tests {
    use crate::bytecode::OpCode;
    use crate::serializable::{SerializableWithContext};
    use crate::types::{ConstantPoolIndex, Size, Arity, AddressRange};
    use crate::objects::ProgramObject;
    use std::io::Cursor;
    use crate::program::Code;

    fn test(expected: ProgramObject, input: Vec<u8>) {
        let mut code = Code::new();
        let object = ProgramObject::from_bytes(&mut Cursor::new(input), &mut code);
        assert_eq!(object, expected);
        assert_eq!(code, Code::new());
    }

    fn test_with_context(expected_object: ProgramObject, expected_code: Code, input: Vec<u8>) {
        let mut code = Code::new();
        let object = ProgramObject::from_bytes(&mut Cursor::new(input), &mut code);
        assert_eq!(object, expected_object);
        assert_eq!(code, expected_code);
    }

    #[test] fn null () {
        let expected = ProgramObject::Null;
        let bytes = vec!(0x01);
        test(expected, bytes);
    }

    #[test] fn integer () {
        let expected = ProgramObject::Integer(42);
        let bytes = vec!(0x00, 0x2A, 0x00, 0x00, 0x00);
        test(expected, bytes);
    }

    #[test] fn boolean () {
        let expected = ProgramObject::Boolean(true);
        let bytes = vec!(0x06, 0x01);
        test(expected, bytes);
    }

    #[test] fn string () {
        let expected = ProgramObject::String("Hello World\0".to_string());
        let bytes = vec!(0x02,
                         0x0C, 0x00, 0x00, 0x00,
                         0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00);
        test(expected, bytes);
    }

    #[test] fn slot () {
        let expected = ProgramObject::Slot { name: ConstantPoolIndex::new(42) };
        let bytes = vec!(0x04, 0x2A, 0x00, );
        test(expected, bytes);
    }

    #[test] fn class () {
        let expected = ProgramObject::Class(vec!(ConstantPoolIndex::new(42),
                                                 ConstantPoolIndex::new(666)));
        let bytes = vec!(0x05,
                         0x02, 0x00,
                         0x2A, 0x00,
                         0x9A, 0x02, );
        test(expected, bytes);
    }


    #[test] fn method () {
        let object = ProgramObject::Method { name: ConstantPoolIndex::new(255),
                                             arguments: Arity::new(3),
                                             locals: Size::new(15),
                                             code: AddressRange::from(0, 2)};

        let code = Code::from(vec!(OpCode::Literal { index: ConstantPoolIndex::new(42) },
                                   OpCode::Return));

        let bytes = vec!(0x03,
                         0xFF, 0x00,
                         0x03,
                         0x0F, 0x00,
                         0x02, 0x00, 0x00, 0x00,
                         0x01,
                         0x2A, 0x00,
                         0x0F);

        test_with_context(object, code, bytes);
    }
}

#[cfg(test)]
mod interpreter_test {
    use crate::bytecode::OpCode;
    use crate::types::{ConstantPoolIndex, Address, LocalFrameIndex, Arity, Size, AddressRange};
    use crate::program::{Program, Code};
    use crate::objects::{ProgramObject, Pointer, Object};
    use crate::interpreter::{State, interpret, LocalFrame, Memory};
    use std::collections::HashMap;
    use std::io::Write;

    macro_rules! hashmap {
        ($key: expr, $value: expr) => {{
            let mut map = HashMap::new();
            map.insert($key, $value);
            map
        }};
        ($key1: expr, $value1: expr, $key2: expr, $value2: expr) => {{
            let mut map = HashMap::new();
            map.insert($key1, $value1);
            map.insert($key2, $value2);
            map
        }};
    }

    #[test] fn literal() {
        let code = Code::from(vec!(
            OpCode::Literal { index: ConstantPoolIndex::new(0) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::Integer(42));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(0)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(42))), "test memory");
    }

    #[test] fn label() {
        let code = Code::from(vec!(
            OpCode::Label { name: ConstantPoolIndex::new(0) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("o.o".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, Vec::new(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!()), "test memory");
    }

    #[test] fn get_local() {
        let code = Code::from(vec!(
            OpCode::GetLocal { index: LocalFrameIndex::new(0) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!();
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        let pointer = state.allocate(Object::from_i32(42));
        state.current_frame_mut().unwrap().push_local(pointer);

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(0)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::from(None, vec!(Pointer::from(0)))), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(42))), "test memory")
    }

    #[test] fn set_local() {
        let code = Code::from(vec!(
            OpCode::SetLocal { index: LocalFrameIndex::new(0) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!();
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.allocate_and_push_operand(Object::from_i32(42));
        let pointer = state.allocate(Object::from_i32(0));
        state.current_frame_mut().unwrap().push_local(pointer);

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(0)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::from(None, vec!(Pointer::from(0)))), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(42),
                                                   Object::from_i32(0))), "test memory");
    }

    #[test] fn get_global() {
        let code = Code::from(vec!(
            OpCode::GetGlobal { name: ConstantPoolIndex::new(0) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("skippy".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!(ConstantPoolIndex::new(0));
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        let pointer = state.allocate(Object::from_i32(666));
        state.register_global("skippy".to_string(), pointer);

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(0)), "test operands");
        assert_eq!(state.globals, hashmap!("skippy".to_string(), Pointer::from(0)), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(666))), "test memory");
    }

    #[test] fn set_global() {
        let code = Code::from(vec!(
            OpCode::SetGlobal { name: ConstantPoolIndex::new(0) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("skippy".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!(ConstantPoolIndex::new(0));
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.allocate_and_push_operand(Object::from_i32(42));
        state.allocate_and_register_global("skippy".to_string(), Object::from_i32(666));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(0)), "test operands");
        assert_eq!(state.globals, hashmap!("skippy".to_string(), Pointer::from(0)), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(42), Object::from_i32(666))), "test memory");
    }

    #[test] fn drop() {
        let code = Code::from(vec!(
            OpCode::Drop,
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!();
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.allocate_and_push_operand(Object::from_i32(7));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, Vec::new(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(7))), "test memory");
    }

    #[test] fn jump() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Label { name: ConstantPoolIndex::new(0) },
            /*1*/ OpCode::Skip,
            /*2*/ OpCode::Jump { label: ConstantPoolIndex::new(0) },
            /*3*/ OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("^.^".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.set_instruction_pointer(Some(Address::from_usize(2)));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, Vec::new(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(0)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::new(), "test memory")
    }

    #[test] fn branch_true() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Label { name: ConstantPoolIndex::new(0) },
            /*1*/ OpCode::Skip,
            /*2*/ OpCode::Branch { label: ConstantPoolIndex::new(0) },
            /*3*/ OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("x.x".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.set_instruction_pointer(Some(Address::from_usize(2)));
        state.allocate_and_push_operand(Object::from_bool(true));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, Vec::new(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(0)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_bool(true))), "test memory");
    }

    #[test] fn branch_false() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Label { name: ConstantPoolIndex::new(0) },
            /*1*/ OpCode::Skip,
            /*2*/ OpCode::Branch { label: ConstantPoolIndex::new(0) },
            /*3*/ OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("butt".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.set_instruction_pointer(Some(Address::from_usize(2)));
        state.allocate_and_push_operand(Object::from_bool(false));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, Vec::new(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(3)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_bool(false))), "test memory");
    }

    #[test] fn print() {
        let code = Code::from(vec!(
            OpCode::Print { format: ConstantPoolIndex::new(0), arguments: Arity::new(0) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("Ahoj przygodo!\n".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "Ahoj przygodo!\n", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(0)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::Null)), "test memory");
    }

    #[test] fn print_one() {
        let code = Code::from(vec!(
            OpCode::Print { format: ConstantPoolIndex::new(0), arguments: Arity::new(1) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("~!\n".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.allocate_and_push_operand(Object::from_i32(42));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "42!\n", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(1)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(42), Object::Null)), "test memory")
    }

    #[test] fn print_two() {
        let code = Code::from(vec!(
            OpCode::Print { format: ConstantPoolIndex::new(0), arguments: Arity::new(2) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("~x~!\n".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.allocate_and_push_operand(Object::from_i32(0));
        state.allocate_and_push_operand(Object::from_i32(42));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "0x42!\n", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(2)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(0), Object::from_i32(42), Object::Null)), "test memory")
    }

    #[test] fn skip() {
        let code = Code::from(vec!(
            OpCode::Skip,
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!();
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, Vec::new(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::new())
    }

    #[test] fn array_zero() {
        let code = Code::from(vec!(
            OpCode::Array,
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!();
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.allocate_and_push_operand(Object::from_i32(0));
        state.allocate_and_push_operand(Object::Null);

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(2)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(0), Object::Null,
                                                   Object::from_pointers(vec!()))), "test memory");
    }

    #[test] fn array_one() {
        let code = Code::from(vec!(
            OpCode::Array,
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!();
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.allocate_and_push_operand(Object::from_i32(1));
        state.allocate_and_push_operand(Object::Null);

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(3)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(1), Object::Null, Object::Null,
                                                   Object::from_pointers(vec!(Pointer::from(2))))), "test memory");
    }

    #[test] fn array_three() {
        let code = Code::from(vec!(
            OpCode::Array,
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!();
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.allocate_and_push_operand(Object::from_i32(3));
        state.allocate_and_push_operand(Object::from_i32(0));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(5)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(3),
                                                   Object::from_i32(0),
                                                   Object::from_i32(0),
                                                   Object::from_i32(0),
                                                   Object::from_i32(0),
                                                   Object::from_pointers(vec!(Pointer::from(2),
                                                                              Pointer::from(3),
                                                                              Pointer::from(4))))), "test memory");
    }

    #[test] fn call_function_zero() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Return,
            /*1*/ OpCode::CallFunction { name: ConstantPoolIndex::new(0), arguments: Arity::new(0) },
            /*2*/ OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(
            ProgramObject::String("bar".to_string()),
            ProgramObject::Method { name: ConstantPoolIndex::new(0),
                arguments: Arity::new(0),
                locals: Size::new(0),
                code: AddressRange::from(0,1) });

        let mut state = State::minimal();
        state.functions.insert("bar".to_string(), constants.get(1).unwrap().clone());
        state.set_instruction_pointer(Some(Address::from_usize(1)));

        let globals: Vec<ConstantPoolIndex> = vec!(ConstantPoolIndex::new(1));
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut output: String = String::new();


        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(0)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty(),
                                      LocalFrame::from(Some(Address::from_usize(2)), vec!())), "test frames");
        assert_eq!(state.memory, Memory::from(vec!()))
    }

    #[test] fn call_function_one() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Return,
            /*1*/ OpCode::CallFunction { name: ConstantPoolIndex::new(0), arguments: Arity::new(1) },
            /*2*/ OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(
            ProgramObject::String("foo".to_string()),
            ProgramObject::Method { name: ConstantPoolIndex::new(0),
                arguments: Arity::new(1),
                locals: Size::new(0),
                code: AddressRange::from(0,1) });

        let mut state = State::minimal();
        state.functions.insert("foo".to_string(), constants.get(1).unwrap().clone());
        state.allocate_and_push_operand(Object::from_i32(42));
        state.set_instruction_pointer(Some(Address::from_usize(1)));

        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut output: String = String::new();

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(0)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty(),
                                      LocalFrame::from(Some(Address::from_usize(2)),
                                                       vec!(Pointer::from(0)))), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(42))), "test memory");
    }

    #[test] fn call_function_three() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Return,
            /*1*/ OpCode::CallFunction { name: ConstantPoolIndex::new(0), arguments: Arity::new(3) },
            /*2*/ OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(
            ProgramObject::String("fun".to_string()),
            ProgramObject::Method { name: ConstantPoolIndex::new(0),
                                    arguments: Arity::new(3),
                                    locals: Size::new(0),
                                    code: AddressRange::from(0,1) });

        let mut state = State::minimal();
        state.functions.insert("fun".to_string(), constants.get(1).unwrap().clone());

        state.allocate_and_push_operand(Object::from_i32(1));
        state.allocate_and_push_operand(Object::from_i32(2));
        state.allocate_and_push_operand(Object::from_i32(3));

        state.set_instruction_pointer(Some(Address::from_usize(1)));

        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut output: String = String::new();

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(0)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty(),
                                      LocalFrame::from(Some(Address::from_usize(2)),
                                                       vec!(Pointer::from(0),
                                                            Pointer::from(1),
                                                            Pointer::from(2),))), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(1),
                                                   Object::from_i32(2),
                                                   Object::from_i32(3),
        )))
    }

    #[test] fn returns() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Return,
            /*1*/ OpCode::CallFunction { name: ConstantPoolIndex::new(1), arguments: Arity::new(3) },
            /*2*/ OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(
            ProgramObject::String("xxx".to_string()),
            ProgramObject::Method { name: ConstantPoolIndex::new(0),
                arguments: Arity::new(3),
                locals: Size::new(0),
                code: AddressRange::from(0,1) });
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        //state.set_instruction_pointer(Some(Address::from_usize(0)));

        let pointer1 = state.allocate(Object::from_i32(1));
        let pointer2 = state.allocate(Object::from_i32(2));
        let pointer3 = state.allocate(Object::from_i32(3));
        state.new_frame(Some(Address::from_usize(2)),
                        vec!(pointer1, pointer2, pointer3));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(2)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(1),
                                                   Object::from_i32(2),
                                                   Object::from_i32(3))), "test memory");
    }

    #[test] fn object_zero() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Return,
            /*1*/ OpCode::Object { class: ConstantPoolIndex::new(2) },
            /*2*/ OpCode::Skip
        ));

        let constants: Vec<ProgramObject> = vec!(
            /*0*/ ProgramObject::String ("+".to_string()),
            /*1*/ ProgramObject::Method { name: ConstantPoolIndex::new(0),
                arguments: Arity::new(1),
                locals: Size::new(0),
                code: AddressRange::from(0, 1)},

            /*2*/ ProgramObject::Class(vec!(ConstantPoolIndex::new(1))),
        );
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.set_instruction_pointer(Some(Address::from_usize(1)));
        state.allocate_and_push_operand(Object::Null);

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(1)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(2)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::Null,
                                                   Object::from(Pointer::from(0),
                                                                HashMap::new(),
                                                                hashmap!("+".to_string(), ProgramObject::Method { name: ConstantPoolIndex::new(0),
                                                                                                                  arguments: Arity::new(1),
                                                                                                                  locals: Size::new(0),
                                                                                                                  code: AddressRange::from(0, 1)})))), "test memory");
    }

    #[test] fn object_one() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Return,
            /*1*/ OpCode::Object { class: ConstantPoolIndex::new(4) },
            /*2*/ OpCode::Skip
        ));

        let constants: Vec<ProgramObject> = vec!(
            /*0*/ ProgramObject::String ("x".to_string()),
            /*1*/ ProgramObject::Slot { name: ConstantPoolIndex::new(0) },

            /*2*/ ProgramObject::String ("+".to_string()),
            /*3*/ ProgramObject::Method { name: ConstantPoolIndex::new(2),
                arguments: Arity::new(1),
                locals: Size::new(0),
                code: AddressRange::from(0, 1)},

            /*4*/ ProgramObject::Class(vec!(ConstantPoolIndex::new(1),
                                            ConstantPoolIndex::new(3))),
        );
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.set_instruction_pointer(Some(Address::from_usize(1)));
        state.allocate_and_push_operand(Object::Null);          // parent
        state.allocate_and_push_operand(Object::from_i32(0));     // x

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(2)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(2)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::Null,
                                                   Object::from_i32(0),
                                                   Object::from(Pointer::from(0),
                                                                hashmap!("x".to_string(), Pointer::from(1)),
                                                                hashmap!("+".to_string(), ProgramObject::Method { name: ConstantPoolIndex::new(2),
                                                                                                                  arguments: Arity::new(1),
                                                                                                                  locals: Size::new(0),
                                                                                                                  code: AddressRange::from(0, 1)})))));
    }

    #[test] fn object_two() {
        let code = Code::from(vec!(
            /*0*/ OpCode::Return,
            /*1*/ OpCode::Object { class: ConstantPoolIndex::new(6) },
            /*2*/ OpCode::Skip
        ));

        let constants: Vec<ProgramObject> = vec!(
            /*0*/ ProgramObject::String ("x".to_string()),
            /*1*/ ProgramObject::Slot { name: ConstantPoolIndex::new(0) },

            /*2*/ ProgramObject::String ("y".to_string()),
            /*3*/ ProgramObject::Slot { name: ConstantPoolIndex::new(2) },

            /*4*/ ProgramObject::String ("+".to_string()),
            /*5*/ ProgramObject::Method { name: ConstantPoolIndex::new(4),
                                          arguments: Arity::new(1),
                                          locals: Size::new(0),
                                          code: AddressRange::from(0, 1)},

            /*6*/ ProgramObject::Class(vec!(ConstantPoolIndex::new(1),
                                            ConstantPoolIndex::new(3),
                                            ConstantPoolIndex::new(5))),
        );
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.set_instruction_pointer(Some(Address::from_usize(1)));
        state.allocate_and_push_operand(Object::Null);              // parent
        state.allocate_and_push_operand(Object::from_i32(42));      // y
        state.allocate_and_push_operand(Object::from_i32(0));       // x


        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(3)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(2)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::Null,
                                                   Object::from_i32(42),
                                                   Object::from_i32(0),
                                                   Object::from(Pointer::from(0),
                                                                hashmap!("x".to_string(), Pointer::from(2), "y".to_string(), Pointer::from(1)),
                                                                hashmap!("+".to_string(), ProgramObject::Method {
                                                                                            name: ConstantPoolIndex::new(4),
                                                                                            arguments: Arity::new(1),
                                                                                            locals: Size::new(0),
                                                                                            code: AddressRange::from(0, 1)})))));
    }

    #[test] fn get_slot() {
        let code = Code::from(vec!(
            OpCode::GetSlot { name: ConstantPoolIndex::new(0) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("value".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.allocate(Object::Null);
        state.allocate(Object::from_i32(42));
        state.allocate_and_push_operand(Object::from(Pointer::from(0),
                                                     hashmap!("value".to_string(), Pointer::from(1)),
                                                     HashMap::new()));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(1)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::Null,
                                                   Object::from_i32(42),
                                                   Object::from(Pointer::from(0),
                                                                hashmap!("value".to_string(), Pointer::from(1)),
                                                                HashMap::new()))));
    }

    #[test] fn set_slot() {
        let code = Code::from(vec!(
            OpCode::SetSlot { name: ConstantPoolIndex::new(0) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("value".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        let object = Object::from(Pointer::from(0),
                                  hashmap!("value".to_string(), Pointer::from(1)),
                                  HashMap::new());

//        state.allocate_and_push_operand(Object::from_i32(42));
        state.allocate(Object::Null);
        state.allocate_and_push_operand(object.clone());
        state.allocate_and_push_operand(Object::from_i32(666));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(2)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::Null,
                                                   Object::from(Pointer::from(0),
                                                                hashmap!("value".to_string(), Pointer::from(2)),
                                                                HashMap::new()),
                                                   Object::from_i32(666))));

        assert_eq!(object, Object::from(Pointer::from(0),
                                        hashmap!("value".to_string(), Pointer::from(1)),
                                        HashMap::new()));
    }

    #[test] fn call_method_zero() {
        let code = Code::from(vec!(
            OpCode::Return,
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(0 + 1) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("f".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        let receiver = Object::from(Pointer::from(0),
                                    HashMap::new(),
                                    hashmap!("f".to_string(), ProgramObject::Method { name: ConstantPoolIndex::new(0),
                                                                                      arguments: Arity::new(0 + 1),
                                                                                      locals: Size::new(0),
                                                                                      code: AddressRange::from(0, 1) }));

        state.set_instruction_pointer(Some(Address::from_usize(1)));
        state.allocate(Object::Null);
        state.allocate_and_push_operand(receiver.clone());

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, Vec::new(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(0)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty(),
                                      LocalFrame::from(Some(Address::from_usize(2)),
                                                       vec!(Pointer::from(1)))), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::Null, receiver.clone())))
    }

    #[test] fn call_method_one() {
        let code = Code::from(vec!(
            OpCode::Return,
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(1 + 1) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("+".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        let receiver = Object::from(Pointer::from(0),
                                    HashMap::new(),
                                    hashmap!("+".to_string(), ProgramObject::Method { name: ConstantPoolIndex::new(0),
                                                                                      arguments: Arity::new(1 + 1),
                                                                                      locals: Size::new(0),
                                                                                      code: AddressRange::from(0, 1) }));

        state.set_instruction_pointer(Some(Address::from_usize(1)));
        state.allocate(Object::Null);
        state.allocate_and_push_operand(receiver.clone());
        state.allocate_and_push_operand(Object::from_i32(1));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, Vec::new(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(0)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty(),
                                      LocalFrame::from(Some(Address::from_usize(2)),
                                                       vec!(Pointer::from(1),
                                                            Pointer::from(2)))), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::Null,
                                                   receiver.clone(),
                                                   Object::from_i32(1))))
    }

    #[test] fn call_method_three() {
        let code = Code::from(vec!(
            OpCode::Return,
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(3 + 1) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("g".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        let receiver = Object::from(Pointer::from(0),
                                    HashMap::new(),
                                    hashmap!("g".to_string(), ProgramObject::Method { name: ConstantPoolIndex::new(0),
                                                                                      arguments: Arity::new(3 + 1),
                                                                                      locals: Size::new(0),
                                                                                      code: AddressRange::from(0, 1) }));

        state.set_instruction_pointer(Some(Address::from_usize(1)));
        state.allocate(Object::Null);
        state.allocate_and_push_operand(receiver.clone());
        state.allocate_and_push_operand(Object::from_i32(1));
        state.allocate_and_push_operand(Object::from_i32(2));
        state.allocate_and_push_operand(Object::from_i32(3));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, Vec::new(), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(0)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty(),
                                      LocalFrame::from(Some(Address::from_usize(2)),
                                                       vec!(Pointer::from(1),
                                                            Pointer::from(2),
                                                            Pointer::from(3),
                                                            Pointer::from(4),))), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::Null,
                                                   receiver.clone(),
                                                   Object::from_i32(1),
                                                   Object::from_i32(2),
                                                   Object::from_i32(3))))
    }

    fn call_method(receiver: Object, argument: Object, operation: &str, result: Object) {
        let code = Code::from(vec!(
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(1 + 1) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String(operation.to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.set_instruction_pointer(Some(Address::from_usize(0)));
        state.allocate_and_push_operand(receiver.clone());
        state.allocate_and_push_operand(argument.clone());

        interpret(&mut state, &mut output, &program);

        let mut expected_memory = Memory::new();
        expected_memory.allocate(receiver.clone());
        expected_memory.allocate(argument.clone());
        let result_pointer = expected_memory.allocate(result.clone());

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(result_pointer), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, expected_memory)
    }

    fn call_method_integer(receiver: i32, argument: i32, operation: &str, result: i32) {
        call_method(Object::from_i32(receiver),
                    Object::from_i32(argument),
                    operation,
                    Object::from_i32(result));
    }

    fn call_method_integer_cmp(receiver: i32, argument: i32, operation: &str, result: bool) {
        call_method(Object::from_i32(receiver),
                    Object::from_i32(argument),
                    operation,
                    Object::from_bool(result));
    }

    fn call_method_boolean(receiver: bool, argument: bool, operation: &str, result: bool) {
        call_method(Object::from_bool(receiver),
                    Object::from_bool(argument),
                    operation,
                    Object::from_bool(result));
    }

    #[test] fn call_method_integer_add() {
        call_method_integer(2, 5, "+", 7);
        call_method_integer(2, 5, "add", 7);
    }

    #[test] fn call_method_integer_subtract() {
        call_method_integer(2, 5, "-", -3);
        call_method_integer(2, 5, "sub", -3);
    }

    #[test] fn call_method_integer_multiply() {
        call_method_integer(2, 5, "*", 10);
        call_method_integer(2, 5, "mul", 10);
    }

    #[test] fn call_method_integer_divide() {
        call_method_integer(2, 5, "/", 0);
        call_method_integer(2, 5, "div", 0);
    }

    #[test] fn call_method_integer_module() {
        call_method_integer(2, 5, "%", 2);
        call_method_integer(2, 5, "mod", 2);
    }

    #[test] fn call_method_integer_equality() {
        call_method_integer_cmp(2, 5, "==", false);
        call_method_integer_cmp(5, 5, "==", true);
        call_method_integer_cmp(2, 5, "eq", false);
        call_method_integer_cmp(5, 5, "eq", true);
    }

    #[test] fn call_method_integer_inequality() {
        call_method_integer_cmp(2, 5, "!=", true);
        call_method_integer_cmp(2, 2, "!=", false);
        call_method_integer_cmp(2, 5, "neq", true);
        call_method_integer_cmp(2, 2, "neq", false);
    }

    #[test] fn call_method_integer_less() {
        call_method_integer_cmp(2, 5, "<", true);
        call_method_integer_cmp(7, 5, "<", false);
        call_method_integer_cmp(5, 5, "<", false);
        call_method_integer_cmp(2, 5, "lt", true);
        call_method_integer_cmp(7, 5, "lt", false);
        call_method_integer_cmp(5, 5, "lt", false);
    }

    #[test] fn call_method_integer_less_equal() {
        call_method_integer_cmp(2, 5, "<=", true);
        call_method_integer_cmp(7, 5, "<=", false);
        call_method_integer_cmp(5, 5, "<=", true);
        call_method_integer_cmp(2, 5, "le", true);
        call_method_integer_cmp(7, 5, "le", false);
        call_method_integer_cmp(5, 5, "le", true);
    }

    #[test] fn call_method_integer_more() {
        call_method_integer_cmp(2, 5, ">", false);
        call_method_integer_cmp(7, 5, ">", true);
        call_method_integer_cmp(5, 5, ">", false);
        call_method_integer_cmp(2, 5, "gt", false);
        call_method_integer_cmp(7, 5, "gt", true);
        call_method_integer_cmp(5, 5, "gt", false);
    }

    #[test] fn call_method_integer_more_equal() {
        call_method_integer_cmp(2, 5, ">=", false);
        call_method_integer_cmp(7, 5, ">=", true);
        call_method_integer_cmp(5, 5, ">=", true);
        call_method_integer_cmp(2, 5, "ge", false);
        call_method_integer_cmp(7, 5, "ge", true);
        call_method_integer_cmp(5, 5, "ge", true);
    }

    #[test] fn call_method_boolean_conjunction() {
        call_method_boolean(true, false, "&",   false);
        call_method_boolean(true, true,  "&",   true);
        call_method_boolean(true, false, "and", false);
        call_method_boolean(true, true,  "and", true);
    }

    #[test] fn call_method_boolean_disjunction() {
        call_method_boolean(true,  false, "|",  true);
        call_method_boolean(false, false, "|",  false);
        call_method_boolean(true,  false, "or", true);
        call_method_boolean(false, false, "or", false);
    }

    #[test] fn call_method_boolean_equal() {
        call_method_boolean(true,  false, "==",  false);
        call_method_boolean(false, false, "==",  true);
        call_method_boolean(true,  true,  "==",  true);
        call_method_boolean(true,  false, "eq",  false);
        call_method_boolean(false, false, "eq",  true);
        call_method_boolean(true,  true,  "eq",  true);
    }

    #[test] fn call_method_boolean_unequal() {
        call_method_boolean(true,  false, "!=",  true);
        call_method_boolean(false, false, "!=",  false);
        call_method_boolean(true,  true,  "!=",  false);
        call_method_boolean(true,  false, "neq",  true);
        call_method_boolean(false, false, "neq",  false);
        call_method_boolean(true,  true,  "neq",  false);
    }

    #[test] fn call_method_array_get() {
        let code = Code::from(vec!(
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(1 + 1) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::from_str("get"));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        state.set_instruction_pointer(Some(Address::from_usize(0)));
        state.allocate(Object::from_i32(1));
        state.allocate(Object::from_i32(2));
        state.allocate(Object::from_i32(3));
        state.allocate_and_push_operand(Object::from_pointers(vec!(Pointer::from(0),
                                                                   Pointer::from(1),
                                                                   Pointer::from(2))));
        state.allocate_and_push_operand(Object::from_i32(1));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(1)), "test operands");
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(1),
                                                   Object::from_i32(2),
                                                   Object::from_i32(3),
                                                   Object::from_pointers(vec!(Pointer::from(0),
                                                                              Pointer::from(1),
                                                                              Pointer::from(2))),
                                                   Object::from_i32(1))), "test memory")
    }

    // before: array(1,2,3)
    //         a.set(1, 42)
    // after:  array(1,42,3)
    #[test] fn call_method_array_set() {
        let code = Code::from(vec!(
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(3) },
            OpCode::Skip,
        ));

        let constants: Vec<ProgramObject> = vec!(ProgramObject::String("set".to_string()));
        let globals: Vec<ConstantPoolIndex> = vec!();
        let entry = ConstantPoolIndex::new(0);
        let program = Program::new(code, constants, globals, entry);

        let mut state = State::minimal();
        let mut output: String = String::new();

        let array = Object::from_pointers(vec!(Pointer::from(0),
                                               Pointer::from(1),
                                               Pointer::from(2)));

        state.set_instruction_pointer(Some(Address::from_usize(0)));
        state.allocate(Object::from_i32(1));
        state.allocate(Object::from_i32(2));
        state.allocate(Object::from_i32(3));
        state.allocate_and_push_operand(array.clone());
        state.allocate_and_push_operand(Object::from_i32(1));
        state.allocate_and_push_operand(Object::from_i32(42));

        interpret(&mut state, &mut output, &program);

        assert_eq!(&output, "", "test output");
        assert_eq!(state.operands, vec!(Pointer::from(6)), "test operands");    // returns null
        assert_eq!(state.globals, HashMap::new(), "test globals");
        assert_eq!(state.instruction_pointer, Some(Address::from_usize(1)), "test instruction pointer");
        assert_eq!(state.frames, vec!(LocalFrame::empty()), "test frames");
        assert_eq!(state.memory, Memory::from(vec!(Object::from_i32(1),
                                                   Object::from_i32(2),
                                                   Object::from_i32(3),
                                                   Object::from_pointers(vec!(Pointer::from(0),
                                                                              Pointer::from(5),
                                                                              Pointer::from(2))),
                                                   Object::from_i32(1),
                                                   Object::from_i32(42),
                                                   Object::Null)), "test memory");

        assert_eq!(array, Object::from_pointers(vec!(Pointer::from(0),
                                                     Pointer::from(1),
                                                     Pointer::from(2))), "test object state");
    }

    #[test] fn call_method_null_equals() {
        call_method(Object::Null, Object::Null, "==", Object::from_bool(true));
        call_method(Object::Null, Object::from_i32(1), "==", Object::from_bool(false));
        call_method(Object::from_i32(1), Object::Null, "==", Object::from_bool(false));

        call_method(Object::Null, Object::Null, "eq", Object::from_bool(true));
        call_method(Object::Null, Object::from_i32(1), "eq", Object::from_bool(false));
        call_method(Object::from_i32(1), Object::Null, "eq", Object::from_bool(false));
    }

    #[test] fn call_method_null_unequals() {
        call_method(Object::Null, Object::Null, "!=", Object::from_bool(false));
        call_method(Object::Null, Object::from_i32(1), "!=", Object::from_bool(true));
        call_method(Object::from_i32(1), Object::Null, "!=", Object::from_bool(true));

        call_method(Object::Null, Object::Null, "neq", Object::from_bool(false));
        call_method(Object::Null, Object::from_i32(1), "neq", Object::from_bool(true));
        call_method(Object::from_i32(1), Object::Null, "neq", Object::from_bool(true));
    }
}

//CallMethod   { name: _,     arguments: _ } => 0x07,

#[cfg(test)]
mod hello_world_tests {
    use crate::program::{Code, Program};
    use crate::objects::ProgramObject;
    use crate::types::{ConstantPoolIndex, Arity, Size, AddressRange};
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::debug::PrettyPrint;
    use std::io::Cursor;
    use crate::interpreter::{interpret, State};

    fn source() -> &'static str {
        r#"Constants :
    #0: String("Hello World\n")
    #1: String("main")
    #2: Method(#1, nargs:0, nlocals:0) :
          printf #0 0
          return
    #3: Null
    #4: String("entry35")
    #5: Method(#4, nargs:0, nlocals:0) :
          call #1 0
          drop
          lit #3
          return
Globals :
    #2
Entry : #5"#}

    fn bytes() -> Vec<u8> {
        vec!(
            0x06, 0x00, 0x02, 0x0C, 0x00, 0x00, 0x00, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57,
            0x6F, 0x72, 0x6C, 0x64, 0x0A, 0x02, 0x04, 0x00, 0x00, 0x00, 0x6D, 0x61, 0x69, 0x6E,
            0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
            0x0F, 0x01, 0x02, 0x07, 0x00, 0x00, 0x00, 0x65, 0x6E, 0x74, 0x72, 0x79, 0x33, 0x35,
            0x03, 0x04, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x08, 0x01, 0x00, 0x00,
            0x10, 0x01, 0x03, 0x00, 0x0F, 0x01, 0x00, 0x02, 0x00, 0x05, 0x00,
        )
    }

    fn program() -> Program {
        let code = Code::from(vec!(
            /* 0 */ OpCode::Print { format: ConstantPoolIndex::new(0), arguments: Arity::new(0) },
            /* 1 */ OpCode::Return,
            /* 2 */ OpCode::CallFunction { name: ConstantPoolIndex::new(1), arguments: Arity::new(0) },
            /* 3 */ OpCode::Drop,
            /* 4 */ OpCode::Literal { index: ConstantPoolIndex::new(3) },
            /* 5 */ OpCode::Return,
        ));

        let constants = vec!(
            /* #0 */ ProgramObject::String("Hello World\n".to_string()),
            /* #1 */ ProgramObject::String("main".to_string()),
            /* #2 */ ProgramObject::Method {
                name: ConstantPoolIndex::new(1),
                arguments: Arity::new(0),
                locals: Size::new(0),
                code: AddressRange::from(0, 2),
            },
            /* #3 */ ProgramObject::Null,
            /* #4 */ ProgramObject::String("entry35".to_string()),
            /* #5 */ ProgramObject::Method {
                name: ConstantPoolIndex::new(4),
                arguments: Arity::new(0),
                locals: Size::new(0),
                code: AddressRange::from(2, 4),
            },
        );

        let globals = vec!(ConstantPoolIndex::new(2));
        let entry = ConstantPoolIndex::new(5);

        Program::new(code, constants, globals, entry)
    }

    #[test] fn deserialize() {
        let object = Program::from_bytes(&mut Cursor::new(bytes()));
        assert_eq!(program(), object);
    }

    #[test] fn serialize() {
        let mut output: Vec<u8> = Vec::new();
        program().serialize(&mut output);
        assert_eq!(bytes(), output);
    }

    #[test] fn print() {
        let mut bytes: Vec<u8> = Vec::new();
        program().pretty_print(&mut bytes);
        assert_eq!(&String::from_utf8(bytes).unwrap(), source());
    }

    #[test] fn eval() {
        let program = program();
        let mut state = State::from(&program);
        let mut output = String::new();

        loop {
            interpret(&mut state, &mut output, &program);
            if let None = state.instruction_pointer() {
                break;
            }
        }

        assert_eq!(output, "Hello World\n");
    }
}

#[cfg(test)]
mod fibonacci_tests {
    use crate::program::{Code, Program};
    use crate::objects::ProgramObject;
    use crate::types::{ConstantPoolIndex, Arity, Size, AddressRange, LocalFrameIndex};
    use crate::bytecode::OpCode;
    use crate::serializable::Serializable;
    use crate::debug::PrettyPrint;
    use std::io::Cursor;
    use crate::interpreter::{State, interpret};

    fn source() -> &'static str {
        r#"Constants :
    #0: String("conseq39")
    #1: String("end40")
    #2: Int(0)
    #3: String("eq")
    #4: String("conseq41")
    #5: String("end42")
    #6: Int(1)
    #7: String("test43")
    #8: String("loop44")
    #9: String("add")
    #10: String("sub")
    #11: Int(2)
    #12: String("ge")
    #13: Null
    #14: String("fib")
    #15: Method(#14, nargs:1, nlocals:3) :
          get local 0
          lit #2
          call slot #3 2
          branch #0
          get local 0
          lit #6
          call slot #3 2
          branch #4
          lit #6
          set local 1
          drop
          lit #6
          set local 2
          drop
          goto #7
       label #8
          get local 1
          get local 2
          call slot #9 2
          set local 3
          drop
          get local 2
          set local 1
          drop
          get local 3
          set local 2
          drop
          get local 0
          lit #6
          call slot #10 2
          set local 0
          drop
       label #7
          get local 0
          lit #11
          call slot #12 2
          branch #8
          lit #13
          drop
          get local 2
          goto #5
       label #4
          lit #6
       label #5
          goto #1
       label #0
          lit #6
       label #1
          return
    #16: String("test45")
    #17: String("loop46")
    #18: String("Fib(~) = ~\n")
    #19: Int(20)
    #20: String("lt")
    #21: String("main")
    #22: Method(#21, nargs:0, nlocals:1) :
          lit #2
          set local 0
          drop
          goto #16
       label #17
          get local 0
          get local 0
          call #14 1
          printf #18 2
          drop
          get local 0
          lit #6
          call slot #9 2
          set local 0
          drop
       label #16
          get local 0
          lit #19
          call slot #20 2
          branch #17
          lit #13
          return
    #23: String("entry47")
    #24: Method(#23, nargs:0, nlocals:0) :
          call #21 0
          drop
          lit #13
          return
Globals :
    #15
    #22
Entry : #24"#}

    fn expected_output() -> &'static str {
        r#"Fib(0) = 1
Fib(1) = 1
Fib(2) = 2
Fib(3) = 3
Fib(4) = 5
Fib(5) = 8
Fib(6) = 13
Fib(7) = 21
Fib(8) = 34
Fib(9) = 55
Fib(10) = 89
Fib(11) = 144
Fib(12) = 233
Fib(13) = 377
Fib(14) = 610
Fib(15) = 987
Fib(16) = 1597
Fib(17) = 2584
Fib(18) = 4181
Fib(19) = 6765
"#
    }

    fn bytes() -> Vec<u8> {
        vec!(
            0x19, 0x00, 0x02, 0x08, 0x00, 0x00, 0x00, 0x63, 0x6F, 0x6E, 0x73, 0x65, 0x71, 0x33,
            0x39, 0x02, 0x05, 0x00, 0x00, 0x00, 0x65, 0x6E, 0x64, 0x34, 0x30, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x00, 0x65, 0x71, 0x02, 0x08, 0x00, 0x00, 0x00,
            0x63, 0x6F, 0x6E, 0x73, 0x65, 0x71, 0x34, 0x31, 0x02, 0x05, 0x00, 0x00, 0x00, 0x65,
            0x6E, 0x64, 0x34, 0x32, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x06, 0x00, 0x00, 0x00,
            0x74, 0x65, 0x73, 0x74, 0x34, 0x33, 0x02, 0x06, 0x00, 0x00, 0x00, 0x6C, 0x6F, 0x6F,
            0x70, 0x34, 0x34, 0x02, 0x03, 0x00, 0x00, 0x00, 0x61, 0x64, 0x64, 0x02, 0x03, 0x00,
            0x00, 0x00, 0x73, 0x75, 0x62, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00,
            0x00, 0x67, 0x65, 0x01, 0x02, 0x03, 0x00, 0x00, 0x00, 0x66, 0x69, 0x62, 0x03, 0x0E,
            0x00, 0x01, 0x03, 0x00, 0x31, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x01, 0x02, 0x00,
            0x07, 0x03, 0x00, 0x02, 0x0D, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x01, 0x06, 0x00, 0x07,
            0x03, 0x00, 0x02, 0x0D, 0x04, 0x00, 0x01, 0x06, 0x00, 0x09, 0x01, 0x00, 0x10, 0x01,
            0x06, 0x00, 0x09, 0x02, 0x00, 0x10, 0x0E, 0x07, 0x00, 0x00, 0x08, 0x00, 0x0A, 0x01,
            0x00, 0x0A, 0x02, 0x00, 0x07, 0x09, 0x00, 0x02, 0x09, 0x03, 0x00, 0x10, 0x0A, 0x02,
            0x00, 0x09, 0x01, 0x00, 0x10, 0x0A, 0x03, 0x00, 0x09, 0x02, 0x00, 0x10, 0x0A, 0x00,
            0x00, 0x01, 0x06, 0x00, 0x07, 0x0A, 0x00, 0x02, 0x09, 0x00, 0x00, 0x10, 0x00, 0x07,
            0x00, 0x0A, 0x00, 0x00, 0x01, 0x0B, 0x00, 0x07, 0x0C, 0x00, 0x02, 0x0D, 0x08, 0x00,
            0x01, 0x0D, 0x00, 0x10, 0x0A, 0x02, 0x00, 0x0E, 0x05, 0x00, 0x00, 0x04, 0x00, 0x01,
            0x06, 0x00, 0x00, 0x05, 0x00, 0x0E, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0x06, 0x00,
            0x00, 0x01, 0x00, 0x0F, 0x02, 0x06, 0x00, 0x00, 0x00, 0x74, 0x65, 0x73, 0x74, 0x34,
            0x35, 0x02, 0x06, 0x00, 0x00, 0x00, 0x6C, 0x6F, 0x6F, 0x70, 0x34, 0x36, 0x02, 0x0B,
            0x00, 0x00, 0x00, 0x46, 0x69, 0x62, 0x28, 0x7E, 0x29, 0x20, 0x3D, 0x20, 0x7E, 0x0A,
            0x00, 0x14, 0x00, 0x00, 0x00, 0x02, 0x02, 0x00, 0x00, 0x00, 0x6C, 0x74, 0x02, 0x04,
            0x00, 0x00, 0x00, 0x6D, 0x61, 0x69, 0x6E, 0x03, 0x15, 0x00, 0x00, 0x01, 0x00, 0x16,
            0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x09, 0x00, 0x00, 0x10, 0x0E, 0x10, 0x00, 0x00,
            0x11, 0x00, 0x0A, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x08, 0x0E, 0x00, 0x01, 0x02, 0x12,
            0x00, 0x02, 0x10, 0x0A, 0x00, 0x00, 0x01, 0x06, 0x00, 0x07, 0x09, 0x00, 0x02, 0x09,
            0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x0A, 0x00, 0x00, 0x01, 0x13, 0x00, 0x07, 0x14,
            0x00, 0x02, 0x0D, 0x11, 0x00, 0x01, 0x0D, 0x00, 0x0F, 0x02, 0x07, 0x00, 0x00, 0x00,
            0x65, 0x6E, 0x74, 0x72, 0x79, 0x34, 0x37, 0x03, 0x17, 0x00, 0x00, 0x00, 0x00, 0x04,
            0x00, 0x00, 0x00, 0x08, 0x15, 0x00, 0x00, 0x10, 0x01, 0x0D, 0x00, 0x0F, 0x02, 0x00,
            0x0F, 0x00, 0x16, 0x00, 0x18, 0x00,
        )
    }

    fn program () -> Program {
        let code = Code::from(vec!(
            /* method fib: start: 0, length: 39 */
            /* 00 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // arg0
            /* 01 */ OpCode::Literal { index: ConstantPoolIndex::new(2) },  // 0
            /* 02 */ OpCode::CallMethod {                                   // 0.eq(arg0)
                        name: ConstantPoolIndex::new(3),
                        arguments: Arity::new(2) },
            /* 03 */ OpCode::Branch { label: ConstantPoolIndex::new(0) },   // branch conseq39
            /* 04 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // also x
            /* 05 */ OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1
            /* 06 */ OpCode::CallMethod {                                   // arg0.eq(1)
                        name: ConstantPoolIndex::new(3),
                        arguments: Arity::new(2) },
            /* 07 */ OpCode::Branch { label: ConstantPoolIndex::new(4) },   // branch conseq41
            /* 08 */ OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1
            /* 09 */ OpCode::SetLocal { index: LocalFrameIndex::new(1) },   // var1 = 1
            /* 10 */ OpCode::Drop,
            /* 11 */ OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1
            /* 12 */ OpCode::SetLocal { index: LocalFrameIndex::new(2) },   // var2 = 1
            /* 13 */ OpCode::Drop,
            /* 14 */ OpCode::Jump { label: ConstantPoolIndex::new(7) },     // goto test43

            /* 15 */ OpCode::Label { name: ConstantPoolIndex::new(8) },     // label loop44
            /* 16 */ OpCode::GetLocal { index: LocalFrameIndex::new(1) },   // var1
            /* 17 */ OpCode::GetLocal { index: LocalFrameIndex::new(2) },   // var2
            /* 18 */ OpCode::CallMethod {                                   // var1.add(var2) -> result1
                        name: ConstantPoolIndex::new(9),
                        arguments: Arity::new(2) },
            /* 19 */ OpCode::SetLocal { index: LocalFrameIndex::new(3) },   // var3 = result1
            /* 20 */ OpCode::Drop,
            /* 21 */ OpCode::GetLocal { index: LocalFrameIndex::new(2) },   // var2
            /* 22 */ OpCode::SetLocal { index: LocalFrameIndex::new(1) },   // var1 = var2
            /* 23 */ OpCode::Drop,
            /* 24 */ OpCode::GetLocal { index: LocalFrameIndex::new(3) },   // var3
            /* 25 */ OpCode::SetLocal { index: LocalFrameIndex::new(2) },   // var2 = var3
            /* 26 */ OpCode::Drop,
            /* 27 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // arg0
            /* 28 */ OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1
            /* 29 */ OpCode::CallMethod {                                   // arg0.sub(1) -> result2
                        name: ConstantPoolIndex::new(10),
                        arguments: Arity::new(2) },
            /* 30 */ OpCode::SetLocal { index: LocalFrameIndex::new(0) },   // arg0 = result2
            /* 31 */ OpCode::Drop,
            /* 32 */ OpCode::Label { name: ConstantPoolIndex::new(7) },     // label test43
            /* 33 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // arg0
            /* 34 */ OpCode::Literal { index: ConstantPoolIndex::new(11) }, // 2
            /* 35 */ OpCode::CallMethod {                                   // arg0.ge(2) -> result3
                        name: ConstantPoolIndex::new(12),
                        arguments: Arity::new(2) },
            /* 36 */ OpCode::Branch { label: ConstantPoolIndex::new(8) },   // loop44
            /* 37 */ OpCode::Literal { index: ConstantPoolIndex::new(13) }, // null
            /* 38 */ OpCode::Drop,
            /* 39 */ OpCode::GetLocal { index: LocalFrameIndex::new(2) },   // arg2 (return arg2)
            /* 40 */ OpCode::Jump { label: ConstantPoolIndex::new(5) },     // goto end42
            /* 41 */ OpCode::Label { name: ConstantPoolIndex::new(4) },     // label conseq41
            /* 42 */ OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1 (return 1)
            /* 43 */ OpCode::Label { name: ConstantPoolIndex::new(5) },     // label end42
            /* 44 */ OpCode::Jump { label: ConstantPoolIndex::new(1) },     // goto end40
            /* 45 */ OpCode::Label { name: ConstantPoolIndex::new(0) },     // label conseq39
            /* 46 */ OpCode::Literal { index: ConstantPoolIndex::new(6) },  // 1 (return 1)
            /* 47 */ OpCode::Label { name: ConstantPoolIndex::new(1) },     // label end40
            /* 48 */ OpCode::Return,

            /* method main: start: 49, length: 22 */
            /* 49 */ OpCode::Literal { index: ConstantPoolIndex::new(2) },  // 0
            /* 50 */ OpCode::SetLocal { index: LocalFrameIndex::new(0) },   // var0 = 0
            /* 51 */ OpCode::Drop,
            /* 52 */ OpCode::Jump { label: ConstantPoolIndex::new(16) },    // goto loop45
            /* 53 */ OpCode::Label { name: ConstantPoolIndex::new(17) },    // label loop46
            /* 54 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // var0
            /* 55 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },   // var0 ... again?
            /* 56 */ OpCode::CallFunction {                                 // fib(var0) -> result1
                        name: ConstantPoolIndex::new(14),
                        arguments: Arity::new(1) },
            /* 57 */ OpCode::Print {                                        // printf "Fib(~) = ~\n" var0 result1
                        format: ConstantPoolIndex::new(18),
                        arguments: Arity::new(2) },
            /* 58 */ OpCode::Drop,
            /* 59 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },    // var0
            /* 60 */ OpCode::Literal { index: ConstantPoolIndex::new(6) },   // 1
            /* 61 */ OpCode::CallMethod {                                    // var0.add(1) -> result2
                        name: ConstantPoolIndex::new(9),
                        arguments: Arity::new(2) },
            /* 62 */ OpCode::SetLocal { index: LocalFrameIndex::new(0) },    // var0 = result2
            /* 63 */ OpCode::Drop,
            /* 64 */ OpCode::Label { name: ConstantPoolIndex::new(16) },     // label test45
            /* 65 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },    // var0
            /* 66 */ OpCode::Literal { index: ConstantPoolIndex::new(19) },  // 20
            /* 67 */ OpCode::CallMethod {                                    // var0.lt(20) -> result3
                        name: ConstantPoolIndex::new(20),
                        arguments: Arity::new(2) },
            /* 68 */ OpCode::Branch { label: ConstantPoolIndex::new(17) },   // branch loop46
            /* 69 */ OpCode::Literal { index: ConstantPoolIndex::new(13) },  // null
            /* 70 */ OpCode::Return,

            /* method entry: start: 71, length: 4 */
            /* 71 */ OpCode::CallFunction {                                 // main() -> result0
                        name: ConstantPoolIndex::new(21),
                        arguments: Arity::new(0) },
            /* 72 */ OpCode::Drop,
            /* 73 */ OpCode::Literal { index: ConstantPoolIndex::new(13) }, // null
            /* 74 */ OpCode::Return
        ));

        let constants = vec!(
            /* #0  0x00 */ ProgramObject::String("conseq39".to_string()),
            /* #1  0x01 */ ProgramObject::String("end40".to_string()),
            /* #2  0x02 */ ProgramObject::Integer(0),
            /* #3  0x03 */ ProgramObject::String("eq".to_string()),
            /* #4  0x04 */ ProgramObject::String("conseq41".to_string()),
            /* #5  0x05 */ ProgramObject::String("end42".to_string()),
            /* #6  0x06 */ ProgramObject::Integer(1),
            /* #7  0x07 */ ProgramObject::String("test43".to_string()),
            /* #8  0x08 */ ProgramObject::String("loop44".to_string()),
            /* #9  0x09 */ ProgramObject::String("add".to_string()),
            /* #10 0x0A */ ProgramObject::String("sub".to_string()),
            /* #11 0x0B */ ProgramObject::Integer(2),
            /* #12 0x0C */ ProgramObject::String("ge".to_string()),
            /* #13 0x0D */ ProgramObject::Null,
            /* #14 0x0E */ ProgramObject::String("fib".to_string()),
            /* #15 0x0F */ ProgramObject::Method {                             // fib
                name: ConstantPoolIndex::new(14),
                arguments: Arity::new(1),
                locals: Size::new(3),
                code: AddressRange::from(0, 49),
            },
            /* #16 0x10 */ ProgramObject::String("test45".to_string()),
            /* #17 0x11 */ ProgramObject::String("loop46".to_string()),
            /* #18 0x11 */ ProgramObject::String("Fib(~) = ~\n".to_string()),
            /* #19 0x12 */ ProgramObject::Integer(20),
            /* #20 0x13 */ ProgramObject::String("lt".to_string()),
            /* #21 0x14 */ ProgramObject::String("main".to_string()),
            /* #22 0x15 */ ProgramObject::Method {                             // main
                name: ConstantPoolIndex::new(21),
                arguments: Arity::new(0),
                locals: Size::new(1),
                code: AddressRange::from(49, 22),
            },
            /* #23 0x15 */ ProgramObject::String("entry47".to_string()),
            /* #24 0x16 */ ProgramObject::Method {                             // entry47
                name: ConstantPoolIndex::new(23),
                arguments: Arity::new(0),
                locals: Size::new(0),
                code: AddressRange::from(71,4),
            }
        );

        let globals = vec!(
            ConstantPoolIndex::new(15),
            ConstantPoolIndex::new(22)
        );

        let entry = ConstantPoolIndex::new(24);

        Program::new (code, constants, globals, entry)
    }

    #[test] fn deserialize() {
        let object = Program::from_bytes(&mut Cursor::new(bytes()));
        assert_eq!(program(), object);
    }

    #[test] fn serialize() {
        let mut output: Vec<u8> = Vec::new();
        program().serialize(&mut output);
        assert_eq!(bytes(), output);
    }

    #[test] fn print() {
        let mut bytes: Vec<u8> = Vec::new();
        program().pretty_print(&mut bytes);
        assert_eq!(&String::from_utf8(bytes).unwrap(), source());
    }

    #[test] fn eval() {
        let program = program();
        let mut state = State::from(&program);
        let mut output = String::new();


        let mut source:Vec<u8> = Vec::new();
        program.pretty_print(&mut source);
        println!("{}", String::from_utf8(source).unwrap());

        program.code().dump();

        loop {
            match state.instruction_pointer() {
                Some(address) => println!("{:?} => {:?}", address, program.get_opcode(address)),
                _ => println!("None => ..."),
            }
            println!("stack before: {:?}", state.operands);
            println!("frame before: {:?}", state.frames.last());
            interpret(&mut state, &mut output, &program);
            if let None = state.instruction_pointer() {
                break;
            }
            println!("stack after:  {:?}", state.operands);
            println!("frame after:  {:?}", state.frames.last());
            println!();
        }

        assert_eq!(output, expected_output());
    }
}

#[cfg(test)]
mod compiler_tests {
    use fml_ast::{AST, Identifier, Operator};
    use crate::compiler::Compiled;
    use crate::program::{Program, Code};
    use crate::bytecode::OpCode;
    use crate::compiler::Bookkeeping;
    use crate::objects::ProgramObject;
    use crate::types::{ConstantPoolIndex, LocalFrameIndex, Arity, Size, AddressRange};
    use fml_ast::AST::{Boolean, VariableAccess};

    #[test] fn number () {
        let ast = AST::Number(1);

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(0) }
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Integer(1)
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn some_more_numbers () {
        let asts = vec!(AST::Number(1), AST::Number(42), AST::Number(0), AST::Number(42));

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        for ast in asts {
            ast.compile_into(&mut program, &mut bookkeeping);
        }

        let expected_bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(0) },
            /* 1 */ OpCode::Literal { index: ConstantPoolIndex::new(1) },
            /* 2 */ OpCode::Literal { index: ConstantPoolIndex::new(2) },
            /* 3 */ OpCode::Literal { index: ConstantPoolIndex::new(1) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Integer(1),
            /* 1 */ ProgramObject::Integer(42),
            /* 2 */ ProgramObject::Integer(0),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn boolean () {
        let ast = AST::Boolean(true);

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(0) }
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Boolean(true)
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn unit () {
        let ast = AST::Unit;

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(0) }
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Null
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn local_definition () {
        let ast = AST::VariableDefinition { name: Identifier::from("x"),
                                         value: Box::new(AST::Number(1)) };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!("x".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::Literal { index: ConstantPoolIndex::new(0) },    // value
            OpCode::SetLocal { index: LocalFrameIndex::new(0) }
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Integer(1)
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn global_definition () {
        let ast = AST::VariableDefinition { name: Identifier::from("x"),
            value: Box::new(AST::Number(1)) };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::without_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_globals(vec!("x".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::Literal { index: ConstantPoolIndex::new(1) },    // value
            OpCode::SetGlobal { name: ConstantPoolIndex::new(0) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_str("x"),
            /* 1 */ ProgramObject::from_i32(1),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn local_access_x () {
        let ast = AST::VariableAccess { name: Identifier::from("x") };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("x".to_string(), "y".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!("x".to_string(), "y".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::GetLocal { index: LocalFrameIndex::new(0) }
        ));

        let expected_constants: Vec<ProgramObject> = vec!();

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn local_access_y () {
        let ast = AST::VariableAccess { name: Identifier::from("y") };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping =
            Bookkeeping::from_locals(vec!("x".to_string(), "y".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping =
            Bookkeeping::from_locals(vec!("x".to_string(), "y".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::GetLocal { index: LocalFrameIndex::new(1) }
        ));

        let expected_constants: Vec<ProgramObject> = vec!();

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn global_access () {
        let ast = AST::VariableAccess { name: Identifier::from("x") };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping =
            Bookkeeping::from_globals(vec!("x".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping =
            Bookkeeping::from_globals(vec!("x".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::GetGlobal { name: ConstantPoolIndex::new(0) }
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            ProgramObject::from_str("x")
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn global_access_from_elsewhere () {
        let ast = AST::VariableAccess { name: Identifier::from("z") };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping =
            Bookkeeping::from(vec!("x".to_string()), vec!("z".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping =
            Bookkeeping::from(vec!("x".to_string()), vec!("z".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::GetGlobal { name: ConstantPoolIndex::new(0) }
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_str("z"),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn loop_de_loop () {
        let ast = AST::Loop { condition: Box::new(AST::Boolean(false)), body: Box::new(AST::Unit) };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Jump { label: ConstantPoolIndex::new(1) },
            /* 1 */ OpCode::Label { name: ConstantPoolIndex::new(0) },
            /* 2 */ OpCode::Literal { index: ConstantPoolIndex::new(2) },
            /* 3 */ OpCode::Label { name: ConstantPoolIndex::new(1) },
            /* 4 */ OpCode::Literal { index: ConstantPoolIndex::new(3) },
            /* 5 */ OpCode::Branch { label: ConstantPoolIndex::new(0) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::String("loop_body_0".to_string()),
            /* 1 */ ProgramObject::String("loop_condition_0".to_string()),
            /* 2 */ ProgramObject::Null,
            /* 3 */ ProgramObject::Boolean(false),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn conditional () {
        let ast = AST::Conditional {
            condition: Box::new(AST::Boolean(true)),
            consequent: Box::new(AST::Number(1)),
            alternative: Box::new(AST::Number(-1))
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(2) },
            /* 1 */ OpCode::Branch { label: ConstantPoolIndex::new(0) },
            /* 2 */ OpCode::Literal { index: ConstantPoolIndex::new(3) },
            /* 3 */ OpCode::Jump { label: ConstantPoolIndex::new(1) },
            /* 4 */ OpCode::Label { name: ConstantPoolIndex::new(0) },
            /* 5 */ OpCode::Literal { index: ConstantPoolIndex::new(4) },
            /* 6 */ OpCode::Label { name: ConstantPoolIndex::new(1) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::String("if_consequent_0".to_string()),
            /* 1 */ ProgramObject::String("if_end_0".to_string()),
            /* 2 */ ProgramObject::Boolean(true),
            /* 3 */ ProgramObject::Integer(-1),
            /* 4 */ ProgramObject::Integer(1),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn array_definition_simple_test() {
        let ast = AST::ArrayDefinition {
            value: Box::new(AST::Unit),
            size: Box::new(AST::Number(10)),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(0) },
            /* 1 */ OpCode::Literal { index: ConstantPoolIndex::new(1) },
            /* 2 */ OpCode::Array,
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Integer(10),
            /* 1 */ ProgramObject::Null,
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn array_definition_complex_test() {
        let ast = AST::ArrayDefinition {
            size: Box::new(AST::Number(10)),
            value: Box::new(AST::FunctionCall {
                function: Identifier::from("f"),
                arguments: vec!()
            }),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!(
            "?size_0".to_string(),
            "?array_1".to_string(),
            "?i_2".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::Literal { index: ConstantPoolIndex::new(2) },   // size
            OpCode::SetLocal { index: LocalFrameIndex::new(0) },    // ?size
            OpCode::Literal { index: ConstantPoolIndex::new(3) },   // null
            OpCode::Array,                                          // array(size, null)
            OpCode::SetLocal { index: LocalFrameIndex::new(1) },    // ?array
            OpCode::Literal { index: ConstantPoolIndex::new(4) },   // 0
            OpCode::SetLocal { index: LocalFrameIndex::new(2) },    // ?i
            OpCode::Label { name: ConstantPoolIndex::new(0) },      // label start
            OpCode::GetLocal { index: LocalFrameIndex::new(0) },    // ?size
            OpCode::CallMethod { name: ConstantPoolIndex::new(5),
                                 arguments: Arity::new(2) },        // ?i.ge(?size)
            OpCode::Branch { label: ConstantPoolIndex::new(1) },    // if true goto end
            OpCode::GetLocal { index: LocalFrameIndex::new(2) },    // ?i
            OpCode::CallFunction { name: ConstantPoolIndex::new(6),
                                   arguments: Arity::new(0) },      // value
            OpCode::CallMethod { name: ConstantPoolIndex::new(7),
                                 arguments: Arity::new(3) },        // ?array[?i] = value
            OpCode::Drop,
            OpCode::Literal { index: ConstantPoolIndex::new(8) },   // 1
            OpCode::GetLocal { index: LocalFrameIndex::new(2) },    // ?i
            OpCode::CallMethod { name: ConstantPoolIndex::new(9),
                                 arguments: Arity::new(2) },        // ?i + 1
            OpCode::SetLocal { index: LocalFrameIndex::new(2) },    // ?i = ?i + 1
            OpCode::Drop,
            OpCode::GetLocal { index: LocalFrameIndex::new(1) },    // ?array
            OpCode::Jump { label: ConstantPoolIndex::new(0) },      // goto start
            OpCode::Label { name: ConstantPoolIndex::new(1) },      // label end
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_str("array_init_start_0"),
            /* 1 */ ProgramObject::from_str("array_init_end_0"),
            /* 2 */ ProgramObject::from_i32(10),
            /* 3 */ ProgramObject::Null,
            /* 4 */ ProgramObject::from_i32(0),
            /* 5 */ ProgramObject::from_str("ge"),
            /* 6 */ ProgramObject::from_str("f"),
            /* 7 */ ProgramObject::from_str("set"),
            /* 8 */ ProgramObject::from_i32(1),
            /* 9 */ ProgramObject::from_str("add"),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn array_access_test() {
        let ast = AST::ArrayAccess {
            array: Box::new(AST::VariableAccess { name: Identifier("x".to_string()) }),
            index: Box::new(AST::Number(1)),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("x".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("x".to_string()));

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },
            /* 1 */ OpCode::Literal { index: ConstantPoolIndex::new(0) },
            /* 2 */ OpCode::CallMethod { name: ConstantPoolIndex::new(1), arguments: Arity::new(2) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Integer(1),
            /* 1 */ ProgramObject::String("get".to_string()),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn array_mutation_test() {
        let ast = AST::ArrayMutation {
            array: Box::new(AST::VariableAccess { name: Identifier("x".to_string()) }),
            index: Box::new(AST::Number(1)),
            value: Box::new(AST::Number(42)),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("x".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("x".to_string()));

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },
            /* 1 */ OpCode::Literal { index: ConstantPoolIndex::new(0) },
            /* 2 */ OpCode::Literal { index: ConstantPoolIndex::new(1) },
            /* 3 */ OpCode::CallMethod { name: ConstantPoolIndex::new(2), arguments: Arity::new(3) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Integer(1),
            /* 1 */ ProgramObject::Integer(42),
            /* 2 */ ProgramObject::String("set".to_string()),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn print_test () {
        let ast = AST::Print {
            format: "~ + ~".to_string(),
            arguments: vec!(
                Box::new(AST::Number(2)),
                Box::new(AST::Number(5)),
            ),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index:  ConstantPoolIndex::new(1) },
            /* 1 */ OpCode::Literal { index:  ConstantPoolIndex::new(2) },
            /* 2 */ OpCode::Print   { format: ConstantPoolIndex::new(0), arguments: Arity::new(2) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::String("~ + ~".to_string()),
            /* 1 */ ProgramObject::Integer(2),
            /* 2 */ ProgramObject::Integer(5),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn function_application_test_three () {
        let ast = AST::FunctionCall {
            function: Identifier("f".to_string()),
            arguments: vec!(
                Box::new(AST::Unit),
                Box::new(AST::Number(0)),
                Box::new(AST::Boolean(true)),
            ),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index:  ConstantPoolIndex::new(1) },
            /* 1 */ OpCode::Literal { index:  ConstantPoolIndex::new(2) },
            /* 2 */ OpCode::Literal { index:  ConstantPoolIndex::new(3) },
            /* 3 */ OpCode::CallFunction { name: ConstantPoolIndex::new(0), arguments: Arity::new(3) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::String("f".to_string()),
            /* 1 */ ProgramObject::Null,
            /* 2 */ ProgramObject::Integer(0),
            /* 3 */ ProgramObject::Boolean(true),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn function_application_test_one () {
        let ast = AST::FunctionCall {
            function: Identifier("f".to_string()),
            arguments: vec!(Box::new(AST::Number(42))),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index:  ConstantPoolIndex::new(1) },
            /* 1 */ OpCode::CallFunction { name: ConstantPoolIndex::new(0), arguments: Arity::new(1) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::String("f".to_string()),
            /* 1 */ ProgramObject::Integer(42),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn function_application_test_zero () {
        let ast = AST::FunctionCall {
            function: Identifier("f".to_string()),
            arguments: vec!()
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::CallFunction { name: ConstantPoolIndex::new(0), arguments: Arity::new(0) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::String("f".to_string()),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn function_definition_three () {
        let ast = AST::FunctionDefinition {
            function: Identifier("project_right".to_string()),
            parameters: vec!(Identifier::from("left"),
                             Identifier::from("middle"),
                             Identifier::from("right")),
            body: Box::new(AST::VariableAccess { name: Identifier::from("left") })
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Jump { label: ConstantPoolIndex::new(0) },
            /* 1 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },
            /* 2 */ OpCode::Return,
            /* 3 */ OpCode::Label { name: ConstantPoolIndex::new(0) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::String("function_guard_0".to_string()),
            /* 1 */ ProgramObject::String("project_right".to_string()),
            /* 2 */ ProgramObject::Method {
                name: ConstantPoolIndex::new(1),
                arguments: Arity::new(3),
                locals: Size::new(0),
                code: AddressRange::from(1, 3),
            },
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!(ConstantPoolIndex::new(2));
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn object_with_methods_and_fields () {
        let ast = AST::ObjectDefinition {
            extends: Some(Box::new(Boolean(true))),
            members: vec!(
                Box::new(AST::FunctionDefinition {
                    function: Identifier::from("implies"),
                    parameters: vec!(Identifier::from("x")),
                    body: Box::new(AST::Boolean(true))}),

                Box::new(AST::VariableDefinition {
                    name: Identifier::from("id"),
                    value: Box::new(AST::Number(1))}),

                Box::new(AST::FunctionDefinition {
                    function: Identifier::from("identity"),
                    parameters: vec!(),
                    body: Box::new(AST::Boolean(true))}),

                Box::new(AST::FunctionDefinition {
                    function: Identifier::from("or"),
                    parameters: vec!(Identifier::from("x")),
                    body: Box::new(AST::Boolean(true))}),

                Box::new(AST::FunctionDefinition {
                    function: Identifier::from("and"),
                    parameters: vec!(Identifier::from("x")),
                    body: Box::new(AST::VariableAccess { name: Identifier::from("x") })}),

                Box::new(AST::VariableDefinition {
                    name: Identifier::from("hash"),
                    value: Box::new(AST::Number(1))}),
            )
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::with_frame();

        let expected_code = Code::from(vec!(
            /*  0 */ OpCode::Jump { label: ConstantPoolIndex::new(0) },      // function_guard_0 - implies
            /*  1 */ OpCode::Literal { index: ConstantPoolIndex::new(1) },   // true
            /*  2 */ OpCode::Return,
            /*  3 */ OpCode::Label { name: ConstantPoolIndex::new(0) },      // function_guard_0

            /*  4 */ OpCode::Literal { index: ConstantPoolIndex::new(4) },   // 1 - slot id

            /*  5 */ OpCode::Jump { label: ConstantPoolIndex::new(7) },      // function_guard_1 - identity
            /*  6 */ OpCode::Literal { index: ConstantPoolIndex::new(1) },   // true
            /*  7 */ OpCode::Return,
            /*  8 */ OpCode::Label { name: ConstantPoolIndex::new(7) },      // function_guard_1

            /*  9 */ OpCode::Jump { label: ConstantPoolIndex::new(10) },     // function_guard_2 - or
            /* 10 */ OpCode::Literal { index: ConstantPoolIndex::new(1) },   // true
            /* 11 */ OpCode::Return,
            /* 12 */ OpCode::Label { name: ConstantPoolIndex::new(10) },     // function_guard_2

            /* 13 */ OpCode::Jump { label: ConstantPoolIndex::new(13) },     // function_guard_3 - or
            /* 14 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },    // x
            /* 15 */ OpCode::Return,
            /* 16 */ OpCode::Label { name: ConstantPoolIndex::new(13) },     // function_guard_3

            /* 17 */ OpCode::Literal { index: ConstantPoolIndex::new(4) },   // 1 - hash

            /* 18 */ OpCode::Literal { index: ConstantPoolIndex::new(1) },   // true - parent
            /* 19 */ OpCode::Object { class: ConstantPoolIndex:: new(18) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 00 */ ProgramObject::from_str("function_guard_0"),
            /* 01 */ ProgramObject::from_bool(true),
            /* 02 */ ProgramObject::from_str("implies"),
            /* 03 */ ProgramObject::Method {
                name: ConstantPoolIndex::new(2),    // implies
                arguments: Arity::new(1),
                locals: Size::new(0),
                code: AddressRange::from(1, 2),     // addresses: 1, 2
            },

            /* 04 */ ProgramObject::from_i32(1),
            /* 05 */ ProgramObject::from_str("id"),
            /* 06 */ ProgramObject::slot_from_u16(5),

            /* 07 */ ProgramObject::from_str("function_guard_1"),
            /* 08 */ ProgramObject::from_str("identity"),
            /* 09 */ ProgramObject::Method {
                name: ConstantPoolIndex::new(8),    // identity
                arguments: Arity::new(0),
                locals: Size::new(0),
                code: AddressRange::from(6, 2),     // addresses: 6, 7
            },

            /* 10 */ ProgramObject::from_str("function_guard_2"),
            /* 11 */ ProgramObject::from_str("or"),
            /* 12 */ ProgramObject::Method {
                name: ConstantPoolIndex::new(11),    // or
                arguments: Arity::new(1),
                locals: Size::new(0),
                code: AddressRange::from(10, 2),     // addresses: 10, 11
            },

            /* 13 */ ProgramObject::from_str("function_guard_3"),
            /* 14 */ ProgramObject::from_str("and"),
            /* 15 */ ProgramObject::Method {
                name: ConstantPoolIndex::new(14),    // and
                arguments: Arity::new(1),
                locals: Size::new(0),
                code: AddressRange::from(14, 2),     // addresses: 14, 15
            },

            /* 16 */ ProgramObject::from_str("hash"),
            /* 17 */ ProgramObject::slot_from_u16(16),
            /* 18 */ ProgramObject::class_from_vec(vec!(3, 6, 9, 12, 15, 17)),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn block_many () {
        let ast = AST::Block(vec!(
            Box::new(AST::Unit),
            Box::new(AST::Number(1)),
            Box::new(AST::Number(42)),
            Box::new(AST::Number(0)),
            Box::new(AST::Boolean(true)),
            Box::new(AST::Number(42))));

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let mut expected_bookkeeping: Bookkeeping = Bookkeeping::with_frame();
        expected_bookkeeping.enter_scope();
        expected_bookkeeping.leave_scope();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(0) },
            /* 1 */ OpCode::Literal { index: ConstantPoolIndex::new(1) },
            /* 3 */ OpCode::Literal { index: ConstantPoolIndex::new(2) },
            /* 4 */ OpCode::Literal { index: ConstantPoolIndex::new(3) },
            /* 5 */ OpCode::Literal { index: ConstantPoolIndex::new(4) },
            /* 6 */ OpCode::Literal { index: ConstantPoolIndex::new(2) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Null,
            /* 1 */ ProgramObject::from_i32(1),
            /* 2 */ ProgramObject::from_i32(42),
            /* 3 */ ProgramObject::from_i32(0),
            /* 4 */ ProgramObject::from_bool(true),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn block_one () {
        let ast = AST::Block(vec!(Box::new(AST::Unit)));

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let mut expected_bookkeeping: Bookkeeping = Bookkeeping::with_frame();
        expected_bookkeeping.enter_scope();
        expected_bookkeeping.leave_scope();

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(0) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::Null,
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn block_zero () {
        let ast = AST::Block(vec!());

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::with_frame();

        ast.compile_into(&mut program, &mut bookkeeping);

        let mut expected_bookkeeping: Bookkeeping = Bookkeeping::with_frame();
        expected_bookkeeping.enter_scope();
        expected_bookkeeping.leave_scope();

        let expected_code = Code::from(vec!());

        let expected_constants: Vec<ProgramObject> = vec!();

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn field_access_test () {
        let ast = AST::FieldAccess {
            object: Box::new(AST::VariableAccess { name: Identifier::from("obj") }),
            field: Identifier::from("x"),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },
            /* 1 */ OpCode::GetSlot { name: ConstantPoolIndex::new(0) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_str("x"),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn field_mutation_test () {
        let ast = AST::FieldMutation {
            object: Box::new(AST::VariableAccess { name: Identifier::from("obj") }),
            field: Identifier::from("x"),
            value: Box::new(AST::Number(42)),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        let expected_code = Code::from(vec!(
            /* 0 */ OpCode::Literal { index: ConstantPoolIndex::new(0) },
            /* 1 */ OpCode::GetLocal { index: LocalFrameIndex::new(0) },
            /* 2 */ OpCode::SetSlot { name: ConstantPoolIndex::new(1) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_i32(42),
            /* 1 */ ProgramObject::from_str("x"),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn method_call_test_three () {
        let ast = AST::MethodCall {
            method: Identifier::from("f"),
            arguments: vec!(Box::new(AST::Number(1)),
                            Box::new(AST::Number(2)),
                            Box::new(AST::Number(3))),
            object: Box::new(VariableAccess { name: Identifier::from("obj") })
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::Literal { index: ConstantPoolIndex::new(1) },
            OpCode::Literal { index: ConstantPoolIndex::new(2) },
            OpCode::Literal { index: ConstantPoolIndex::new(3) },
            OpCode::GetLocal { index: LocalFrameIndex::new(0) },
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(4) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_str("f"),
            /* 1 */ ProgramObject::from_i32(1),
            /* 2 */ ProgramObject::from_i32(2),
            /* 3 */ ProgramObject::from_i32(3),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn method_call_test_one () {
        let ast = AST::MethodCall {
            method: Identifier::from("f"),
            arguments: vec!(Box::new(AST::Number(42))),
            object: Box::new(VariableAccess { name: Identifier::from("obj") })
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::Literal { index: ConstantPoolIndex::new(1) },
            OpCode::GetLocal { index: LocalFrameIndex::new(0) },
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(2) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_str("f"),
            /* 1 */ ProgramObject::from_i32(42),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn method_call_test_zero () {
        let ast = AST::MethodCall {
            method: Identifier::from("f"),
            arguments: vec!(),
            object: Box::new(VariableAccess { name: Identifier::from("obj") })
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!("obj".to_string()));

        let expected_code = Code::from(vec!(
            OpCode::GetLocal { index: LocalFrameIndex::new(0) },
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(1) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_str("f"),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn operator_call_test () {
        let ast = AST::OperatorCall {
            operator: Operator::Subtraction,
            arguments: vec!(Box::new(AST::Number(1))),
            object: Box::new(AST::Number(7)),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!());

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!());

        let expected_code = Code::from(vec!(
            OpCode::Literal { index: ConstantPoolIndex::new(1) },
            OpCode::Literal { index: ConstantPoolIndex::new(2) },
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(2) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_str("-"),
            /* 1 */ ProgramObject::from_i32(1),
            /* 2 */ ProgramObject::from_i32(7),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }

    #[test] fn operation_test () {
        let ast = AST::Operation {
            operator: Operator::Subtraction,
            right: Box::new(AST::Number(1)),
            left: Box::new(AST::Number(7)),
        };

        let mut program: Program = Program::empty();
        let mut bookkeeping: Bookkeeping = Bookkeeping::from_locals(vec!());

        ast.compile_into(&mut program, &mut bookkeeping);

        let expected_bookkeeping = Bookkeeping::from_locals(vec!());

        let expected_code = Code::from(vec!(
            OpCode::Literal { index: ConstantPoolIndex::new(1) },
            OpCode::Literal { index: ConstantPoolIndex::new(2) },
            OpCode::CallMethod { name: ConstantPoolIndex::new(0), arguments: Arity::new(2) },
        ));

        let expected_constants: Vec<ProgramObject> = vec!(
            /* 0 */ ProgramObject::from_str("-"),
            /* 1 */ ProgramObject::from_i32(1),
            /* 2 */ ProgramObject::from_i32(7),
        );

        let expected_globals: Vec<ConstantPoolIndex> = vec!();
        let expected_entry = ConstantPoolIndex::new(0);

        let expected_program =
            Program::new(expected_code, expected_constants, expected_globals, expected_entry);

        assert_eq!(program, expected_program);
        assert_eq!(bookkeeping, expected_bookkeeping);
    }
}

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::stdin;
    use std::io::Read;
    use fml_ast::AST;

    use crate::program::Program;

    println!("{:?}", env::args());

    let files: Vec<String> = env::args().into_iter().map(|e| e.to_string()).collect();

    let input = match files.len() {
        0 => unreachable!(),
        1 => {
            let mut input = String::new();
            stdin().read_to_string(&mut input).expect("Error reading from stdin");
            input
        },
        2 => {
            let path = files.last().unwrap();              // Cannot explode due to conditions above
            let mut file = File::open(path).expect(&format!("Cannot open file: {}", path));
            let mut input = String::new();
            file.read_to_string(&mut input).expect(&format!("Cannot read file: {}", path));
            input
        },
        n => {
            panic!("Can only parse 1 file at a time, but the following files {} were provided: {:?}",
                    n, files)
        },
    };

    println!("{}", input);

    let ast: AST = fml_parser::parse(&input).expect("Parse error");

    println!("{:?}", ast);

    let program: Program = compiler::compile(&ast);

    println!("{:?}", program);

    let mut source:Vec<u8> = Vec::new();
    program.pretty_print(&mut source);
    println!("{}", String::from_utf8(source).unwrap());

    interpreter::evaluate(&program);
}