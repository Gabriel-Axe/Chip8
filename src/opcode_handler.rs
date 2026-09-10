pub enum Instruction
{
    CLEAR_SCREEN_0,
    LOAD_REGISTER_VX_WITH_VALUE_6 { x: u8, value: u8 },
    LOAD_INDEX_REGISTER_WITH_VALUE_A { value: u16 },
    ADD_VALUE_TO_REGISTER_7 { x: u8, value: u8 },
    DRAW_D { x: u8, y: u8, n: u8 },
    UNKNOWN,
}
