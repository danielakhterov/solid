pub trait Encode {
    fn encode(self) -> Vec<u8>;
    fn required_len(&self) -> u64;
    fn is_dynamic() -> bool;
}

impl<T> Encode for Vec<T>
where T: Encode {
    fn encode(self) -> Vec<u8> {
        let len = self.required_len();

        let mut buf = Vec::with_capacity(len as usize);
        buf[24..32].copy_from_slice(&(self.len() as u64).to_be_bytes());

        let mut offset = self.len() * 32;

        for (index, bytes) in self
            .into_iter()
            .map(Encode::encode)
            .enumerate()
        {
            if T::is_dynamic() {
                buf[index * 32 + 24..(index + 1) + 32].copy_from_slice(&(offset as u64).to_be_bytes());
                buf[offset..offset + bytes.len()].copy_from_slice(&bytes);
                offset += bytes.len()
            } else {
                buf[index * 32..(index + 1) * 32].copy_from_slice(&bytes);
            }
        }

        buf
    }

    fn required_len(&self) -> u64 {
        self.iter().map(Encode::required_len).sum::<u64>() + 32 + if T::is_dynamic() {
            32 * self.len() + 32
        } else {
            32
        } as u64
    }

    fn is_dynamic() -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_test() {
        let buf = "random_bytes".to_string().encode();

        let first_len = hex::decode("000000000000000000000000000000000000000000000000000000000000000c").unwrap();
        let first_data = hex::decode("72616e646f6d5f6279746573000000000000000000000000000000000000000000").unwrap();

        assert_eq!(64, buf.len());
        assert_eq!(&first_len[0..32],    &buf[32 * 0..32 * 1]);
        assert_eq!(&first_data[0..32],   &buf[32 * 1..32 * 2]);
    }
}