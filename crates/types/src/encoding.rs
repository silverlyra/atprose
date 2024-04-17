use fast32::make_base32_alpha as alphabet;
pub use fast32::DecodeError;

#[cfg(feature = "plc")]
alphabet!(BASE32, DECODE_BASE32, b"abcdefghijklmnopqrstuvwxyz234567");
#[cfg(feature = "rkey")]
alphabet!(
    BASE32_SORTABLE,
    DECODE_BASE32_SORTABLE,
    b"234567abcdefghijklmnopqrstuvwxyz"
);

#[cfg(feature = "plc")]
pub fn encode(data: impl AsRef<[u8]>) -> String {
    BASE32.encode(data.as_ref())
}

#[allow(dead_code)]
#[cfg(feature = "plc")]
pub fn decode(data: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
    BASE32.decode_str(data)
}

#[cfg(feature = "plc")]
pub fn decode_into<const N: usize>(
    data: impl AsRef<str>,
    dest: &mut [u8; N],
) -> Result<(), DecodeError> {
    let data = data.as_ref().as_bytes();
    if BASE32.capacity_decode(data) < N {
        return Err(DecodeError::InvalidLength { length: data.len() });
    }

    // fast32 can only decode into a Vec, so we give it a phony one
    let mut dest = unsafe { Vec::from_raw_parts(dest.as_mut_ptr(), 0, N) };
    BASE32.decode_into(data, &mut dest)?;
    std::mem::forget(dest);

    Ok(())
}

#[cfg(feature = "rkey")]
pub fn encode_u64(value: u64) -> String {
    BASE32_SORTABLE.encode_u64(value)
}

#[cfg(feature = "rkey")]
pub fn decode_u64(data: impl AsRef<str>) -> Result<u64, DecodeError> {
    BASE32_SORTABLE.decode_u64_str(data)
}
