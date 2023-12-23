// 2C02 PPU
pub struct ppu {
    pub mem: [u8; 64*1024],
    pub oam: [u8; 256]
}