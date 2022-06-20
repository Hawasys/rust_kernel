pub unsafe fn outbyte(port:u16, val:u8){
    asm!(
        "outb %al, %dx", in("al") val, in("dx") port,
        options(att_syntax, nomem, nostack)
    );
}

pub unsafe fn inbyte(port:u16) -> u8 {
    let val:u8;
    asm!(
        "inb %dx, %al", out("al") val, in("dx") port,
        options(att_syntax, nomem, nostack)
    );
    return val;
}

pub unsafe fn enable_cursor(cursor_start: u8, cursor_end:u8)
{
	outbyte(0x3D4, 0x0A);
	outbyte(0x3D5, (inbyte(0x3D5) & 0xC0) | cursor_start);
 
	outbyte(0x3D4, 0x0B);
	outbyte(0x3D5, (inbyte(0x3D5) & 0xE0) | cursor_end);
}

pub unsafe fn disable_cursor()
{
	outbyte(0x3D4, 0x0A);
	outbyte(0x3D5, 0x20);
}


