
fn int(b: u8) -> u32{
    if b == 0x0{
        return 0;
    }
    else{
        return 1;
    }
    
}

pub fn get_unicode_char(input: &[u8; 8]) -> Option<char>{
    let hex: u32 = 
        int(input[0]) * 0x1 + 
        int(input[1]) * 0x8 +
        int(input[2]) * 0x2 +
        int(input[3]) * 0x10 +
        int(input[4]) * 0x4 +
        int(input[5]) * 0x20 +
        int(input[6]) * 0x40 +
        int(input[7]) * 0x80;

    return char::from_u32(0x2800 + hex);
}