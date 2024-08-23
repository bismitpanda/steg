const MARKER: [u8; 32] = [
    0, 1, 1, 1, 0, 0, 1, 1, // "s"
    0, 1, 1, 1, 0, 1, 0, 0, // "t"
    0, 1, 1, 0, 0, 1, 0, 1, // "e"
    0, 1, 1, 0, 0, 1, 1, 1, // "g"
];

fn u8_to_bits(inp: u8) -> [u8; 8] {
    let mut out = [0u8; 8];
    for i in 0..8 {
        out[7 - i] = inp >> i & 1;
    }

    out
}

fn bits_to_u8(bits: &[u8]) -> u8 {
    let mut out = 0;
    out |= bits[0];
    for bit in &bits[1..8] {
        out <<= 1;
        out |= bit;
    }

    out
}

fn u16_to_bits(inp: u16) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[15 - i] = (inp >> i & 1) as u8;
    }

    out
}

fn bits_to_u16(bits: &[u8]) -> u16 {
    let mut out = 0u16;
    out |= bits[0] as u16;
    for &bit in &bits[1..16] {
        out <<= 1;
        out |= bit as u16;
    }

    out
}

pub fn convert_to_bits(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::from(MARKER);
    out.extend_from_slice(&u16_to_bits(bytes.len() as u16));

    for byte in bytes {
        out.extend_from_slice(&u8_to_bits(*byte));
    }

    out
}

pub fn convert_from_bits(bits: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();

    let marker: [u8; 32] = bits[0..32]
        .try_into()
        .expect("Could not convert to marker bits");
    assert!(
        marker == MARKER,
        "Image is not a valid steg. Got marker: [{:#?}]",
        marker
            .chunks_exact(8)
            .map(|chunk| format!("{0}({0:b})", bits_to_u8(chunk)))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let bits = &bits[32..];

    let len = bits_to_u16(
        bits[0..16]
            .try_into()
            .expect("Could not convert to length bits"),
    ) as usize;

    for bits in bits[16..(len + 2) * 8].chunks_exact(8) {
        out.push(bits_to_u8(bits));
    }

    out
}
